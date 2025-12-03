use crate::error::Result;
use crate::history::History;
use colored::Colorize;

pub async fn handle_history_command(
    history: &History,
    limit: usize,
    search: Option<String>,
    export: Option<std::path::PathBuf>,
) -> Result<()> {
    if let Some(export_path) = export {
        history.export_to_csv(&export_path)?;
        println!("{} Exported history to: {:?}", "✓".green().bold(), export_path);
        return Ok(());
    }

    let entries = if let Some(query) = search {
        history.search(&query)
    } else {
        history.get_recent(limit)
    };

    if entries.is_empty() {
        println!("\n{}", "No history entries found".yellow());
        return Ok(());
    }

    println!("\n{}", "Download History:".green().bold());
    println!("{}", "=".repeat(80));

    for entry in entries {
        println!("\nTitle: {}", entry.title);
        println!("URL: {}", entry.url);
        println!("Date: {}", entry.timestamp.format("%Y-%m-%d %H:%M:%S"));
        println!("Size: {} bytes", entry.file_size);
        println!("Quality: {} ({})", entry.quality, entry.format);
        println!("Path: {:?}", entry.file_path);
    }

    println!("\nTotal entries: {}", history.len());

    Ok(())
}

pub async fn handle_clear_history_command(history: &mut History, older_than: Option<i64>) -> Result<()> {
    if let Some(days) = older_than {
        history.clear_older_than(days);
        println!(
            "{} Cleared history entries older than {} days",
            "✓".green().bold(),
            days
        );
    } else {
        history.clear();
        println!("{} Cleared all history", "✓".green().bold());
    }

    history.save()?;
    Ok(())
}
