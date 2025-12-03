use crate::tui::{
    app::{DownloadProgress, FormatOption, VideoInfo},
    theme::Theme,
    widgets::progress_bar::{format_bytes, format_duration, format_speed},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

pub fn render_downloading(
    frame: &mut Frame,
    theme: &Theme,
    video_info: &VideoInfo,
    format: &FormatOption,
    progress: &DownloadProgress,
) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Downloading")
        .style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // Video title
    let video_title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(" Video ");

    let video_title_text = vec![Line::from(vec![Span::raw(&video_info.title)])];

    let video_title_para = Paragraph::new(video_title_text)
        .block(video_title_block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.foreground));
    frame.render_widget(video_title_para, chunks[1]);

    // Progress bar
    let progress_color = if progress.percentage >= 100.0 {
        theme.success
    } else {
        theme.info
    };

    let label = format!("{:.1}%", progress.percentage);
    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Progress ")
                .border_style(Style::default().fg(theme.border)),
        )
        .gauge_style(Style::default().fg(progress_color))
        .ratio(progress.percentage / 100.0)
        .label(label);

    frame.render_widget(gauge, chunks[2]);

    // Statistics
    let stats_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(" Statistics ");

    let downloaded = format_bytes(progress.downloaded_bytes);
    let total = format_bytes(progress.total_bytes);
    let speed = format_speed(progress.speed);
    let elapsed = format_duration(progress.elapsed);
    let eta = progress
        .eta
        .map(format_duration)
        .unwrap_or_else(|| "calculating...".to_string());

    let stats_text = vec![
        Line::from(vec![
            Span::styled("Downloaded: ", Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
            Span::raw(format!("{} / {}", downloaded, total)),
        ]),
        Line::from(vec![
            Span::styled("Speed: ", Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
            Span::raw(speed),
        ]),
        Line::from(vec![
            Span::styled("Elapsed: ", Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
            Span::raw(elapsed),
        ]),
        Line::from(vec![
            Span::styled("ETA: ", Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
            Span::raw(eta),
        ]),
    ];

    let stats_para = Paragraph::new(stats_text)
        .block(stats_block)
        .style(Style::default().fg(theme.foreground));
    frame.render_widget(stats_para, chunks[3]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("[Ctrl+C] ", Style::default().fg(theme.error).add_modifier(Modifier::BOLD)),
        Span::raw("Cancel Download"),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(footer, chunks[4]);
}
