use crate::ui::theme::Theme;
use chrono::{DateTime, Local};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub title: String,
    pub uploader: String,
    pub duration: String,
    pub view_count: Option<String>,
    pub upload_date: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FormatOption {
    pub label: String,
    pub resolution: String,
    pub file_size: String,
    pub format_id: String,
}

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub percentage: f64,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub speed: f64,
    pub eta: Option<u64>,
    pub elapsed: u64,
}

#[derive(Debug, Clone)]
pub struct DownloadSuccess {
    pub filename: String,
    pub file_size: String,
    pub duration: String,
    pub save_location: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DownloadHistory {
    pub title: String,
    pub timestamp: DateTime<Local>,
    pub url: String,
}

#[derive(Debug, Clone)]
pub enum AppState {
    Welcome,
    UrlInput {
        input: String,
        cursor_pos: usize,
        is_valid: Option<bool>,
        validation_message: String,
        recent_downloads: Vec<DownloadHistory>,
    },
    FetchingInfo {
        url: String,
    },
    FormatSelection {
        url: String,
        video_info: VideoInfo,
        formats: Vec<FormatOption>,
        selected_index: usize,
    },
    Downloading {
        url: String,
        video_info: VideoInfo,
        format: FormatOption,
        progress: DownloadProgress,
    },
    Success {
        info: DownloadSuccess,
    },
    Error {
        error_type: String,
        message: String,
        suggestions: Vec<String>,
        last_url: Option<String>,
    },
}

pub struct App {
    pub state: AppState,
    pub theme: Theme,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Welcome,
            theme: Theme::default(),
            should_quit: false,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn go_to_url_input(&mut self) {
        self.state = AppState::UrlInput {
            input: String::new(),
            cursor_pos: 0,
            is_valid: None,
            validation_message: String::from("Paste a YouTube URL"),
            recent_downloads: vec![],
        };
    }

    pub fn go_to_welcome(&mut self) {
        self.state = AppState::Welcome;
    }

    pub fn go_to_error(&mut self, error_type: String, message: String, suggestions: Vec<String>) {
        let last_url = match &self.state {
            AppState::UrlInput { input, .. } => Some(input.clone()),
            AppState::FetchingInfo { url } => Some(url.clone()),
            AppState::Downloading { url, .. } => Some(url.clone()),
            _ => None,
        };

        self.state = AppState::Error {
            error_type,
            message,
            suggestions,
            last_url,
        };
    }

    pub fn update_input(&mut self, input: String, cursor_pos: usize) {
        if let AppState::UrlInput { input: ref mut i, cursor_pos: ref mut c, ref mut is_valid, ref mut validation_message, .. } = self.state {
            *i = input.clone();
            *c = cursor_pos;

            // Basic URL validation
            if input.is_empty() {
                *is_valid = None;
                *validation_message = "Paste a YouTube URL".to_string();
            } else if input.contains("youtube.com") || input.contains("youtu.be") {
                *is_valid = Some(true);
                *validation_message = "Valid YouTube URL".to_string();
            } else {
                *is_valid = Some(false);
                *validation_message = "Invalid YouTube URL".to_string();
            }
        }
    }

    pub fn start_fetching_info(&mut self, url: String) {
        self.state = AppState::FetchingInfo { url };
    }

    pub fn show_format_selection(&mut self, url: String, video_info: VideoInfo, formats: Vec<FormatOption>) {
        self.state = AppState::FormatSelection {
            url,
            video_info,
            formats,
            selected_index: 0,
        };
    }

    pub fn select_next_format(&mut self) {
        if let AppState::FormatSelection { ref mut selected_index, ref formats, .. } = self.state {
            if *selected_index < formats.len() - 1 {
                *selected_index += 1;
            }
        }
    }

    pub fn select_previous_format(&mut self) {
        if let AppState::FormatSelection { ref mut selected_index, .. } = self.state {
            if *selected_index > 0 {
                *selected_index -= 1;
            }
        }
    }

    pub fn start_download(&mut self) {
        if let AppState::FormatSelection { url, video_info, formats, selected_index } = &self.state {
            let format = formats[*selected_index].clone();
            self.state = AppState::Downloading {
                url: url.clone(),
                video_info: video_info.clone(),
                format,
                progress: DownloadProgress {
                    percentage: 0.0,
                    downloaded_bytes: 0,
                    total_bytes: 0,
                    speed: 0.0,
                    eta: None,
                    elapsed: 0,
                },
            };
        }
    }

    pub fn update_progress(&mut self, progress: DownloadProgress) {
        if let AppState::Downloading { progress: p, .. } = &mut self.state {
            *p = progress;
        }
    }

    pub fn download_complete(&mut self, info: DownloadSuccess) {
        self.state = AppState::Success { info };
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
