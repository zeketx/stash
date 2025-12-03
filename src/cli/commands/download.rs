use crate::cli::config::Config;
use crate::infra::downloader::Downloader;
use crate::shared::Result;
use crate::core::history::{History, HistoryEntry};
use colored::Colorize;
use tracing::{info, warn};

pub async fn download_single_url(url: &str, config: &Config, history: &mut History, resume: bool) -> Result<()> {
    let downloader = Downloader::new(config.output_dir.clone(), config.quality.clone());

    if history.contains_url(url) {
        warn!("URL already downloaded");
        if let Some(entry) = history.get_entry_by_url(url) {
            println!("\n{} This URL was already downloaded:", "⚠".yellow().bold());
            println!("  Title: {}", entry.title);
            println!("  Date: {}", entry.timestamp.format("%Y-%m-%d %H:%M:%S"));
            println!("  Path: {:?}", entry.file_path);
        }

        if !resume {
            println!("\nUse --resume to re-download");
            return Ok(());
        }
    }

    println!("\n{}", "Starting download...".green().bold());

    let output_path = if resume {
        downloader.resume_download(url, config.audio_only).await?
    } else {
        downloader.download(url, config.audio_only).await?
    };

    println!(
        "\n{} Downloaded to: {}",
        "✓".green().bold(),
        output_path.display()
    );

    let video_info = downloader.fetch_video_info(url).await.ok();
    let title = video_info
        .as_ref()
        .map(|v| v.title.clone())
        .unwrap_or_else(|| url.to_string());

    let file_size = tokio::fs::metadata(&output_path)
        .await
        .map(|m| m.len())
        .unwrap_or(0);

    let entry = HistoryEntry::new(
        url.to_string(),
        title,
        output_path,
        file_size,
        config.quality.clone(),
        if config.audio_only {
            "mp3".to_string()
        } else {
            "mp4".to_string()
        },
    );

    history.add_entry(entry);
    history.save()?;

    Ok(())
}
