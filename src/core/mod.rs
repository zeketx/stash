//! Core business logic layer
//!
//! This module contains the domain logic for the application,
//! independent of infrastructure concerns.

pub mod batch;
pub mod history;
pub mod playlist;

pub use batch::{BatchDownloadItem, BatchDownloader, BatchDownloadStats, BatchProgress, DownloadStatus};
pub use history::{History, HistoryEntry};
pub use playlist::{PlaylistDownloader, PlaylistInfo};
