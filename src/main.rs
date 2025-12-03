mod cli;
mod core;
mod infra;
mod shared;
mod tui;

use crate::cli::{
    download_single_url, handle_clear_history_command, handle_config_command,
    handle_history_command, handle_playlist_download, show_video_info, CliConfig, Cli, Commands,
    Config,
};
use crate::core::{BatchDownloader, History, PlaylistDownloader};
use crate::infra::{get_clipboard_url, init_logger, level_from_verbosity, ClipboardWatcher,
    LoggerConfig};
use crate::shared::{constants::*, check_ffmpeg, check_ytdlp, validate_youtube_url, Result};
use clap::Parser;
use colored::Colorize;
use std::process;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // Handle --examples flag
    if cli.examples {
        print_examples();
        return Ok(());
    }

    // Initialize logging first
    let log_level = if cli.quiet {
        tracing::Level::ERROR
    } else {
        level_from_verbosity(cli.verbose)
    };

    let logger_config = LoggerConfig::new()
        .with_level(log_level)
        .with_file_logging(cli.log_file)
        .with_json_format(cli.log_json);

    let _guard = init_logger(logger_config).expect("Failed to initialize logger");

    // Log application startup information
    info!("ytdl v{}", env!("CARGO_PKG_VERSION"));
    info!("Operating System: {}", std::env::consts::OS);
    info!("Architecture: {}", std::env::consts::ARCH);
    info!("Working Directory: {:?}", std::env::current_dir().ok());
    info!("Command-line arguments: {:?}", std::env::args().collect::<Vec<_>>());

    // Validate CLI arguments
    if let Err(e) = cli.validate() {
        error!("Validation error: {}", e);
        return Err(shared::YtdlError::Config(e));
    }

    // Check for yt-dlp
    match check_ytdlp() {
        Ok(version) => info!("Found yt-dlp: {}", version),
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    }

    // Load configuration
    let mut config = if let Some(config_path) = &cli.config {
        info!("Loading config from: {:?}", config_path);
        Config::load_from_file(config_path)?
    } else if let Some(default_path) = Config::get_default_config_path() {
        if default_path.exists() {
            info!("Loading config from default location: {:?}", default_path);
            Config::load_from_file(&default_path)?
        } else {
            info!("No config file found, using defaults with environment overrides");
            Config::load_with_env_overrides()
        }
    } else {
        Config::load_with_env_overrides()
    };

    // Merge CLI options into config
    let cli_config = CliConfig {
        output: cli.output.clone(),
        quality: cli.quality.clone(),
        audio_only: cli.audio_only,
        enable_file_logging: cli.log_file,
        enable_json_logging: cli.log_json,
    };
    config.merge_with_cli(cli_config);

    if cli.concurrent > 0 {
        config.concurrent_downloads = Some(cli.concurrent);
    }

    config.log_config();

    let mut history = History::load().unwrap_or_else(|e| {
        warn!("Failed to load history: {}", e);
        History::new()
    });

    // Handle subcommands
    if let Some(command) = cli.command {
        return handle_subcommand(command, &config, &mut history).await;
    }

    // Handle watch mode
    if cli.watch {
        info!("Starting clipboard watch mode");
        let mut watcher = ClipboardWatcher::new()?;

        return watcher
            .watch_loop(|url| {
                info!("New URL detected: {}", url);
                println!("\n{} New URL detected: {}", "📋".green(), url);

                let config_clone = config.clone();
                let mut history_clone = history.clone();

                tokio::spawn(async move {
                    if let Err(e) = download_single_url(&url, &config_clone, &mut history_clone, false).await {
                        error!("Download failed: {}", e);
                        eprintln!("{} {}", "Error:".red().bold(), e);
                    }
                });
            })
            .await;
    }

    // Check for FFmpeg if audio conversion is needed
    if config.audio_only {
        match check_ffmpeg() {
            Ok(version) => info!("Found ffmpeg: {}", version),
            Err(e) => {
                warn!("{}", e);
                warn!("Audio conversion may not work without FFmpeg");
            }
        }
    }

    // Handle interactive mode
    if cli.interactive {
        info!("Starting interactive TUI mode");
        return tui::run_tui().await;
    }

    // Handle batch download
    if let Some(batch_file) = cli.batch {
        info!("Starting batch download from: {:?}", batch_file);

        let mut batch_downloader = BatchDownloader::new(config.clone(), history.clone(), cli.stop_on_error);
        batch_downloader.load_from_file(&batch_file).await?;

        if config.skip_duplicates.unwrap_or(true) {
            batch_downloader.skip_duplicates().await;
        }

        println!("\n{}", "Starting batch download...".green().bold());
        let stats = batch_downloader.download_all().await?;

        println!("\n{}", "Batch Download Complete!".green().bold());
        println!("{}", SEPARATOR_LINE.repeat(SEPARATOR_WIDTH));
        println!("Total: {}", stats.total);
        println!("{} Successful: {}", "✓".green(), stats.successful);
        println!("{} Failed: {}", "✗".red(), stats.failed);
        println!("{} Skipped: {}", "⊘".yellow(), stats.skipped);

        return Ok(());
    }

    // Handle clipboard
    let url = if cli.clipboard {
        match get_clipboard_url() {
            Some(url) => {
                info!("Using URL from clipboard: {}", url);
                println!("{} Using URL from clipboard", "📋".green());
                Some(url)
            }
            None => {
                return Err(shared::YtdlError::Other(
                    "No valid YouTube URL found in clipboard".to_string(),
                ));
            }
        }
    } else {
        cli.url.clone()
    };

    // Handle single URL download
    if let Some(url) = url {
        validate_youtube_url(&url)?;

        // Handle --info flag: show video information without downloading
        if cli.info {
            return show_video_info(&url, &config).await;
        }

        if cli.playlist || PlaylistDownloader::is_playlist_url(&url) {
            info!("Detected playlist URL");
            return handle_playlist_download(&url, &config, cli.range.as_deref(), cli.folder.as_deref()).await;
        }

        return download_single_url(&url, &config, &mut history, cli.resume).await;
    } else {
        Err(shared::YtdlError::Config(
            "No URL provided. Use --help for usage information.".to_string(),
        ))
    }
}

