use crate::cli::config::Config;
use crate::infra::downloader::{Downloader, DownloadProgressInfo};
use crate::shared::Result;
use crate::tui::{
    app::{App, AppState, DownloadProgress, DownloadSuccess, FormatOption, VideoInfo},
    events::{is_back_key, is_quit_key, Event, EventHandler},
    screens::{
        render_downloading, render_error, render_fetching, render_format_selection,
        render_help, render_settings, render_success, render_url_input, render_welcome,
    },
    terminal::{restore_terminal, setup_panic_hook, setup_terminal},
};
use crossterm::event::{KeyCode, KeyModifiers};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

pub async fn run_tui() -> Result<()> {
    // Setup panic hook to restore terminal
    setup_panic_hook();

    // Setup terminal
    let mut terminal = setup_terminal().map_err(|e| {
        crate::shared::YtdlError::Other(format!("Failed to setup terminal: {}", e))
    })?;
    info!("Terminal initialized in raw mode");

    // Create application state wrapped in Arc<Mutex> for sharing with download task
    let app = Arc::new(Mutex::new(App::new()));
    let event_handler = EventHandler::default();

    // Main event loop
    let result = loop {
        // Render current state
        {
            let mut app_locked = app.lock().await;
            if let Err(e) = terminal.draw(|frame| render(&mut app_locked, frame)) {
                error!("Failed to render frame: {}", e);
                break Err(crate::shared::YtdlError::Other(format!(
                    "Render error: {}",
                    e
                )));
            }
        }

        // Handle events
        match event_handler.next() {
            Ok(event) => {
                if let Err(e) = handle_event(Arc::clone(&app), event).await {
                    error!("Error handling event: {}", e);
                    let mut app_locked = app.lock().await;
                    app_locked.go_to_error(
                        "Event Error".to_string(),
                        format!("{}", e),
                        vec!["Try restarting the application".to_string()],
                    );
                }
            }
            Err(e) => {
                error!("Error reading event: {}", e);
                break Err(crate::shared::YtdlError::Other(format!(
                    "Event error: {}",
                    e
                )));
            }
        }

        // Check if we should quit
        {
            let app_locked = app.lock().await;
            if app_locked.should_quit {
                info!("Application quit requested");
                break Ok(());
            }
        }
    };

    // Restore terminal
    restore_terminal(&mut terminal).map_err(|e| {
        crate::shared::YtdlError::Other(format!("Failed to restore terminal: {}", e))
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

async fn handle_event(app: Arc<Mutex<App>>, event: Event) -> Result<()> {
    match event {
        Event::Paste(text) => {
            // Handle pasted text - only in URL input state
            let current_state = {
                let app_locked = app.lock().await;
                app_locked.state.clone()
            };

            if let AppState::UrlInput { input, cursor_pos, .. } = &current_state {
                let mut new_input = input.clone();
                // Insert the pasted text at cursor position
                new_input.insert_str(*cursor_pos, &text);
                let new_cursor_pos = cursor_pos + text.len();

                let mut app_locked = app.lock().await;
                app_locked.update_input(new_input, new_cursor_pos);
            }
        }
        Event::Key(key) => {
            // Global quit key
            if is_quit_key(key) {
                let mut app_locked = app.lock().await;
                app_locked.quit();
                return Ok(());
            }

            // Global help key (h or ?) - but NOT in URL input where user might be typing
            if matches!(key.code, KeyCode::Char('h') | KeyCode::Char('?')) && key.modifiers.is_empty() {
                let mut app_locked = app.lock().await;
                // Don't open help if already in help, settings, or URL input
                if !matches!(app_locked.state, AppState::Help { .. } | AppState::Settings { .. } | AppState::UrlInput { .. }) {
                    app_locked.go_to_help();
                    return Ok(());
                }
            }

            // State-specific key handling
            let current_state = {
                let app_locked = app.lock().await;
                app_locked.state.clone()
            };

            match &current_state {
                AppState::Welcome => {
                    match key.code {
                        KeyCode::Enter => {
                            let mut app_locked = app.lock().await;
                            app_locked.go_to_url_input();
                        }
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            let mut app_locked = app.lock().await;
                            app_locked.go_to_settings(
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
                            let mut app_locked = app.lock().await;
                            app_locked.update_input(new_input, cursor_pos + 1);
                        }
                        KeyCode::Backspace => {
                            if *cursor_pos > 0 {
                                let mut new_input = input.clone();
                                new_input.remove(cursor_pos - 1);
                                let mut app_locked = app.lock().await;
                                app_locked.update_input(new_input, cursor_pos - 1);
                            }
                        }
                        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            let mut app_locked = app.lock().await;
                            app_locked.update_input(String::new(), 0);
                        }
                        KeyCode::Enter => {
                            // Validate and proceed
                            if input.contains("youtube.com") || input.contains("youtu.be") {
                                let mut app_locked = app.lock().await;
                                app_locked.start_fetching_info(input.clone());
                                drop(app_locked); // Release lock before async call
                                // Fetch real video info
                                fetch_video_info(&mut *app.lock().await, input.clone()).await;
                            }
                        }
                        KeyCode::Esc => {
                            let mut app_locked = app.lock().await;
                            app_locked.go_to_welcome();
                        }
                        _ => {}
                    }
                }
                AppState::FetchingInfo { .. } => {
                    if is_back_key(key) {
                        let mut app_locked = app.lock().await;
                        app_locked.go_to_url_input();
                    }
                }
                AppState::FormatSelection { url, .. } => {
                    match key.code {
                        KeyCode::Up => {
                            let mut app_locked = app.lock().await;
                            app_locked.select_previous_format();
                        }
                        KeyCode::Down => {
                            let mut app_locked = app.lock().await;
                            app_locked.select_next_format();
                        }
                        KeyCode::Enter => {
                            let url_clone = url.clone();
                            let selected_format = {
                                let mut app_locked = app.lock().await;
                                app_locked.start_download();

                                // Get format to determine if audio only
                                if let AppState::Downloading { format, .. } = &app_locked.state {
                                    format.format_id.clone()
                                } else {
                                    "best".to_string()
                                }
                            };

                            let audio_only = selected_format == "audio";

                            // Spawn download task
                            tokio::spawn(perform_download(Arc::clone(&app), url_clone, audio_only));
                        }
                        KeyCode::Char('a') | KeyCode::Char('A') => {
                            // Quick select audio only
                            let url_clone = url.clone();
                            {
                                let mut app_locked = app.lock().await;
                                app_locked.start_download();
                            }
                            // Spawn download task for audio
                            tokio::spawn(perform_download(Arc::clone(&app), url_clone, true));
                        }
                        KeyCode::Esc => {
                            let mut app_locked = app.lock().await;
                            app_locked.go_to_url_input();
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
                        let mut app_locked = app.lock().await;
                        app_locked.go_to_url_input();
                    }
                }
                AppState::Success { .. } => {
                    match key.code {
                        KeyCode::Char('n') | KeyCode::Char('N') => {
                            let mut app_locked = app.lock().await;
                            app_locked.go_to_url_input();
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
                            let mut app_locked = app.lock().await;
                            app_locked.go_to_url_input();
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            // Retry - go back to URL input
                            let mut app_locked = app.lock().await;
                            app_locked.go_to_url_input();
                        }
                        _ => {}
                    }
                }
                AppState::Help { .. } => {
                    if is_back_key(key) {
                        let mut app_locked = app.lock().await;
                        app_locked.back_from_overlay();
                    }
                }
                AppState::Settings { .. } => {
                    match key.code {
                        KeyCode::Up => {
                            let mut app_locked = app.lock().await;
                            app_locked.select_previous_setting();
                        }
                        KeyCode::Down => {
                            let mut app_locked = app.lock().await;
                            app_locked.select_next_setting();
                        }
                        KeyCode::Enter => {
                            // TODO: Edit selected setting
                            info!("Edit setting selected");
                        }
                        KeyCode::Esc => {
                            let mut app_locked = app.lock().await;
                            app_locked.back_from_overlay();
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
            let mut app_locked = app.lock().await;
            app_locked.tick();
        }
    }

    Ok(())
}

// Fetch real video information
async fn fetch_video_info(app: &mut App, url: String) {
    let config = Config::load_with_env_overrides();
    let downloader = Downloader::new(config.output_dir.clone(), config.quality.clone());

    match downloader.fetch_video_info(&url).await {
        Ok(metadata) => {
            // Convert VideoMetadata to display-friendly VideoInfo
            let video_info = metadata.to_display_info();

            // Convert formats to TUI FormatOptions
            let mut formats = vec![
                FormatOption {
                    label: "Best Quality".to_string(),
                    resolution: "Auto".to_string(),
                    file_size: "Best available".to_string(),
                    format_id: "best".to_string(),
                },
            ];

            // Get unique video formats sorted by resolution
            let mut video_formats: Vec<_> = metadata.formats.iter()
                .filter(|f| f.vcodec.as_ref().map(|v| v != "none").unwrap_or(false))
                .collect();

            video_formats.sort_by(|a, b| {
                b.resolution.as_ref()
                    .and_then(|r| r.split('x').nth(1))
                    .and_then(|h| h.parse::<u32>().ok())
                    .unwrap_or(0)
                    .cmp(&a.resolution.as_ref()
                        .and_then(|r| r.split('x').nth(1))
                        .and_then(|h| h.parse::<u32>().ok())
                        .unwrap_or(0))
            });

            for format in video_formats.iter().take(5) {
                let resolution = format.resolution.as_ref()
                    .map(|r| r.clone())
                    .unwrap_or_else(|| "Unknown".to_string());
                let file_size = format.filesize
                    .map(|s| format!("{:.1} MB", s as f64 / 1_000_000.0))
                    .unwrap_or_else(|| "Unknown".to_string());

                formats.push(FormatOption {
                    label: resolution.clone(),
                    resolution: resolution.clone(),
                    file_size,
                    format_id: format.format_id.clone(),
                });
            }

            // Add audio only option
            formats.push(FormatOption {
                label: "Audio Only (MP3)".to_string(),
                resolution: "N/A".to_string(),
                file_size: "~5-10 MB".to_string(),
                format_id: "audio".to_string(),
            });

            if let AppState::FetchingInfo { url: _ } = &app.state {
                app.show_format_selection(url.clone(), video_info, formats);
            }
        }
        Err(e) => {
            error!("Failed to fetch video info: {}", e);
            app.go_to_error(
                "Fetch Error".to_string(),
                format!("Failed to fetch video information: {}", e),
                vec![
                    "Check your internet connection".to_string(),
                    "Verify the URL is correct".to_string(),
                    "Try updating yt-dlp".to_string(),
                ],
            );
        }
    }
}

// Perform real download with progress updates
async fn perform_download(app: Arc<Mutex<App>>, url: String, audio_only: bool) {
    let config = Config::load_with_env_overrides();
    let downloader = Downloader::new(config.output_dir.clone(), config.quality.clone());

    let app_clone = Arc::clone(&app);
    let start_time = std::time::Instant::now();

    let result = downloader.download_with_progress(
        &url,
        audio_only,
        move |progress_info: DownloadProgressInfo| {
            let elapsed = start_time.elapsed().as_secs();
            let app_handle = Arc::clone(&app_clone);

            // Update the app state with real progress
            tokio::spawn(async move {
                let mut app_locked = app_handle.lock().await;
                if let AppState::Downloading { progress, .. } = &mut app_locked.state {
                    *progress = DownloadProgress {
                        percentage: progress_info.percentage,
                        downloaded_bytes: progress_info.downloaded_bytes,
                        total_bytes: progress_info.total_bytes,
                        speed: progress_info.speed,
                        eta: progress_info.eta,
                        elapsed,
                    };
                }
            });
        }
    ).await;

    let mut app_locked = app.lock().await;

    match result {
        Ok(file_path) => {
            let filename = file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();

            let file_size = tokio::fs::metadata(&file_path)
                .await
                .ok()
                .map(|m| {
                    let bytes = m.len();
                    if bytes > 1_000_000_000 {
                        format!("{:.2} GB", bytes as f64 / 1_000_000_000.0)
                    } else if bytes > 1_000_000 {
                        format!("{:.1} MB", bytes as f64 / 1_000_000.0)
                    } else if bytes > 1_000 {
                        format!("{:.1} KB", bytes as f64 / 1_000.0)
                    } else {
                        format!("{} bytes", bytes)
                    }
                })
                .unwrap_or_else(|| "Unknown".to_string());

            let duration = start_time.elapsed();
            let duration_str = if duration.as_secs() > 60 {
                format!("{} min {} sec", duration.as_secs() / 60, duration.as_secs() % 60)
            } else {
                format!("{} seconds", duration.as_secs())
            };

            let save_location = file_path.parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("./downloads"));

            let success_info = DownloadSuccess {
                filename,
                file_size,
                duration: duration_str,
                save_location,
            };

            app_locked.download_complete(success_info);
        }
        Err(e) => {
            error!("Download failed: {}", e);
            app_locked.go_to_error(
                "Download Error".to_string(),
                format!("Failed to download video: {}", e),
                vec![
                    "Check your internet connection".to_string(),
                    "Verify the video is still available".to_string(),
                    "Try a different quality or format".to_string(),
                ],
            );
        }
    }
}
