use crate::cli::config::Config;
use crate::infra::downloader::Downloader;
use crate::shared::{constants::*, Result};
use colored::Colorize;

pub async fn show_video_info(url: &str, config: &Config) -> Result<()> {
    let downloader = Downloader::new(config.output_dir.clone(), config.quality.clone());

    println!("\n{}", "Fetching video information...".green().bold());
    let video_info = downloader.fetch_video_info(url).await?;

    println!("\n{}", "Video Information:".green().bold());
    println!("{}", SEPARATOR_LINE.repeat(SEPARATOR_WIDTH));
    println!("Title: {}", video_info.title);
    println!("Uploader: {}", video_info.uploader);

    if let Some(duration) = video_info.duration {
        let hours = duration / 3600;
        let minutes = (duration % 3600) / 60;
        let seconds = duration % 60;
        if hours > 0 {
            println!("Duration: {}:{:02}:{:02}", hours, minutes, seconds);
        } else {
            println!("Duration: {}:{:02}", minutes, seconds);
        }
    }

    if let Some(view_count) = video_info.view_count {
        println!("Views: {}", view_count.to_string().as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<std::result::Result<Vec<&str>, _>>()
            .unwrap()
            .join(","));
    }

    if let Some(upload_date) = video_info.upload_date {
        println!("Upload Date: {}", upload_date);
    }

    if let Some(description) = video_info.description {
        let desc_preview = if description.len() > 200 {
            format!("{}...", &description[..200])
        } else {
            description
        };
        println!("\nDescription:");
        println!("{}", desc_preview);
    }

    println!("\n{}", "Available Formats:".cyan().bold());
    println!("{}", SUBSEPARATOR_LINE.repeat(SEPARATOR_WIDTH));

    let mut video_formats: Vec<_> = video_info.formats.iter()
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

    for (i, format) in video_formats.iter().take(10).enumerate() {
        let res = format.resolution.as_ref().map(|s| s.as_str()).unwrap_or("unknown");
        let fps = format.fps.map(|f| format!("{}fps", f)).unwrap_or_else(|| "".to_string());
        let size = format.filesize.map(|s| format!("{:.1} MB", s as f64 / 1_000_000.0))
            .unwrap_or_else(|| "unknown size".to_string());
        println!("  {}. {} {} - {} ({})", i + 1, res, fps, size, format.ext);
    }

    println!("\n{}", "To download, run:".yellow().bold());
    println!("  ytdl \"{}\"", url);

    Ok(())
}
