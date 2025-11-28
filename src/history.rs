use crate::error::{Result, YtdlError};
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{debug, error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub file_path: PathBuf,
    pub file_size: u64,
    pub timestamp: DateTime<Utc>,
    pub quality: String,
    pub format: String,
}

impl HistoryEntry {
    pub fn new(
        url: String,
        title: String,
        file_path: PathBuf,
        file_size: u64,
        quality: String,
        format: String,
    ) -> Self {
        Self {
            url,
            title,
            file_path,
            file_size,
            timestamp: Utc::now(),
            quality,
            format,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct History {
    entries: Vec<HistoryEntry>,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_history_path()?;

        if !path.exists() {
            debug!("No history file found, creating new history");
            return Ok(Self::new());
        }

        let content = std::fs::read_to_string(&path).map_err(|e| {
            error!("Failed to read history file: {}", e);
            YtdlError::Io(e)
        })?;

        let history: History = serde_json::from_str(&content).map_err(|e| {
            error!("Failed to parse history file: {}", e);
            YtdlError::Other(format!("Failed to parse history: {}", e))
        })?;

        info!("Loaded {} history entries", history.entries.len());
        Ok(history)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_history_path()?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                error!("Failed to create history directory: {}", e);
                YtdlError::Io(e)
            })?;
        }

        let content = serde_json::to_string_pretty(self).map_err(|e| {
            error!("Failed to serialize history: {}", e);
            YtdlError::Other(format!("Failed to serialize history: {}", e))
        })?;

        std::fs::write(&path, content).map_err(|e| {
            error!("Failed to write history file: {}", e);
            YtdlError::Io(e)
        })?;

        debug!("Saved history with {} entries", self.entries.len());
        Ok(())
    }

    pub fn add_entry(&mut self, entry: HistoryEntry) {
        info!("Adding history entry: {}", entry.title);
        self.entries.push(entry);
    }

    pub fn contains_url(&self, url: &str) -> bool {
        self.entries.iter().any(|e| e.url == url)
    }

    pub fn get_entry_by_url(&self, url: &str) -> Option<&HistoryEntry> {
        self.entries.iter().find(|e| e.url == url)
    }

    pub fn get_recent(&self, limit: usize) -> Vec<&HistoryEntry> {
        let mut entries: Vec<&HistoryEntry> = self.entries.iter().collect();
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        entries.into_iter().take(limit).collect()
    }

    pub fn search(&self, query: &str) -> Vec<&HistoryEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.title.to_lowercase().contains(&query_lower)
                    || e.url.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn clear(&mut self) {
        info!("Clearing all history entries");
        self.entries.clear();
    }

    pub fn clear_older_than(&mut self, days: i64) {
        let cutoff = Utc::now() - chrono::Duration::days(days);
        let before_count = self.entries.len();
        self.entries.retain(|e| e.timestamp > cutoff);
        let removed = before_count - self.entries.len();
        info!("Removed {} entries older than {} days", removed, days);
    }

    pub fn export_to_csv(&self, path: &Path) -> Result<()> {
        let mut csv_content = String::from("URL,Title,File Path,File Size,Timestamp,Quality,Format\n");

        for entry in &self.entries {
            csv_content.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",{},\"{}\",\"{}\",\"{}\"\n",
                entry.url.replace("\"", "\"\""),
                entry.title.replace("\"", "\"\""),
                entry.file_path.display().to_string().replace("\"", "\"\""),
                entry.file_size,
                entry.timestamp.to_rfc3339(),
                entry.quality.replace("\"", "\"\""),
                entry.format.replace("\"", "\"\"")
            ));
        }

        std::fs::write(path, csv_content).map_err(|e| {
            error!("Failed to export history to CSV: {}", e);
            YtdlError::Io(e)
        })?;

        info!("Exported {} entries to CSV: {:?}", self.entries.len(), path);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn get_history_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("", "", "ytdl").ok_or_else(|| {
            YtdlError::Other("Failed to determine project directories".to_string())
        })?;

        Ok(proj_dirs.data_local_dir().join("history.json"))
    }

    pub fn get_history_file_path() -> Option<PathBuf> {
        Self::get_history_path().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_history() {
        let history = History::new();
        assert_eq!(history.len(), 0);
        assert!(history.is_empty());
    }

    #[test]
    fn test_add_entry() {
        let mut history = History::new();
        let entry = HistoryEntry::new(
            "https://youtube.com/watch?v=test".to_string(),
            "Test Video".to_string(),
            PathBuf::from("/tmp/test.mp4"),
            1024,
            "720p".to_string(),
            "mp4".to_string(),
        );

        history.add_entry(entry);
        assert_eq!(history.len(), 1);
        assert!(history.contains_url("https://youtube.com/watch?v=test"));
    }

    #[test]
    fn test_search() {
        let mut history = History::new();
        history.add_entry(HistoryEntry::new(
            "https://youtube.com/watch?v=1".to_string(),
            "Rust Tutorial".to_string(),
            PathBuf::from("/tmp/1.mp4"),
            1024,
            "720p".to_string(),
            "mp4".to_string(),
        ));
        history.add_entry(HistoryEntry::new(
            "https://youtube.com/watch?v=2".to_string(),
            "Python Guide".to_string(),
            PathBuf::from("/tmp/2.mp4"),
            2048,
            "1080p".to_string(),
            "mp4".to_string(),
        ));

        let results = history.search("rust");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Rust Tutorial");
    }
}
