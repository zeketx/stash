//! Shared utilities and types
//!
//! This module contains utilities and types used across all layers

pub mod error;
pub mod utils;

pub use error::{Result, YtdlError};
pub use utils::{check_ffmpeg, check_ytdlp, format_bytes, validate_youtube_url};
