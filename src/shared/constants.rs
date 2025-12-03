//! Application-wide constants
//!
//! This module centralizes magic numbers and strings used throughout the application

// UI Formatting Constants
pub const SEPARATOR_LINE: &str = "=";
pub const SUBSEPARATOR_LINE: &str = "-";
pub const SEPARATOR_WIDTH: usize = 80;

// Time Constants
pub const SECONDS_PER_MINUTE: u64 = 60;
pub const SECONDS_PER_HOUR: u64 = 3600;
pub const MINUTES_PER_HOUR: u64 = 60;

// Byte Size Constants
pub const BYTES_PER_KB: f64 = 1024.0;
pub const BYTES_PER_MB: f64 = 1024.0 * 1024.0;
pub const BYTES_PER_GB: f64 = 1024.0 * 1024.0 * 1024.0;

// Alternative byte units (SI units for display)
pub const BYTES_PER_SI_KB: f64 = 1_000.0;
pub const BYTES_PER_SI_MB: f64 = 1_000_000.0;
pub const BYTES_PER_SI_GB: f64 = 1_000_000_000.0;

// Notification Constants
pub const NOTIFICATION_TIMEOUT_MS: u32 = 5000;

// Default Configuration Values
pub const DEFAULT_CONCURRENT_DOWNLOADS: usize = 3;
pub const MAX_CONCURRENT_DOWNLOADS: usize = 10;
pub const MIN_CONCURRENT_DOWNLOADS: usize = 1;

// History Constants
pub const DEFAULT_HISTORY_LIMIT: usize = 10;

// TUI Animation Constants
pub const ANIMATION_FRAME_DELAY_MS: u64 = 100;
pub const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
pub const CHECKMARK_FRAMES: &[&str] = &["⠀", "⠁", "⠉", "⠋", "⠛", "⠟", "⠿", "✓"];

// Format Display Constants
pub const MAX_FORMAT_DISPLAY_COUNT: usize = 10;
pub const DEFAULT_VIDEO_FORMAT_COUNT: usize = 5;

// Error Messages
pub const ERROR_NO_YTDLP: &str = "yt-dlp is not installed or not in PATH";
pub const ERROR_NO_FFMPEG: &str = "ffmpeg is not installed or not in PATH";
pub const ERROR_NO_CLIPBOARD_URL: &str = "No valid YouTube URL found in clipboard";

// Branding
pub const APP_NAME: &str = "ytdl";
pub const APP_REPO: &str = "https://github.com/anthropics/claude-code";
pub const GENERATED_WITH_CLAUDE: &str = "🤖 Generated with [Claude Code](https://claude.com/claude-code)";
pub const CO_AUTHORED_BY: &str = "Co-Authored-By: Claude <noreply@anthropic.com>";
