use crate::error::Result;
use crate::ui::{
    app::{App, AppState},
    events::{is_back_key, is_quit_key, Event, EventHandler},
    screens::{
        render_downloading, render_error, render_fetching, render_format_selection,
        render_help, render_settings, render_success, render_url_input, render_welcome,
    },
    terminal::{restore_terminal, setup_panic_hook, setup_terminal},
};
use crossterm::event::{KeyCode, KeyModifiers};
use tracing::{error, info};

pub async fn run_tui() -> Result<()> {
    // Setup panic hook to restore terminal
    setup_panic_hook();

    // Setup terminal
    let mut terminal = setup_terminal().map_err(|e| {
        crate::error::YtdlError::Other(format!("Failed to setup terminal: {}", e))
    })?;
    info!("Terminal initialized in raw mode");

    // Create application state
    let mut app = App::new();
    let event_handler = EventHandler::default();

    // Main event loop
    let result = loop {
        // Render current state
        if let Err(e) = terminal.draw(|frame| render(&mut app, frame)) {
            error!("Failed to render frame: {}", e);
            break Err(crate::error::YtdlError::Other(format!(
                "Render error: {}",
                e
            )));
        }

        // Handle events
        match event_handler.next() {
            Ok(event) => {
                if let Err(e) = handle_event(&mut app, event).await {
                    error!("Error handling event: {}", e);
                    app.go_to_error(
                        "Event Error".to_string(),
                        format!("{}", e),
                        vec!["Try restarting the application".to_string()],
                    );
                }
            }
            Err(e) => {
                error!("Error reading event: {}", e);
                break Err(crate::error::YtdlError::Other(format!(
                    "Event error: {}",
                    e
                )));
            }
        }

        // Check if we should quit
        if app.should_quit {
            info!("Application quit requested");
            break Ok(());
        }
    };

    // Restore terminal
    restore_terminal(&mut terminal).map_err(|e| {
        crate::error::YtdlError::Other(format!("Failed to restore terminal: {}", e))
    })?;
    info!("Terminal restored");

    result
}

fn render(app: &mut App, frame: &mut ratatui::Frame) {
    match &app.state {
        AppState::Welcome => {
            render_welcome(frame, &app.theme);
        }
        AppState::UrlInput {
            input,
            cursor_pos,
            is_valid,
            validation_message,
            recent_downloads,
        } => {
            render_url_input(
                frame,
                &app.theme,
                input,
                *cursor_pos,
                *is_valid,
                validation_message,
                recent_downloads,
            );
        }
        AppState::FetchingInfo { url } => {
            render_fetching(frame, &app.theme, url);
        }
        AppState::FormatSelection {
            video_info,
            formats,
            selected_index,
            ..
        } => {
            render_format_selection(frame, &app.theme, video_info, formats, *selected_index);
        }
        AppState::Downloading {
            video_info,
            format,
            progress,
            ..
        } => {
            render_downloading(frame, &app.theme, video_info, format, progress);
        }
        AppState::Success { info } => {
            render_success(frame, &app.theme, info);
        }
        AppState::Error {
            error_type,
            message,
            suggestions,
            ..
        } => {
            render_error(frame, &app.theme, error_type, message, suggestions);
        }
        AppState::Help { .. } => {
            render_help(frame, &app.theme);
        }
        AppState::Settings { settings, .. } => {
            render_settings(frame, &app.theme, settings, settings.selected_index);
        }
    }
}

