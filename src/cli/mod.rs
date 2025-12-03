//! Command-line interface layer
//!
//! This module handles CLI argument parsing, configuration,
//! and command execution

pub mod commands;
pub mod config;
pub mod parser;

pub use commands::{
    download_single_url, handle_clear_history_command, handle_config_command,
    handle_history_command, handle_playlist_download, show_video_info,
};
pub use config::{CliConfig, Config};
pub use parser::{Cli, Commands};