// Moved to commands::info module

// Moved to commands::download module

// Moved to commands::playlist module

async fn handle_subcommand(command: Commands, _config: &Config, history: &mut History) -> Result<()> {
    match command {
        Commands::Config => handle_config_command().await,
        Commands::History { limit, search, export } => {
            handle_history_command(history, limit, search, export).await
        }
        Commands::ClearHistory { older_than } => {
            handle_clear_history_command(history, older_than).await
        }
        Commands::Completions { shell: _ } => {
            warn!("Shell completions not yet implemented");
            Err(crate::shared::YtdlError::Other(
                "Shell completions coming in a future release".to_string(),
            ))
        }
    }
}

fn print_examples() {
    println!("{}", "Common Usage Examples:".green().bold());
    println!("{}", SEPARATOR_LINE.repeat(SEPARATOR_WIDTH));
    println!();

    println!("{}", "Basic Downloads:".cyan().bold());
    println!("  {} ytdl \"https://youtube.com/watch?v=VIDEO_ID\"", "$".yellow());
    println!("    Download a single video in best quality");
    println!();
    println!("  {} ytdl -a \"https://youtube.com/watch?v=VIDEO_ID\"", "$".yellow());
    println!("    Download audio only as MP3");
    println!();
    println!("  {} ytdl -q 720 \"https://youtube.com/watch?v=VIDEO_ID\"", "$".yellow());
    println!("    Download video in 720p quality");
    println!();

    println!("{}", "Interactive Mode:".cyan().bold());
    println!("  {} ytdl -i", "$".yellow());
    println!("    Launch interactive TUI mode with visual interface");
    println!();

    println!("{}", "Playlists:".cyan().bold());
    println!("  {} ytdl -p \"https://youtube.com/playlist?list=PLAYLIST_ID\"", "$".yellow());
    println!("    Download entire playlist");
    println!();
    println!("  {} ytdl --range 1-5 \"https://youtube.com/playlist?list=PLAYLIST_ID\"", "$".yellow());
    println!("    Download videos 1-5 from playlist");
    println!();

    println!("{}", "Batch Downloads:".cyan().bold());
    println!("  {} ytdl -b urls.txt", "$".yellow());
    println!("    Download all URLs from file (one per line)");
    println!();
    println!("  {} ytdl -b urls.txt --concurrent 5", "$".yellow());
    println!("    Download with 5 concurrent downloads");
    println!();

    println!("{}", "Clipboard:".cyan().bold());
    println!("  {} ytdl --clipboard", "$".yellow());
    println!("    Download URL from clipboard");
    println!();
    println!("  {} ytdl --watch", "$".yellow());
    println!("    Watch clipboard and auto-download URLs");
    println!();

    println!("{}", "History:".cyan().bold());
    println!("  {} ytdl history", "$".yellow());
    println!("    Show recent downloads");
    println!();
    println!("  {} ytdl history --search \"keyword\"", "$".yellow());
    println!("    Search download history");
    println!();
    println!("  {} ytdl history --export history.csv", "$".yellow());
    println!("    Export history to CSV file");
    println!();

    println!("{}", "Configuration:".cyan().bold());
    println!("  {} ytdl config", "$".yellow());
    println!("    Show current configuration");
    println!();
    println!("  {} ytdl -o ~/Videos \"URL\"", "$".yellow());
    println!("    Download to specific directory");
    println!();

    println!("{}", "Logging:".cyan().bold());
    println!("  {} ytdl -v \"URL\"", "$".yellow());
    println!("    Enable verbose logging (DEBUG level)");
    println!();
    println!("  {} ytdl --log-file \"URL\"", "$".yellow());
    println!("    Enable file logging to ~/.ytdl/logs/");
    println!();

    println!("{}", "For more information, use:".green().bold());
    println!("  {} ytdl --help", "$".yellow());
    println!();
}