async fn handle_event(app: &mut App, event: Event) -> Result<()> {
    match event {
        Event::Key(key) => {
            // Global quit key
            if is_quit_key(key) {
                app.quit();
                return Ok(());
            }

            // Global help key (h or ?) - but NOT in URL input where user might be typing
            if matches!(key.code, KeyCode::Char('h') | KeyCode::Char('?')) && key.modifiers.is_empty() {
                // Don't open help if already in help, settings, or URL input
                if !matches!(app.state, AppState::Help { .. } | AppState::Settings { .. } | AppState::UrlInput { .. }) {
                    app.go_to_help();
                    return Ok(());
                }
            }

            // State-specific key handling
            match &app.state {
                AppState::Welcome => {
                    match key.code {
                        KeyCode::Enter => app.go_to_url_input(),
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            app.go_to_settings(
                                "./downloads".to_string(),
                                "best".to_string(),
                                3,
                            );
                        }
                        _ => {}
                    }
                }
                AppState::UrlInput {
                    input, cursor_pos, ..
                } => {
                    match key.code {
                        KeyCode::Char(c) if key.modifiers.is_empty() => {
                            let mut new_input = input.clone();
                            new_input.insert(*cursor_pos, c);
                            app.update_input(new_input, cursor_pos + 1);
                        }
                        KeyCode::Backspace => {
                            if *cursor_pos > 0 {
                                let mut new_input = input.clone();
                                new_input.remove(cursor_pos - 1);
                                app.update_input(new_input, cursor_pos - 1);
                            }
                        }
                        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.update_input(String::new(), 0);
                        }
                        KeyCode::Enter => {
                            // Validate and proceed
                            if input.contains("youtube.com") || input.contains("youtu.be") {
                                app.start_fetching_info(input.clone());
                                // Simulate fetching (in real implementation, this would be async)
                                simulate_video_fetch(app).await;
                            }
                        }
                        KeyCode::Esc => {
                            app.go_to_welcome();
                        }
                        _ => {}
                    }
                }
                AppState::FetchingInfo { .. } => {
                    if is_back_key(key) {
                        app.go_to_url_input();
                    }
                }
                AppState::FormatSelection { .. } => {
                    match key.code {
                        KeyCode::Up => app.select_previous_format(),
                        KeyCode::Down => app.select_next_format(),
                        KeyCode::Enter => {
                            app.start_download();
                            // Simulate download (in real implementation, this would be async)
                            simulate_download(app).await;
                        }
                        KeyCode::Char('a') | KeyCode::Char('A') => {
                            // Quick select audio only - would select first audio format
                            app.start_download();
                            simulate_download(app).await;
                        }
                        KeyCode::Esc => {
                            app.go_to_url_input();
                        }
                        _ => {}
                    }
                }
                AppState::Downloading { .. } => {
                    // During download, only allow cancel
                    if matches!(
                        (key.code, key.modifiers),
                        (KeyCode::Char('c'), KeyModifiers::CONTROL)
                    ) {
                        app.go_to_url_input();
                    }
                }
                AppState::Success { .. } => {
                    match key.code {
                        KeyCode::Char('n') | KeyCode::Char('N') => {
                            app.go_to_url_input();
                        }
                        KeyCode::Char('o') | KeyCode::Char('O') => {
                            // Open file not implemented
                            info!("Open file requested");
                        }
                        KeyCode::Char('f') | KeyCode::Char('F') => {
                            // Open folder not implemented
                            info!("Open folder requested");
                        }
                        _ => {}
                    }
                }
                AppState::Error { .. } => {
                    match key.code {
                        KeyCode::Char('n') | KeyCode::Char('N') => {
                            app.go_to_url_input();
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            // Retry - go back to URL input
                            app.go_to_url_input();
                        }
                        _ => {}
                    }
                }
                AppState::Help { .. } => {
                    if is_back_key(key) {
                        app.back_from_overlay();
                    }
                }
                AppState::Settings { .. } => {
                    match key.code {
                        KeyCode::Up => app.select_previous_setting(),
                        KeyCode::Down => app.select_next_setting(),
                        KeyCode::Enter => {
                            // TODO: Edit selected setting
                            info!("Edit setting selected");
                        }
                        KeyCode::Esc => {
                            app.back_from_overlay();
                        }
                        _ => {}
                    }
                }
            }
        }
        Event::Resize(_, _) => {
            // Terminal was resized, will be handled automatically by next render
        }
        Event::Tick => {
            // Regular tick for animations and updates
            app.tick();
        }
    }

    Ok(())
}

// Simulate video fetch for demo purposes
async fn simulate_video_fetch(app: &mut App) {
    use crate::ui::app::{FormatOption, VideoInfo};
    use tokio::time::{sleep, Duration};

    // Simulate network delay
    sleep(Duration::from_millis(500)).await;

    let video_info = VideoInfo {
        title: "Example Video Title".to_string(),
        uploader: "Example Channel".to_string(),
        duration: "5:30".to_string(),
        view_count: Some("1,234,567".to_string()),
        upload_date: Some("2024-01-15".to_string()),
    };

    let formats = vec![
        FormatOption {
            label: "Best Quality".to_string(),
            resolution: "1920x1080".to_string(),
            file_size: "~500 MB".to_string(),
            format_id: "best".to_string(),
        },
        FormatOption {
            label: "1080p".to_string(),
            resolution: "1920x1080".to_string(),
            file_size: "~450 MB".to_string(),
            format_id: "1080p".to_string(),
        },
        FormatOption {
            label: "720p".to_string(),
            resolution: "1280x720".to_string(),
            file_size: "~200 MB".to_string(),
            format_id: "720p".to_string(),
        },
        FormatOption {
            label: "480p".to_string(),
            resolution: "854x480".to_string(),
            file_size: "~100 MB".to_string(),
            format_id: "480p".to_string(),
        },
        FormatOption {
            label: "Audio Only".to_string(),
            resolution: "N/A".to_string(),
            file_size: "~5 MB".to_string(),
            format_id: "audio".to_string(),
        },
    ];

    if let AppState::FetchingInfo { url } = &app.state {
        app.show_format_selection(url.clone(), video_info, formats);
    }
}

// Simulate download for demo purposes
async fn simulate_download(app: &mut App) {
    use crate::ui::app::{DownloadProgress, DownloadSuccess};
    use std::path::PathBuf;
    use tokio::time::{sleep, Duration};

    // Simulate download progress
    for i in 0..=100 {
        if let AppState::Downloading { progress, .. } = &mut app.state {
            let new_progress = DownloadProgress {
                percentage: i as f64,
                downloaded_bytes: (i as u64 * 5_000_000),
                total_bytes: 500_000_000,
                speed: 5_000_000.0,
                eta: Some((100 - i) as u64),
                elapsed: i as u64,
            };
            app.update_progress(new_progress);
        }
        sleep(Duration::from_millis(50)).await;
    }

    // Mark as complete
    let success_info = DownloadSuccess {
        filename: "example_video.mp4".to_string(),
        file_size: "500 MB".to_string(),
        duration: "5 seconds".to_string(),
        save_location: PathBuf::from("./downloads"),
    };

    app.download_complete(success_info);
}
