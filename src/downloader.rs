use crate::error::{Result, YtdlError};
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tracing::{debug, error, info, trace};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub uploader: String,
    pub duration: Option<u64>,
    pub view_count: Option<u64>,
    pub upload_date: Option<String>,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub formats: Vec<Format>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Format {
    pub format_id: String,
    pub ext: String,
    pub resolution: Option<String>,
    pub fps: Option<u32>,
    pub filesize: Option<u64>,
    pub vcodec: Option<String>,
    pub acodec: Option<String>,
}

pub struct Downloader {
    output_dir: PathBuf,
    quality: String,
}

impl Downloader {
    pub fn new(output_dir: PathBuf, quality: String) -> Self {
        Self {
            output_dir,
            quality,
        }
    }

    pub fn check_partial_download(&self, _url: &str) -> Option<PathBuf> {
        let entries = std::fs::read_dir(&self.output_dir).ok()?;

        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "part" {
                    debug!("Found partial download: {:?}", path);
                    return Some(path);
                }
            }
        }

        None
    }

    pub async fn resume_download(&self, url: &str, audio_only: bool) -> Result<PathBuf> {
        info!("Attempting to resume download for: {}", url);
        self.download_with_resume(url, audio_only, true).await
    }

    async fn download_with_resume(&self, url: &str, audio_only: bool, continue_download: bool) -> Result<PathBuf> {
        info!("Starting download: {} (audio_only: {}, resume: {})", url, audio_only, continue_download);

        std::fs::create_dir_all(&self.output_dir)?;

        let mut args = vec![
            "-o".to_string(),
            format!("{}/%(title)s.%(ext)s", self.output_dir.display()),
            "--progress".to_string(),
            "--newline".to_string(),
        ];

        if continue_download {
            args.push("--continue".to_string());
            info!("Resume mode enabled");
        }

        if audio_only {
            args.extend_from_slice(&[
                "-x".to_string(),
                "--audio-format".to_string(),
                "mp3".to_string(),
            ]);
            info!("Audio-only mode: converting to MP3");
        } else {
            if self.quality == "best" {
                args.push("-f".to_string());
                args.push("bestvideo+bestaudio/best".to_string());
            } else {
                args.push("-f".to_string());
                args.push(format!("bestvideo[height<={}]+bestaudio/best", self.quality));
            }
            info!("Video quality: {}", self.quality);
        }

        args.push(url.to_string());

        debug!("Executing yt-dlp with args: {:?}", args);

        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {percent}% {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );

        let mut child = TokioCommand::new("yt-dlp")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| YtdlError::YtdlpFailed(format!("Failed to spawn yt-dlp: {}", e)))?;

        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr = child.stderr.take().expect("Failed to capture stderr");

        let pb_clone = pb.clone();
        let stdout_handle = tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            let progress_re = Regex::new(r"\[download\]\s+(\d+\.?\d*)%").unwrap();
            let eta_re = Regex::new(r"ETA\s+([\d:]+)").unwrap();
            let speed_re = Regex::new(r"at\s+([\d.]+\w+/s)").unwrap();

            while let Ok(Some(line)) = lines.next_line().await {
                trace!("yt-dlp stdout: {}", line);

                if let Some(caps) = progress_re.captures(&line) {
                    if let Ok(percent) = caps[1].parse::<f64>() {
                        pb_clone.set_position(percent as u64);

                        let mut msg = format!("{:.1}%", percent);

                        if let Some(speed_caps) = speed_re.captures(&line) {
                            msg.push_str(&format!(" at {}", &speed_caps[1]));
                        }

                        if let Some(eta_caps) = eta_re.captures(&line) {
                            msg.push_str(&format!(" ETA {}", &eta_caps[1]));
                        }

                        pb_clone.set_message(msg);
                        debug!("Progress: {:.1}%", percent);
                    }
                }
            }
        });

        let stderr_handle = tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                if !line.is_empty() {
                    trace!("yt-dlp stderr: {}", line);
                }
            }
        });

        let status = child
            .wait()
            .await
            .map_err(|e| YtdlError::YtdlpFailed(format!("Failed to wait for yt-dlp: {}", e)))?;

        stdout_handle.await.ok();
        stderr_handle.await.ok();

        pb.finish_with_message("Download complete!");

        if !status.success() {
            error!("yt-dlp exited with status: {}", status);
            return Err(YtdlError::YtdlpFailed(format!(
                "yt-dlp exited with code {}",
                status.code().unwrap_or(-1)
            )));
        }

        info!("Download completed successfully");
        Ok(self.output_dir.clone())
    }

    pub async fn fetch_video_info(&self, url: &str) -> Result<VideoInfo> {
        info!("Fetching video information for: {}", url);

        let output = TokioCommand::new("yt-dlp")
            .args(&["--dump-json", "--no-playlist", url])
            .output()
            .await
            .map_err(|e| YtdlError::YtdlpFailed(format!("Failed to execute yt-dlp: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("yt-dlp failed: {}", stderr);
            return Err(YtdlError::YtdlpFailed(stderr.to_string()));
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        trace!("yt-dlp JSON output: {}", json_str);

        let json_value: serde_json::Value = serde_json::from_str(&json_str)?;

        let formats = if let Some(formats_array) = json_value["formats"].as_array() {
            formats_array
                .iter()
                .filter_map(|f| {
                    Some(Format {
                        format_id: f["format_id"].as_str()?.to_string(),
                        ext: f["ext"].as_str()?.to_string(),
                        resolution: f["resolution"].as_str().map(|s| s.to_string()),
                        fps: f["fps"].as_u64().map(|v| v as u32),
                        filesize: f["filesize"].as_u64(),
                        vcodec: f["vcodec"].as_str().map(|s| s.to_string()),
                        acodec: f["acodec"].as_str().map(|s| s.to_string()),
                    })
                })
                .collect()
        } else {
            Vec::new()
        };

        let info = VideoInfo {
            id: json_value["id"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
            title: json_value["title"]
                .as_str()
                .unwrap_or("Unknown Title")
                .to_string(),
            uploader: json_value["uploader"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string(),
            duration: json_value["duration"].as_u64(),
            view_count: json_value["view_count"].as_u64(),
            upload_date: json_value["upload_date"].as_str().map(|s| s.to_string()),
            description: json_value["description"].as_str().map(|s| s.to_string()),
            thumbnail: json_value["thumbnail"].as_str().map(|s| s.to_string()),
            formats,
        };

        debug!("Fetched video info: {:?}", info);
        Ok(info)
    }

    pub async fn download(&self, url: &str, audio_only: bool) -> Result<PathBuf> {
        self.download_with_resume(url, audio_only, false).await
    }

    pub fn list_formats(&self, info: &VideoInfo) {
        println!("\nAvailable formats for: {}", info.title);
        println!("{}", "=".repeat(80));
        println!(
            "{:<10} {:<8} {:<15} {:<8} {:<15} {:<15}",
            "Format ID", "Ext", "Resolution", "FPS", "Video Codec", "Audio Codec"
        );
        println!("{}", "-".repeat(80));

        for format in &info.formats {
            println!(
                "{:<10} {:<8} {:<15} {:<8} {:<15} {:<15}",
                format.format_id,
                format.ext,
                format.resolution.as_deref().unwrap_or("N/A"),
                format.fps.map(|f| f.to_string()).unwrap_or_else(|| "N/A".to_string()),
                format.vcodec.as_deref().unwrap_or("N/A"),
                format.acodec.as_deref().unwrap_or("N/A"),
            );
        }
    }
}
