use crate::config::Config;
use crate::error::{Result, YtdlError};
use crate::playlist::PlaylistDownloader;
use colored::Colorize;
use tracing::info;

pub async fn handle_playlist_download(
    url: &str,
    config: &Config,
    range: Option<&str>,
    folder: Option<&str>,
) -> Result<()> {
    let mut playlist_downloader = PlaylistDownloader::new(config.output_dir.clone(), config.quality.clone());

    if let Some(folder_name) = folder {
        playlist_downloader = playlist_downloader.with_folder(folder_name.to_string());
    }

    if let Some(range_str) = range {
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() == 2 {
            let start: usize = parts[0].parse().map_err(|_| {
                YtdlError::Config("Invalid range format".to_string())
            })?;
            let end: usize = parts[1].parse().map_err(|_| {
                YtdlError::Config("Invalid range format".to_string())
            })?;

            println!("\n{}", "Downloading playlist videos...".green().bold());
            let paths = playlist_downloader
                .download_range(url, start, end, config.audio_only)
                .await?;

            println!(
                "\n{} Downloaded {} videos from playlist",
                "✓".green().bold(),
                paths.len()
            );
        } else {
            return Err(YtdlError::Config(
                "Invalid range format. Use: --range 1-10".to_string(),
            ));
        }
    } else {
        let playlist_info = playlist_downloader.fetch_playlist_info(url).await?;

        println!("\n{}", "Playlist Information:".green().bold());
        println!("{}", "=".repeat(80));
        println!("Title: {}", playlist_info.title);
        if let Some(ref uploader) = playlist_info.uploader {
            println!("Uploader: {}", uploader);
        }
        println!("Videos: {}", playlist_info.video_count);

        println!("\n{}", "Downloading all videos...".green().bold());
        let paths = playlist_downloader
            .download_playlist(&playlist_info, config.audio_only)
            .await?;

        println!(
            "\n{} Downloaded {} of {} videos",
            "✓".green().bold(),
            paths.len(),
            playlist_info.video_count
        );
    }

    Ok(())
}
