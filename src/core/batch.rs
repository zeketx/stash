use crate::cli::Config;
use crate::core::{History, HistoryEntry};
use crate::infra::Downloader;
use crate::shared::{validate_youtube_url, Result, YtdlError};
use futures::stream::{self, StreamExt};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone, PartialEq)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Complete,
    Failed(String),
    Skipped,
}

#[derive(Debug, Clone)]
pub struct BatchDownloadItem {
    pub url: String,
    pub status: DownloadStatus,
    pub title: Option<String>,
    pub output_path: Option<PathBuf>,
    pub progress: f32,
    pub file_size: u64,
}

impl BatchDownloadItem {
    pub fn new(url: String) -> Self {
        Self {
            url,
            status: DownloadStatus::Pending,
            title: None,
            output_path: None,
            progress: 0.0,
            file_size: 0,
        }
    }
}

pub struct BatchDownloader {
    items: Arc<Mutex<Vec<BatchDownloadItem>>>,
    config: Config,
    history: Arc<Mutex<History>>,
    stop_on_error: bool,
    concurrent_limit: usize,
}

impl BatchDownloader {
    pub fn new(config: Config, history: History, stop_on_error: bool) -> Self {
        let concurrent_limit = config
            .concurrent_downloads
            .unwrap_or(3)
            .max(1)
            .min(10);

        Self {
            items: Arc::new(Mutex::new(Vec::new())),
            config,
            history: Arc::new(Mutex::new(history)),
            stop_on_error,
            concurrent_limit,
        }
    }

    pub async fn load_from_file(&mut self, path: &Path) -> Result<()> {
        info!("Loading batch URLs from file: {:?}", path);

        let content = tokio::fs::read_to_string(path).await.map_err(|e| {
            error!("Failed to read batch file: {}", e);
            YtdlError::Io(e)
        })?;

        let mut urls = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            match validate_youtube_url(line) {
                Ok(_) => {
                    urls.push(line.to_string());
                    debug!("Added URL from line {}: {}", line_num + 1, line);
                }
                Err(e) => {
                    warn!("Invalid URL on line {}: {} - {}", line_num + 1, line, e);
                }
            }
        }

        if urls.is_empty() {
            return Err(YtdlError::Config("No valid URLs found in batch file".to_string()));
        }

        let mut items = self.items.lock().await;
        for url in urls {
            items.push(BatchDownloadItem::new(url));
        }

