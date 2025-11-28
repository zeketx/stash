use crate::downloader::Downloader;
use crate::error::{Result, YtdlError};
use regex::Regex;
use std::path::PathBuf;
use tracing::{debug, error, info};

#[derive(Debug, Clone)]
pub struct PlaylistInfo {
    pub id: String,
    pub title: String,
    pub uploader: Option<String>,
    pub video_count: usize,
    pub videos: Vec<PlaylistVideo>,
}

#[derive(Debug, Clone)]
pub struct PlaylistVideo {
    pub url: String,
    pub title: String,
    pub duration: Option<u64>,
    pub index: usize,
    pub selected: bool,
}

pub struct PlaylistDownloader {
    downloader: Downloader,
    output_folder: Option<PathBuf>,
}

impl PlaylistDownloader {
    pub fn new(output_dir: PathBuf, quality: String) -> Self {
        Self {
            downloader: Downloader::new(output_dir, quality),
            output_folder: None,
        }
    }

    pub fn with_folder(mut self, folder: String) -> Self {
        self.output_folder = Some(PathBuf::from(folder));
        self
    }

    pub fn is_playlist_url(url: &str) -> bool {
        let playlist_patterns = [
            r"youtube\.com/playlist\?list=",
            r"youtube\.com/watch\?.*list=",
            r"youtu\.be/.*\?list=",
        ];

        playlist_patterns.iter().any(|pattern| {
            Regex::new(pattern)
                .map(|re| re.is_match(url))
                .unwrap_or(false)
        })
    }

    pub async fn fetch_playlist_info(&self, url: &str) -> Result<PlaylistInfo> {
        info!("Fetching playlist information: {}", url);

        if !Self::is_playlist_url(url) {
            return Err(YtdlError::Other(
                "URL is not a valid playlist URL".to_string(),
            ));
        }

        let output = tokio::process::Command::new("yt-dlp")
            .arg("--flat-playlist")
            .arg("--dump-json")
            .arg(url)
            .output()
            .await
            .map_err(|e| {
                error!("Failed to execute yt-dlp: {}", e);
                YtdlError::Io(e)
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("yt-dlp failed: {}", stderr);
            return Err(YtdlError::YtdlpFailed(stderr.to_string()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut videos = Vec::new();
        let mut playlist_title = String::new();
        let mut playlist_id = String::new();
        let mut uploader = None;

        for (index, line) in stdout.lines().enumerate() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                if index == 0 {
                    if let Some(title) = json.get("playlist_title").and_then(|v| v.as_str()) {
                        playlist_title = title.to_string();
                    }
                    if let Some(id) = json.get("playlist_id").and_then(|v| v.as_str()) {
                        playlist_id = id.to_string();
                    }
                    if let Some(up) = json.get("uploader").and_then(|v| v.as_str()) {
                        uploader = Some(up.to_string());
                    }
                }

                let video_url = json
                    .get("url")
                    .or_else(|| json.get("webpage_url"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let video_id = json.get("id").and_then(|v| v.as_str());

                let final_url = match (video_url, video_id) {
                    (Some(url), _) => url,
                    (None, Some(id)) => format!("https://www.youtube.com/watch?v={}", id),
                    _ => continue,
                };

                let title = json
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();

                let duration = json.get("duration").and_then(|v| v.as_u64());

                videos.push(PlaylistVideo {
                    url: final_url,
                    title,
                    duration,
                    index: index + 1,
                    selected: true,
                });
            }
        }

        if videos.is_empty() {
            return Err(YtdlError::Other("No videos found in playlist".to_string()));
        }

        if playlist_title.is_empty() {
            playlist_title = format!("Playlist {}", playlist_id);
        }

        info!("Found {} videos in playlist: {}", videos.len(), playlist_title);

        Ok(PlaylistInfo {
            id: playlist_id,
            title: playlist_title,
            uploader,
            video_count: videos.len(),
            videos,
        })
    }

    pub async fn download_playlist(
        &self,
        playlist_info: &PlaylistInfo,
        audio_only: bool,
    ) -> Result<Vec<PathBuf>> {
        let selected_videos: Vec<&PlaylistVideo> = playlist_info
            .videos
            .iter()
            .filter(|v| v.selected)
            .collect();

        if selected_videos.is_empty() {
            return Err(YtdlError::Other("No videos selected for download".to_string()));
        }

        info!(
            "Downloading {} videos from playlist: {}",
            selected_videos.len(),
            playlist_info.title
        );

        let mut output_paths = Vec::new();

        let selected_count = selected_videos.len();

        for video in selected_videos {
            debug!("Downloading video {}: {}", video.index, video.title);

            match self.downloader.download(&video.url, audio_only).await {
                Ok(path) => {
                    info!("Downloaded: {:?}", path);
                    output_paths.push(path);
                }
                Err(e) => {
                    error!("Failed to download {}: {}", video.title, e);
                }
            }
        }

        info!("Playlist download complete: {} of {} successful", output_paths.len(), selected_count);

        Ok(output_paths)
    }

    pub async fn download_range(
        &self,
        url: &str,
        start: usize,
        end: usize,
        audio_only: bool,
    ) -> Result<Vec<PathBuf>> {
        let mut playlist_info = self.fetch_playlist_info(url).await?;

        for video in &mut playlist_info.videos {
            video.selected = video.index >= start && video.index <= end;
        }

        self.download_playlist(&playlist_info, audio_only).await
    }
}

pub fn extract_playlist_id(url: &str) -> Option<String> {
    let patterns = [
        r"[?&]list=([a-zA-Z0-9_-]+)",
        r"playlist\?list=([a-zA-Z0-9_-]+)",
    ];

    for pattern in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(url) {
                if let Some(id) = caps.get(1) {
                    return Some(id.as_str().to_string());
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_playlist_url() {
        assert!(PlaylistDownloader::is_playlist_url(
            "https://youtube.com/playlist?list=PLtest"
        ));
        assert!(PlaylistDownloader::is_playlist_url(
            "https://youtube.com/watch?v=test&list=PLtest"
        ));
        assert!(!PlaylistDownloader::is_playlist_url(
            "https://youtube.com/watch?v=test"
        ));
    }

    #[test]
    fn test_extract_playlist_id() {
        let id = extract_playlist_id("https://youtube.com/playlist?list=PLtest123");
        assert_eq!(id, Some("PLtest123".to_string()));

        let id = extract_playlist_id("https://youtube.com/watch?v=abc&list=PLtest456");
        assert_eq!(id, Some("PLtest456".to_string()));

        let id = extract_playlist_id("https://youtube.com/watch?v=abc");
        assert_eq!(id, None);
    }
}
