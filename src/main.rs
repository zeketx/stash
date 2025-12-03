mod batch;
mod clipboard;
mod cli;
mod config;
mod downloader;
mod error;
mod history;
mod logger;
mod notifications;
mod playlist;
mod tui;
mod ui;
mod utils;

use crate::batch::BatchDownloader;
use crate::cli::{Cli, Commands};
use crate::clipboard::{get_clipboard_url, ClipboardWatcher};
use crate::config::{CliConfig, Config};
use crate::downloader::Downloader;
use crate::error::Result;
use crate::history::{History, HistoryEntry};
use crate::logger::{init_logger, level_from_verbosity, LoggerConfig};
use crate::playlist::PlaylistDownloader;
use crate::utils::{check_ffmpeg, check_ytdlp, validate_youtube_url};
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
        return Err(error::YtdlError::Config(e));
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
        println!("{}", "=".repeat(80));
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
                return Err(error::YtdlError::Other(
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
        Err(error::YtdlError::Config(
            "No URL provided. Use --help for usage information.".to_string(),
        ))
    }
}

async fn show_video_info(url: &str, config: &Config) -> Result<()> {
    let downloader = Downloader::new(config.output_dir.clone(), config.quality.clone());

    println!("\n{}", "Fetching video information...".green().bold());
    let video_info = downloader.fetch_video_info(url).await?;

    println!("\n{}", "Video Information:".green().bold());
    println!("{}", "=".repeat(80));
    println!("Title: {}", video_info.title);
    println!("Uploader: {}", video_info.uploader);

    if let Some(duration) = video_info.duration {
        let hours = duration / 3600;
        let minutes = (duration % 3600) / 60;
        let seconds = duration % 60;
        if hours > 0 {
            println!("Duration: {}:{:02}:{:02}", hours, minutes, seconds);
        } else {
            println!("Duration: {}:{:02}", minutes, seconds);
        }
    }

    if let Some(view_count) = video_info.view_count {
        println!("Views: {}", view_count.to_string().as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<std::result::Result<Vec<&str>, _>>()
            .unwrap()
            .join(","));
    }

    if let Some(upload_date) = video_info.upload_date {
        println!("Upload Date: {}", upload_date);
    }

    if let Some(description) = video_info.description {
        let desc_preview = if description.len() > 200 {
            format!("{}...", &description[..200])
        } else {
            description
        };
        println!("\nDescription:");
        println!("{}", desc_preview);
    }

    println!("\n{}", "Available Formats:".cyan().bold());
    println!("{}", "-".repeat(80));

    let mut video_formats: Vec<_> = video_info.formats.iter()
        .filter(|f| f.vcodec.as_ref().map(|v| v != "none").unwrap_or(false))
        .collect();
    video_formats.sort_by(|a, b| {
        b.resolution.as_ref()
            .and_then(|r| r.split('x').nth(1))
            .and_then(|h| h.parse::<u32>().ok())
            .unwrap_or(0)
            .cmp(&a.resolution.as_ref()
                .and_then(|r| r.split('x').nth(1))
                .and_then(|h| h.parse::<u32>().ok())
                .unwrap_or(0))
    });

    for (i, format) in video_formats.iter().take(10).enumerate() {
        let res = format.resolution.as_ref().map(|s| s.as_str()).unwrap_or("unknown");
        let fps = format.fps.map(|f| format!("{}fps", f)).unwrap_or_else(|| "".to_string());
        let size = format.filesize.map(|s| format!("{:.1} MB", s as f64 / 1_000_000.0))
            .unwrap_or_else(|| "unknown size".to_string());
        println!("  {}. {} {} - {} ({})", i + 1, res, fps, size, format.ext);
    }

    println!("\n{}", "To download, run:".yellow().bold());
    println!("  ytdl \"{}\"", url);

    Ok(())
}

async fn download_single_url(url: &str, config: &Config, history: &mut History, resume: bool) -> Result<()> {
    let downloader = Downloader::new(config.output_dir.clone(), config.quality.clone());

    if history.contains_url(url) {
        warn!("URL already downloaded");
        if let Some(entry) = history.get_entry_by_url(url) {
            println!("\n{} This URL was already downloaded:", "⚠".yellow().bold());
            println!("  Title: {}", entry.title);
            println!("  Date: {}", entry.timestamp.format("%Y-%m-%d %H:%M:%S"));
            println!("  Path: {:?}", entry.file_path);
        }

        if !resume {
            println!("\nUse --resume to re-download");
            return Ok(());
        }
    }

    println!("\n{}", "Starting download...".green().bold());

    let output_path = if resume {
        downloader.resume_download(url, config.audio_only).await?
    } else {
        downloader.download(url, config.audio_only).await?
    };

    println!(
        "\n{} Downloaded to: {}",
        "✓".green().bold(),
        output_path.display()
    );

    let video_info = downloader.fetch_video_info(url).await.ok();
    let title = video_info
        .as_ref()
        .map(|v| v.title.clone())
        .unwrap_or_else(|| url.to_string());

    let file_size = tokio::fs::metadata(&output_path)
        .await
        .map(|m| m.len())
        .unwrap_or(0);

    let entry = HistoryEntry::new(
        url.to_string(),
        title,
        output_path,
        file_size,
        config.quality.clone(),
        if config.audio_only {
            "mp3".to_string()
        } else {
            "mp4".to_string()
        },
    );

    history.add_entry(entry);
    history.save()?;

    Ok(())
}