        info!("Loaded {} URLs for batch download", items.len());
        Ok(())
    }

    pub async fn add_urls(&mut self, urls: Vec<String>) -> Result<()> {
        let mut items = self.items.lock().await;

        for url in urls {
            validate_youtube_url(&url)?;
            items.push(BatchDownloadItem::new(url));
        }

        info!("Added {} URLs to batch", items.len());
        Ok(())
    }

    pub async fn check_duplicates(&self) -> Vec<String> {
        let items = self.items.lock().await;
        let history = self.history.lock().await;

        let mut duplicates = Vec::new();
        for item in items.iter() {
            if history.contains_url(&item.url) {
                duplicates.push(item.url.clone());
            }
        }

        duplicates
    }

    pub async fn skip_duplicates(&self) {
        let mut items = self.items.lock().await;
        let history = self.history.lock().await;

        for item in items.iter_mut() {
            if history.contains_url(&item.url) {
                info!("Skipping duplicate URL: {}", item.url);
                item.status = DownloadStatus::Skipped;
            }
        }
    }

    pub async fn download_all(&self) -> Result<BatchDownloadStats> {
        let total_count = {
            let items = self.items.lock().await;
            items.len()
        };

        info!(
            "Starting batch download of {} items with concurrency limit {}",
            total_count, self.concurrent_limit
        );

        let indices: Vec<usize> = (0..total_count).collect();
        let stream = stream::iter(indices)
            .map(|index| {
                let items = Arc::clone(&self.items);
                let history = Arc::clone(&self.history);
                let config = self.config.clone();
                let stop_on_error = self.stop_on_error;

                async move {
                    Self::download_item(index, items, history, config, stop_on_error).await
                }
            })
            .buffer_unordered(self.concurrent_limit);

        let results: Vec<Result<()>> = stream.collect().await;

        let mut stats = BatchDownloadStats::default();
        stats.total = total_count;

        for result in results {
            match result {
                Ok(_) => stats.successful += 1,
                Err(_) => stats.failed += 1,
            }
        }

        let items = self.items.lock().await;
        stats.skipped = items
            .iter()
            .filter(|i| i.status == DownloadStatus::Skipped)
            .count();

        info!(
            "Batch download complete: {} successful, {} failed, {} skipped",
            stats.successful, stats.failed, stats.skipped
        );

        let history = self.history.lock().await;
        history.save()?;

        Ok(stats)
    }

    async fn download_item(
        index: usize,
        items: Arc<Mutex<Vec<BatchDownloadItem>>>,
        history: Arc<Mutex<History>>,
        config: Config,
        stop_on_error: bool,
    ) -> Result<()> {
        let url = {
            let items = items.lock().await;
            if items[index].status == DownloadStatus::Skipped {
                return Ok(());
            }
            items[index].url.clone()
        };

        {
            let mut items = items.lock().await;
            items[index].status = DownloadStatus::Downloading;
        }

        info!("Starting download {}: {}", index + 1, url);

        let downloader = Downloader::new(config.output_dir.clone(), config.quality.clone());

        let result = downloader.download(&url, config.audio_only).await;

        match result {
            Ok(output_path) => {
                info!("Download {} complete: {:?}", index + 1, output_path);

                let file_size = tokio::fs::metadata(&output_path)
                    .await
                    .map(|m| m.len())
                    .unwrap_or(0);

                let video_info = downloader.fetch_video_info(&url).await.ok();
                let title = video_info.as_ref().map(|v| v.title.clone()).unwrap_or_else(|| url.clone());

                {
                    let mut items = items.lock().await;
                    items[index].status = DownloadStatus::Complete;
                    items[index].output_path = Some(output_path.clone());
                    items[index].progress = 100.0;
                    items[index].file_size = file_size;
                    items[index].title = Some(title.clone());
                }

                let entry = HistoryEntry::new(
                    url.clone(),
                    title,
                    output_path,
                    file_size,
                    config.quality.clone(),
                    if config.audio_only { "mp3".to_string() } else { "mp4".to_string() },
                );

                let mut history = history.lock().await;
                history.add_entry(entry);

                Ok(())
            }
            Err(e) => {
                error!("Download {} failed: {}", index + 1, e);

                {
                    let mut items = items.lock().await;
                    items[index].status = DownloadStatus::Failed(e.to_string());
                }

                if stop_on_error {
                    return Err(e);
                }

                Ok(())
            }
        }
    }

    pub async fn get_items(&self) -> Vec<BatchDownloadItem> {
        let items = self.items.lock().await;
        items.clone()
    }

    pub async fn get_progress(&self) -> BatchProgress {
        let items = self.items.lock().await;

        let total = items.len();
        let complete = items
            .iter()
            .filter(|i| i.status == DownloadStatus::Complete)
            .count();
        let failed = items
            .iter()
            .filter(|i| matches!(i.status, DownloadStatus::Failed(_)))
            .count();
        let skipped = items
            .iter()
            .filter(|i| i.status == DownloadStatus::Skipped)
            .count();
        let downloading = items
            .iter()
            .filter(|i| i.status == DownloadStatus::Downloading)
            .count();

        let total_bytes: u64 = items.iter().map(|i| i.file_size).sum();

        BatchProgress {
            total,
            complete,
            failed,
            skipped,
            downloading,
            total_bytes,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct BatchDownloadStats {
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
    pub skipped: usize,
}

#[derive(Debug, Clone)]
pub struct BatchProgress {
    pub total: usize,
    pub complete: usize,
    pub failed: usize,
    pub skipped: usize,
    pub downloading: usize,
    pub total_bytes: u64,
}

impl BatchProgress {
    pub fn percentage(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            ((self.complete + self.failed + self.skipped) as f32 / self.total as f32) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_batch_download_item() {
        let item = BatchDownloadItem::new("https://youtube.com/watch?v=test".to_string());
        assert_eq!(item.status, DownloadStatus::Pending);
        assert_eq!(item.progress, 0.0);
    }

    #[tokio::test]
    async fn test_add_urls() {
        let config = Config::default();
        let history = History::new();
        let mut batch = BatchDownloader::new(config, history, false);

        let urls = vec!["https://youtube.com/watch?v=test".to_string()];
        batch.add_urls(urls).await.unwrap();

        let items = batch.get_items().await;
        assert_eq!(items.len(), 1);
    }
}
