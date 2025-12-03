use crate::cli::config::Config;
use crate::shared::Result;
use crate::core::history::History;
use colored::Colorize;

pub async fn handle_config_command() -> Result<()> {
    let config = Config::load_with_env_overrides();

    println!("\n{}", "Current Configuration:".green().bold());
    println!("{}", "=".repeat(80));
    println!("Output Directory: {:?}", config.output_dir);
    println!("Quality: {}", config.quality);
    println!("Audio Only: {}", config.audio_only);
    println!("Log Level: {}", config.log_level);
    println!("File Logging: {}", config.enable_file_logging);
    println!("JSON Logging: {}", config.enable_json_logging);
    println!("Concurrent Downloads: {:?}", config.concurrent_downloads);
    println!("Skip Duplicates: {:?}", config.skip_duplicates);

    if let Some(path) = Config::get_default_config_path() {
        println!("\nDefault config path: {:?}", path);
    }

    if let Some(path) = History::get_history_file_path() {
        println!("History file path: {:?}", path);
    }

    Ok(())
}
