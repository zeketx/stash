//! Shared utilities and types
//!
//! This module contains utilities and types used across all layers

pub mod constants;
pub mod error;
pub mod utils;

pub use error::{Result, YtdlError};
pub use utils::{check_ffmpeg, check_ytdlp, format_bytes, validate_youtube_url};

// Re-export commonly used constants
pub use constants::{
    BYTES_PER_GB, BYTES_PER_KB, BYTES_PER_MB, BYTES_PER_SI_GB, BYTES_PER_SI_KB,
    BYTES_PER_SI_MB, SEPARATOR_LINE, SEPARATOR_WIDTH, SUBSEPARATOR_LINE,
};
