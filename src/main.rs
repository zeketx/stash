mod cli;
mod config;
mod downloader;
mod error;
mod logger;
mod utils;

use crate::cli::{Cli, Commands};
use crate::config::{CliConfig, Config};
use crate::downloader::Downloader;
use crate::error::Result;
use crate::logger::{init_logger, level_from_verbosity, LoggerConfig};
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
    config.log_config();

    // Handle subcommands
    if let Some(command) = cli.command {
        return handle_subcommand(command, &config).await;
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
        error!("Interactive TUI mode not yet implemented");
        return Err(error::YtdlError::Other(
            "TUI mode coming in Phase 2".to_string(),
        ));
    }

    // Handle batch download
    if let Some(_batch_file) = cli.batch {
        error!("Batch download not yet implemented");
        return Err(error::YtdlError::Other(
            "Batch download coming in a future release".to_string(),
        ));
    }

    // Handle single URL download
    if let Some(url) = &cli.url {
        validate_youtube_url(url)?;

        let downloader = Downloader::new(config.output_dir.clone(), config.quality.clone());

        // Show info only
        if cli.info {
            info!("Fetching video information...");
            let video_info = downloader.fetch_video_info(url).await?;

            println!("\n{}", "Video Information:".green().bold());
            println!("{}", "=".repeat(80));
            println!("Title: {}", video_info.title);
            println!("Uploader: {}", video_info.uploader);
            println!("ID: {}", video_info.id);
            if let Some(duration) = video_info.duration {
                println!("Duration: {}", utils::format_duration(duration));
            }
            if let Some(views) = video_info.view_count {
                println!("Views: {}", views);
            }
            if let Some(ref date) = video_info.upload_date {
                println!("Upload Date: {}", date);
            }
            if let Some(ref thumb) = video_info.thumbnail {
                println!("Thumbnail: {}", thumb);
            }

            downloader.list_formats(&video_info);
            return Ok(());
        }

        // Download the video
        println!("\n{}", "Starting download...".green().bold());
        let output_path = downloader.download(url, config.audio_only).await?;
        println!(
            "\n{} Downloaded to: {}",
            "✓".green().bold(),
            output_path.display()
        );

        Ok(())
    } else {
        Err(error::YtdlError::Config(
            "No URL provided. Use --help for usage information.".to_string(),
        ))
    }
}

async fn handle_subcommand(command: Commands, config: &Config) -> Result<()> {
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

            if let Some(path) = Config::get_default_config_path() {
                println!("\nDefault config path: {:?}", path);
            }

            Ok(())
        }
        Commands::History => {
            warn!("History feature not yet implemented");
            Err(error::YtdlError::Other(
                "History feature coming in a future release".to_string(),
            ))
        }
        Commands::ClearHistory => {
            warn!("Clear history feature not yet implemented");
            Err(error::YtdlError::Other(
                "Clear history feature coming in a future release".to_string(),
            ))
        }
        Commands::Completions { shell: _ } => {
            warn!("Shell completions not yet implemented");
            Err(error::YtdlError::Other(
                "Shell completions coming in a future release".to_string(),
            ))
        }
    }
}
