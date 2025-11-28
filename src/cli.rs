use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "ytdl")]
#[command(version = "0.1.0")]
#[command(about = "A comprehensive YouTube downloader with logging and observability", long_about = None)]
pub struct Cli {
    /// YouTube video or playlist URL
    pub url: Option<String>,

    /// Output directory
    #[arg(short, long, env = "YTDL_OUTPUT_DIR")]
    pub output: Option<PathBuf>,

    /// Video quality (e.g., 1080, 720, 480, or 'best')
    #[arg(short, long, default_value = "best")]
    pub quality: Option<String>,

    /// Download audio only as MP3
    #[arg(short, long)]
    pub audio_only: bool,

    /// Download as playlist
    #[arg(short, long)]
    pub playlist: bool,

    /// Show video information without downloading
    #[arg(long)]
    pub info: bool,

    /// Launch TUI mode
    #[arg(short, long)]
    pub interactive: bool,

    /// Batch download from file
    #[arg(short, long)]
    pub batch: Option<PathBuf>,

    /// Number of concurrent downloads for batch mode
    #[arg(long, default_value = "3")]
    pub concurrent: usize,

    /// Stop batch download on first error
    #[arg(long)]
    pub stop_on_error: bool,

    /// Use URL from clipboard
    #[arg(long)]
    pub clipboard: bool,

    /// Watch clipboard for URLs and auto-download
    #[arg(long)]
    pub watch: bool,

    /// Download specific playlist videos by range (e.g., 1-10)
    #[arg(long)]
    pub range: Option<String>,

    /// Custom folder name for playlist downloads
    #[arg(long)]
    pub folder: Option<String>,

    /// Resume incomplete downloads
    #[arg(long)]
    pub resume: bool,

    /// Verbose logging (-v: DEBUG, -vv: TRACE)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Quiet mode (errors only)
    #[arg(short = 'Q', long)]
    pub quiet: bool,

    /// Enable file logging
    #[arg(long, env = "YTDL_LOG_FILE")]
    pub log_file: bool,

    /// Use JSON log format
    #[arg(long, env = "YTDL_LOG_JSON")]
    pub log_json: bool,

    /// Custom config file path
    #[arg(long)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Show current configuration
    Config,

    /// Show download history
    History {
        /// Number of recent entries to show
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Search history by keyword
        #[arg(short, long)]
        search: Option<String>,

        /// Export history to CSV file
        #[arg(short, long)]
        export: Option<PathBuf>,
    },

    /// Clear download history
    ClearHistory {
        /// Clear entries older than N days
        #[arg(long)]
        older_than: Option<i64>,
    },

    /// Generate shell completions
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

impl Cli {
    pub fn validate(&self) -> Result<(), String> {
        if self.url.is_none() && self.batch.is_none() && self.command.is_none() && !self.interactive {
            return Err("No URL provided. Use --help for usage information.".to_string());
        }

        if self.quiet && self.verbose > 0 {
            return Err("Cannot use both --quiet and --verbose flags".to_string());
        }

        Ok(())
    }
}

mod clap_complete {
    use clap::ValueEnum;

    #[derive(Debug, Clone, Copy, ValueEnum)]
    pub enum Shell {
        Bash,
        Zsh,
        Fish,
        PowerShell,
        Elvish,
    }
}
