use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge},
    Frame,
};

pub fn render_progress_bar(
    frame: &mut Frame,
    area: Rect,
    percentage: f64,
    color: Color,
    title: &str,
) {
    let label = format!("{:.1}%", percentage);
    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Rgb(160, 160, 160))),
        )
        .gauge_style(Style::default().fg(color))
        .ratio(percentage / 100.0)
        .label(label);

    frame.render_widget(gauge, area);
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_speed(bytes_per_sec: f64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;

    if bytes_per_sec >= MB {
        format!("{:.2} MB/s", bytes_per_sec / MB)
    } else if bytes_per_sec >= KB {
        format!("{:.2} KB/s", bytes_per_sec / KB)
    } else {
        format!("{:.0} B/s", bytes_per_sec)
    }
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
