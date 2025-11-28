use crate::ui::{app::DownloadHistory, theme::Theme};
use chrono::Local;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_url_input(
    frame: &mut Frame,
    theme: &Theme,
    input: &str,
    cursor_pos: usize,
    is_valid: Option<bool>,
    validation_message: &str,
    recent_downloads: &[DownloadHistory],
) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Paste YouTube URL")
        .style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // Input box
    let input_style = match is_valid {
        Some(true) => Style::default().fg(theme.success),
        Some(false) => Style::default().fg(theme.error),
        None => Style::default().fg(theme.foreground),
    };

    let border_style = match is_valid {
        Some(true) => Style::default().fg(theme.success),
        Some(false) => Style::default().fg(theme.error),
        None => Style::default().fg(theme.border),
    };

    let placeholder = if input.is_empty() {
        "https://www.youtube.com/watch?v=..."
    } else {
        ""
    };

    let display_text = if input.is_empty() {
        placeholder
    } else {
        input
    };

    let input_widget = Paragraph::new(display_text)
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(" URL "),
        );
    frame.render_widget(input_widget, chunks[1]);

    // Show cursor
    if !input.is_empty() || cursor_pos > 0 {
        let cursor_x = chunks[1].x + cursor_pos as u16 + 1;
        let cursor_y = chunks[1].y + 1;
        if cursor_x < chunks[1].x + chunks[1].width - 1 {
            frame.set_cursor_position((cursor_x, cursor_y));
        }
    }

    // Validation message
    let validation_style = match is_valid {
        Some(true) => Style::default().fg(theme.success),
        Some(false) => Style::default().fg(theme.error),
        None => Style::default().fg(theme.secondary),
    };

    let validation = Paragraph::new(validation_message)
        .style(validation_style)
        .alignment(Alignment::Center);
    frame.render_widget(validation, chunks[2]);

    // Recent downloads
    if !recent_downloads.is_empty() {
        let recent_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title(" Recent Downloads ");

        let items: Vec<ListItem> = recent_downloads
            .iter()
            .take(5)
            .map(|download| {
                let now = Local::now();
                let duration = now.signed_duration_since(download.timestamp);
                let time_str = if duration.num_minutes() < 60 {
                    format!("{} mins ago", duration.num_minutes())
                } else if duration.num_hours() < 24 {
                    format!("{} hours ago", duration.num_hours())
                } else {
                    format!("{} days ago", duration.num_days())
                };

                let line = Line::from(vec![
                    Span::styled(&download.title, Style::default().fg(theme.foreground)),
                    Span::raw("  "),
                    Span::styled(time_str, Style::default().fg(theme.secondary)),
                ]);
                ListItem::new(line)
            })
            .collect();

        let list = List::new(items).block(recent_block);
        frame.render_widget(list, chunks[3]);
    }

    // Help text
    let help_text = vec![Line::from(vec![
        Span::styled("[Enter] ", Style::default().fg(theme.success).add_modifier(Modifier::BOLD)),
        Span::raw("Continue  "),
        Span::styled("[Ctrl+U] ", Style::default().fg(theme.info).add_modifier(Modifier::BOLD)),
        Span::raw("Clear  "),
        Span::styled("[Esc] ", Style::default().fg(theme.error).add_modifier(Modifier::BOLD)),
        Span::raw("Back"),
    ])];

    let help = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.foreground));
    frame.render_widget(help, chunks[4]);
}
