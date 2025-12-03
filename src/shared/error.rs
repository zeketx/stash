use thiserror::Error;

#[derive(Error, Debug)]
pub enum YtdlError {
    #[error("Invalid YouTube URL: {0}")]
    InvalidUrl(String),

    #[error("yt-dlp not found in PATH. Please install yt-dlp:\n  macOS: brew install yt-dlp\n  Linux: pip install yt-dlp\n  Windows: winget install yt-dlp")]
    YtdlpNotFound,

    #[error("yt-dlp execution failed: {0}")]
    YtdlpFailed(String),

    #[error("FFmpeg not found in PATH. Please install FFmpeg:\n  macOS: brew install ffmpeg\n  Linux: apt install ffmpeg\n  Windows: winget install ffmpeg")]
    FfmpegNotFound,

    #[error("Insufficient disk space. Required: {required} GB, Available: {available} GB")]
    InsufficientDiskSpace { required: u64, available: u64 },

    #[error("Network error: {0}")]
    Network(String),

    #[error("Failed to parse JSON from yt-dlp: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Failed to parse progress output: {0}")]
    ProgressParse(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Video format not available: {0}")]
    FormatNotAvailable(String),

    #[error("Download interrupted by user")]
    Interrupted,

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, YtdlError>;
