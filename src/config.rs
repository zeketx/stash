use crate::error::{Result, YtdlError};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub output_dir: PathBuf,
    pub quality: String,
    pub audio_only: bool,
    pub log_level: String,
    pub enable_file_logging: bool,
    pub enable_json_logging: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./downloads"),
            quality: "best".to_string(),
            audio_only: false,
            log_level: "info".to_string(),
            enable_file_logging: false,
            enable_json_logging: false,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            YtdlError::Config(format!("Failed to read config file: {}", e))
        })?;

        let config: Config = toml::from_str(&content).map_err(|e| {
            YtdlError::Config(format!("Failed to parse config file: {}", e))
        })?;

        debug!("Loaded configuration from file: {:?}", path);
        Ok(config)
    }

    pub fn load_with_env_overrides() -> Self {
        let mut config = Self::default();

        if let Ok(dir) = std::env::var("YTDL_OUTPUT_DIR") {
            config.output_dir = PathBuf::from(dir);
            debug!("Overriding output_dir from YTDL_OUTPUT_DIR");
        }

        if let Ok(level) = std::env::var("YTDL_LOG_LEVEL") {
            config.log_level = level;
            debug!("Overriding log_level from YTDL_LOG_LEVEL");
        }

        if let Ok(val) = std::env::var("YTDL_LOG_FILE") {
            config.enable_file_logging = val.to_lowercase() == "true" || val == "1";
            debug!("Overriding enable_file_logging from YTDL_LOG_FILE");
        }

        if let Ok(val) = std::env::var("YTDL_LOG_JSON") {
            config.enable_json_logging = val.to_lowercase() == "true" || val == "1";
            debug!("Overriding enable_json_logging from YTDL_LOG_JSON");
        }

        config
    }

    pub fn merge_with_cli(&mut self, cli_config: CliConfig) {
        if let Some(output) = cli_config.output {
            self.output_dir = output;
        }

        if let Some(quality) = cli_config.quality {
            self.quality = quality;
        }

        if cli_config.audio_only {
            self.audio_only = true;
        }

        if cli_config.enable_file_logging {
            self.enable_file_logging = true;
        }

        if cli_config.enable_json_logging {
            self.enable_json_logging = true;
        }
    }

    pub fn get_default_config_path() -> Option<PathBuf> {
        ProjectDirs::from("", "", "ytdl")
            .map(|proj_dirs| proj_dirs.config_dir().join("config.toml"))
    }

    pub fn log_config(&self) {
        info!("Configuration:");
        info!("  Output directory: {:?}", self.output_dir);
        info!("  Quality: {}", self.quality);
        info!("  Audio only: {}", self.audio_only);
        info!("  Log level: {}", self.log_level);
        info!("  File logging: {}", self.enable_file_logging);
        info!("  JSON logging: {}", self.enable_json_logging);
    }
}

#[derive(Debug, Default)]
pub struct CliConfig {
    pub output: Option<PathBuf>,
    pub quality: Option<String>,
    pub audio_only: bool,
    pub enable_file_logging: bool,
    pub enable_json_logging: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = Config::default();
        assert_eq!(config.output_dir, PathBuf::from("./downloads"));
        assert_eq!(config.quality, "best");
        assert!(!config.audio_only);
    }

    #[test]
    fn test_merge_with_cli() {
        let mut config = Config::default();
        let cli_config = CliConfig {
            output: Some(PathBuf::from("/tmp/videos")),
            quality: Some("720p".to_string()),
            audio_only: true,
            enable_file_logging: true,
            enable_json_logging: false,
        };

        config.merge_with_cli(cli_config);
        assert_eq!(config.output_dir, PathBuf::from("/tmp/videos"));
        assert_eq!(config.quality, "720p");
        assert!(config.audio_only);
    }
}
