use crate::tui::screens::SettingsState;
use crate::tui::theme::Theme;
use crate::tui::widgets::{BlinkingCursor, CheckmarkAnimation, ColorCycle, PulsingSelection, Spinner};
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
        retry_count: usize,
    },
    Help {
        previous_state: Box<AppState>,
    },
    Settings {
        settings: SettingsState,
        previous_state: Box<AppState>,
    },
}

pub struct App {
    pub state: AppState,
    pub theme: Theme,
    pub should_quit: bool,
    pub spinner: Spinner,
    pub cursor: BlinkingCursor,
    pub checkmark: CheckmarkAnimation,
    pub color_cycle: ColorCycle,
    pub pulsing_selection: PulsingSelection,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::UrlInput {
                input: String::new(),
                cursor_pos: 0,
                is_valid: None,
                validation_message: String::from("Paste a YouTube URL or press Ctrl+V"),
                recent_downloads: vec![],
            },
            theme: Theme::default(),
            should_quit: false,
            spinner: Spinner::new(),
            cursor: BlinkingCursor::new(),
            checkmark: CheckmarkAnimation::new(),
            color_cycle: ColorCycle::new(),
            pulsing_selection: PulsingSelection::new(),
        }
    }

    /// Update animations (should be called on tick)
    pub fn tick(&mut self) {
        self.spinner.tick();
        self.cursor.tick();
        self.checkmark.tick();
        self.color_cycle.tick();
        self.pulsing_selection.tick();
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn go_to_url_input(&mut self) {
        self.state = AppState::UrlInput {
            input: String::new(),
            cursor_pos: 0,
            is_valid: None,
            validation_message: String::from("Paste a YouTube URL or press Ctrl+V"),
            recent_downloads: vec![],
        };
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
            retry_count: 0,
        };
    }

    pub fn go_to_help(&mut self) {
        let previous = Box::new(self.state.clone());
        self.state = AppState::Help {
            previous_state: previous,
        };
    }

    pub fn go_to_settings(&mut self, output_dir: String, quality: String, concurrent: usize) {
        let previous = Box::new(self.state.clone());
        self.state = AppState::Settings {
            settings: SettingsState::new(output_dir, quality, concurrent),
            previous_state: previous,
        };
    }

    pub fn back_from_overlay(&mut self) {
        match &self.state {
            AppState::Help { previous_state } => {
                self.state = (**previous_state).clone();
            }
            AppState::Settings { previous_state, .. } => {
                self.state = (**previous_state).clone();
            }
            _ => {}
        }
    }

    pub fn select_next_setting(&mut self) {
        if let AppState::Settings { ref mut settings, .. } = self.state {
            if settings.selected_index < 4 {
                settings.selected_index += 1;
            }
        }
    }

    pub fn select_previous_setting(&mut self) {
        if let AppState::Settings { ref mut settings, .. } = self.state {
            if settings.selected_index > 0 {
                settings.selected_index -= 1;
            }
        }
    }

    pub fn update_input(&mut self, input: String, cursor_pos: usize) {
        if let AppState::UrlInput { input: ref mut i, cursor_pos: ref mut c, ref mut is_valid, ref mut validation_message, .. } = self.state {
            *i = input.clone();
            *c = cursor_pos;

            // Basic URL validation
            if input.is_empty() {
                *is_valid = None;
                *validation_message = "Paste a YouTube URL or press Ctrl+V".to_string();
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
