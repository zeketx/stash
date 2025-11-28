use crate::error::{Result, YtdlError};
use crate::utils::validate_youtube_url;
use arboard::Clipboard;
use tracing::{debug, error, info, warn};

pub struct ClipboardManager {
    clipboard: Clipboard,
}

impl ClipboardManager {
    pub fn new() -> Result<Self> {
        let clipboard = Clipboard::new().map_err(|e| {
            error!("Failed to initialize clipboard: {}", e);
            YtdlError::Other(format!("Failed to initialize clipboard: {}", e))
        })?;

        Ok(Self { clipboard })
    }

    pub fn get_text(&mut self) -> Result<String> {
        self.clipboard.get_text().map_err(|e| {
            debug!("Failed to read clipboard: {}", e);
            YtdlError::Other(format!("Failed to read clipboard: {}", e))
        })
    }

    pub fn set_text(&mut self, text: &str) -> Result<()> {
        self.clipboard.set_text(text.to_string()).map_err(|e| {
            error!("Failed to write to clipboard: {}", e);
            YtdlError::Other(format!("Failed to write to clipboard: {}", e))
        })?;

        info!("Copied to clipboard");
        Ok(())
    }

    pub fn get_youtube_url(&mut self) -> Option<String> {
        match self.get_text() {
            Ok(text) => {
                let text = text.trim();
                if validate_youtube_url(text).is_ok() {
                    info!("Found YouTube URL in clipboard: {}", text);
                    Some(text.to_string())
                } else {
                    debug!("Clipboard does not contain a valid YouTube URL");
                    None
                }
            }
            Err(_) => None,
        }
    }

    pub fn has_youtube_url(&mut self) -> bool {
        self.get_youtube_url().is_some()
    }
}

pub struct ClipboardWatcher {
    clipboard_manager: ClipboardManager,
    last_content: String,
}

impl ClipboardWatcher {
    pub fn new() -> Result<Self> {
        let mut clipboard_manager = ClipboardManager::new()?;
        let last_content = clipboard_manager.get_text().unwrap_or_default();

        Ok(Self {
            clipboard_manager,
            last_content,
        })
    }

    pub fn check_for_new_url(&mut self) -> Option<String> {
        match self.clipboard_manager.get_text() {
            Ok(content) => {
                if content != self.last_content {
                    self.last_content = content.clone();

                    let content = content.trim();
                    if validate_youtube_url(content).is_ok() {
                        info!("New YouTube URL detected in clipboard: {}", content);
                        return Some(content.to_string());
                    }
                }
                None
            }
            Err(_) => None,
        }
    }

    pub async fn watch_loop<F>(&mut self, mut callback: F) -> Result<()>
    where
        F: FnMut(String) + Send,
    {
        info!("Starting clipboard watch mode");

        loop {
            if let Some(url) = self.check_for_new_url() {
                callback(url);
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }
}

pub fn get_clipboard_url() -> Option<String> {
    match ClipboardManager::new() {
        Ok(mut manager) => manager.get_youtube_url(),
        Err(e) => {
            warn!("Failed to access clipboard: {}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_manager_creation() {
        let result = ClipboardManager::new();
        assert!(result.is_ok());
    }
}
