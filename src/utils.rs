use crate::error::{Result, YtdlError};
use regex::Regex;
use std::process::Command;
use tracing::{debug, info};

pub fn validate_youtube_url(url: &str) -> Result<()> {
    let youtube_patterns = [
        r"^https?://(www\.)?youtube\.com/watch\?v=[\w-]+",
        r"^https?://(www\.)?youtu\.be/[\w-]+",
        r"^https?://(www\.)?youtube\.com/playlist\?list=[\w-]+",
        r"^https?://(www\.)?youtube\.com/shorts/[\w-]+",
    ];

    for pattern in &youtube_patterns {
        let re = Regex::new(pattern).unwrap();
        if re.is_match(url) {
            debug!("URL validated successfully: {}", url);
            return Ok(());
        }
    }

    Err(YtdlError::InvalidUrl(format!(
        "Invalid YouTube URL. Expected formats:\n  \
         - https://youtube.com/watch?v=VIDEO_ID\n  \
         - https://youtu.be/VIDEO_ID\n  \
         - https://youtube.com/playlist?list=PLAYLIST_ID\n  \
         - https://youtube.com/shorts/VIDEO_ID\n\
         Got: {}",
        url
    )))
}

pub fn check_command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn check_ytdlp() -> Result<String> {
    if !check_command_exists("yt-dlp") {
        return Err(YtdlError::YtdlpNotFound);
    }

    let output = Command::new("yt-dlp")
        .arg("--version")
        .output()
        .map_err(|e| YtdlError::YtdlpFailed(format!("Failed to get version: {}", e)))?;

    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    info!("yt-dlp version: {}", version);

    Ok(version)
}

pub fn check_ffmpeg() -> Result<String> {
    if !check_command_exists("ffmpeg") {
        return Err(YtdlError::FfmpegNotFound);
    }

    let output = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map_err(|e| YtdlError::Other(format!("Failed to get ffmpeg version: {}", e)))?;

    let version_output = String::from_utf8_lossy(&output.stdout);
    let version = version_output
        .lines()
        .next()
        .unwrap_or("unknown")
        .to_string();

    info!("ffmpeg version: {}", version);
    Ok(version)
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_youtube_url() {
        assert!(validate_youtube_url("https://youtube.com/watch?v=dQw4w9WgXcQ").is_ok());
        assert!(validate_youtube_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ").is_ok());
        assert!(validate_youtube_url("https://youtu.be/dQw4w9WgXcQ").is_ok());
        assert!(validate_youtube_url("https://youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf").is_ok());
        assert!(validate_youtube_url("https://youtube.com/shorts/abc123def").is_ok());
        assert!(validate_youtube_url("https://example.com/video").is_err());
        assert!(validate_youtube_url("not a url").is_err());
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0.00 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1_048_576), "1.00 MB");
        assert_eq!(format_bytes(1_073_741_824), "1.00 GB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0), "00:00");
        assert_eq!(format_duration(65), "01:05");
        assert_eq!(format_duration(3661), "01:01:01");
    }
}