async fn handle_playlist_download(
    url: &str,
    config: &Config,
    range: Option<&str>,
    folder: Option<&str>,
) -> Result<()> {
    let mut playlist_downloader = PlaylistDownloader::new(config.output_dir.clone(), config.quality.clone());

    if let Some(folder_name) = folder {
        playlist_downloader = playlist_downloader.with_folder(folder_name.to_string());
    }

    if let Some(range_str) = range {
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() == 2 {
            let start: usize = parts[0].parse().map_err(|_| {
                error::YtdlError::Config("Invalid range format".to_string())
            })?;
            let end: usize = parts[1].parse().map_err(|_| {
                error::YtdlError::Config("Invalid range format".to_string())
            })?;

            println!("\n{}", "Downloading playlist videos...".green().bold());
            let paths = playlist_downloader
                .download_range(url, start, end, config.audio_only)
                .await?;

            println!(
                "\n{} Downloaded {} videos from playlist",
                "✓".green().bold(),
                paths.len()
            );
        } else {
            return Err(error::YtdlError::Config(
                "Invalid range format. Use: --range 1-10".to_string(),
            ));
        }
    } else {
        let playlist_info = playlist_downloader.fetch_playlist_info(url).await?;

        println!("\n{}", "Playlist Information:".green().bold());
        println!("{}", "=".repeat(80));
        println!("Title: {}", playlist_info.title);
        if let Some(ref uploader) = playlist_info.uploader {
            println!("Uploader: {}", uploader);
        }
        println!("Videos: {}", playlist_info.video_count);

        println!("\n{}", "Downloading all videos...".green().bold());
        let paths = playlist_downloader
            .download_playlist(&playlist_info, config.audio_only)
            .await?;

        println!(
            "\n{} Downloaded {} of {} videos",
            "✓".green().bold(),
            paths.len(),
            playlist_info.video_count
        );
    }

    Ok(())
}

async fn handle_subcommand(command: Commands, config: &Config, history: &mut History) -> Result<()> {
    match command {
        Commands::Config => {
            println!("\n{}", "Current Configuration:".green().bold());
            println!("{}", "=".repeat(80));
            println!("Output Directory: {:?}", config.output_dir);
            println!("Quality: {}", config.quality);
            println!("Audio Only: {}", config.audio_only);
            println!("Log Level: {}", config.log_level);
            println!("File Logging: {}", config.enable_file_logging);
            println!("JSON Logging: {}", config.enable_json_logging);
            println!("Concurrent Downloads: {:?}", config.concurrent_downloads);
            println!("Skip Duplicates: {:?}", config.skip_duplicates);

            if let Some(path) = Config::get_default_config_path() {
                println!("\nDefault config path: {:?}", path);
            }

            if let Some(path) = History::get_history_file_path() {
                println!("History file path: {:?}", path);
            }

            Ok(())
        }
        Commands::History { limit, search, export } => {
            if let Some(export_path) = export {
                history.export_to_csv(&export_path)?;
                println!("{} Exported history to: {:?}", "✓".green().bold(), export_path);
                return Ok(());
            }

            let entries = if let Some(query) = search {
                history.search(&query)
            } else {
                history.get_recent(limit)
            };

            if entries.is_empty() {
                println!("\n{}", "No history entries found".yellow());
                return Ok(());
            }

            println!("\n{}", "Download History:".green().bold());
            println!("{}", "=".repeat(80));

            for entry in entries {
                println!("\nTitle: {}", entry.title);
                println!("URL: {}", entry.url);
                println!("Date: {}", entry.timestamp.format("%Y-%m-%d %H:%M:%S"));
                println!("Size: {} bytes", entry.file_size);
                println!("Quality: {} ({})", entry.quality, entry.format);
                println!("Path: {:?}", entry.file_path);
            }

            println!("\nTotal entries: {}", history.len());

            Ok(())
        }
        Commands::ClearHistory { older_than } => {
            if let Some(days) = older_than {
                history.clear_older_than(days);
                println!(
                    "{} Cleared history entries older than {} days",
                    "✓".green().bold(),
                    days
                );
            } else {
                history.clear();
                println!("{} Cleared all history", "✓".green().bold());
            }

            history.save()?;
            Ok(())
        }
        Commands::Completions { shell: _ } => {
            warn!("Shell completions not yet implemented");
            Err(error::YtdlError::Other(
                "Shell completions coming in a future release".to_string(),
            ))
        }
    }
}

fn print_examples() {
    println!("{}", "Common Usage Examples:".green().bold());
    println!("{}", "=".repeat(80));
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
