use anyhow::Result;
use directories::ProjectDirs;
use std::path::PathBuf;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

pub struct LoggerConfig {
    pub level: Level,
    pub enable_file_logging: bool,
    pub enable_json_format: bool,
    pub log_dir: Option<PathBuf>,
    pub tui_mode: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: Level::INFO,
            enable_file_logging: false,
            enable_json_format: false,
            log_dir: None,
            tui_mode: false,
        }
    }
}

impl LoggerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn with_file_logging(mut self, enable: bool) -> Self {
        self.enable_file_logging = enable;
        self
    }

    pub fn with_json_format(mut self, enable: bool) -> Self {
        self.enable_json_format = enable;
        self
    }

    pub fn with_log_dir(mut self, dir: PathBuf) -> Self {
        self.log_dir = Some(dir);
        self
    }

    pub fn with_tui_mode(mut self, enable: bool) -> Self {
        self.tui_mode = enable;
        self
    }

    pub fn get_log_dir(&self) -> PathBuf {
        if let Some(ref dir) = self.log_dir {
            return dir.clone();
        }

        // Default to ~/.local/share/ytdl/logs on Linux/macOS
        if let Some(proj_dirs) = ProjectDirs::from("", "", "ytdl") {
            proj_dirs.data_dir().join("logs")
        } else {
            // Fallback to current directory
            PathBuf::from("./logs")
        }
    }
}

pub fn init_logger(config: LoggerConfig) -> Result<Option<WorkerGuard>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.level.as_str()));

    let mut guard = None;

    // TUI mode: Only log to file, never to stdout
    if config.tui_mode {
        let log_dir = config.get_log_dir();
        std::fs::create_dir_all(&log_dir)?;

        let file_appender = tracing_appender::rolling::daily(&log_dir, "ytdl");
        let (non_blocking, worker_guard) = tracing_appender::non_blocking(file_appender);
        guard = Some(worker_guard);

        let file_layer = fmt::layer()
            .with_writer(non_blocking)
            .with_thread_ids(true)
            .with_line_number(true)
            .with_file(true)
            .with_ansi(false)
            .with_filter(env_filter)
            .boxed();

        tracing_subscriber::registry()
            .with(file_layer)
            .init();

        return Ok(guard);
    }

    // CLI mode: Original behavior (stdout + optional file logging)
    if config.enable_file_logging {
        let log_dir = config.get_log_dir();
        std::fs::create_dir_all(&log_dir)?;

        let file_appender = tracing_appender::rolling::daily(&log_dir, "ytdl");
        let (non_blocking, worker_guard) = tracing_appender::non_blocking(file_appender);
        guard = Some(worker_guard);

        let console_layer = if config.enable_json_format {
            fmt::layer()
                .json()
                .with_writer(std::io::stdout)
                .with_filter(env_filter.clone())
                .boxed()
        } else {
            fmt::layer()
                .pretty()
                .with_writer(std::io::stdout)
                .with_thread_ids(true)
                .with_line_number(true)
                .with_file(true)
                .with_filter(env_filter.clone())
                .boxed()
        };

        let file_layer = if config.enable_json_format {
            fmt::layer()
                .json()
                .with_writer(non_blocking)
                .with_filter(env_filter)
                .boxed()
        } else {
            fmt::layer()
                .with_writer(non_blocking)
                .with_thread_ids(true)
                .with_line_number(true)
                .with_file(true)
                .with_ansi(false)
                .with_filter(env_filter)
                .boxed()
        };

        tracing_subscriber::registry()
            .with(console_layer)
            .with(file_layer)
            .init();
    } else {
        // Console-only logging (CLI mode)
        let console_layer = if config.enable_json_format {
            fmt::layer()
                .json()
                .with_writer(std::io::stdout)
                .with_filter(env_filter)
                .boxed()
        } else {
            fmt::layer()
                .pretty()
                .with_writer(std::io::stdout)
                .with_thread_ids(true)
                .with_line_number(true)
                .with_file(true)
                .with_filter(env_filter)
                .boxed()
        };

        tracing_subscriber::registry().with(console_layer).init();
    }

    Ok(guard)
}

pub fn level_from_verbosity(verbosity: u8) -> Level {
    match verbosity {
        0 => Level::INFO,
        1 => Level::DEBUG,
        2 => Level::TRACE,
        _ => Level::TRACE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_from_verbosity() {
        assert_eq!(level_from_verbosity(0), Level::INFO);
        assert_eq!(level_from_verbosity(1), Level::DEBUG);
        assert_eq!(level_from_verbosity(2), Level::TRACE);
        assert_eq!(level_from_verbosity(3), Level::TRACE);
    }

    #[test]
    fn test_logger_config_defaults() {
        let config = LoggerConfig::default();
        assert_eq!(config.level, Level::INFO);
        assert!(!config.enable_file_logging);
        assert!(!config.enable_json_format);
    }
}
