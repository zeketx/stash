//! Infrastructure layer
//!
//! This module contains code that interacts with external systems
//! and dependencies (yt-dlp, clipboard, file system, logging, etc.)

pub mod clipboard;
pub mod downloader;
pub mod logger;
pub mod notifications;

pub use clipboard::{get_clipboard_url, ClipboardWatcher};
pub use downloader::{DownloadProgressInfo, Downloader, Format, VideoMetadata};
pub use logger::{init_logger, level_from_verbosity, LoggerConfig};
pub use notifications::{are_notifications_available, notify_download_complete, notify_download_error};
