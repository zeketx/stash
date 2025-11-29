use notify_rust::{Notification, Timeout};
use tracing::{error, info};

/// Send a desktop notification for successful download
pub fn notify_download_complete(title: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        info!("Sending download complete notification for: {}", title);
        Notification::new()
            .summary("Download Complete")
            .body(&format!("{}\nSaved to: {}", title, file_path))
            .timeout(Timeout::Milliseconds(5000))
            .show()
            .map_err(|e| {
                error!("Failed to send notification: {}", e);
                e
            })?;
    }

    #[cfg(not(target_os = "macos"))]
    {
        // Notifications for other platforms (Linux, Windows)
        info!("Sending download complete notification for: {}", title);
        Notification::new()
            .summary("Download Complete")
            .body(&format!("{}\nSaved to: {}", title, file_path))
            .timeout(Timeout::Milliseconds(5000))
            .show()
            .map_err(|e| {
                error!("Failed to send notification: {}", e);
                e
            })?;
    }

    Ok(())
}

/// Send a desktop notification for download error
pub fn notify_download_error(title: &str, error: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        info!("Sending download error notification for: {}", title);
        Notification::new()
            .summary("Download Failed")
            .body(&format!("{}\nError: {}", title, error))
            .timeout(Timeout::Milliseconds(5000))
            .show()
            .map_err(|e| {
                error!("Failed to send notification: {}", e);
                e
            })?;
    }

    #[cfg(not(target_os = "macos"))]
    {
        info!("Sending download error notification for: {}", title);
        Notification::new()
            .summary("Download Failed")
            .body(&format!("{}\nError: {}", title, error))
            .timeout(Timeout::Milliseconds(5000))
            .show()
            .map_err(|e| {
                error!("Failed to send notification: {}", e);
                e
            })?;
    }

    Ok(())
}

/// Check if notifications are available on this platform
pub fn are_notifications_available() -> bool {
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    {
        true
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        false
    }
}
