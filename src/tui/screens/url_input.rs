use crate::tui::{app::DownloadHistory, theme::Theme};
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
            Constraint::Length(2),
            Constraint::Length(5),
            Constraint::Length(2),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(area);

    // Conversational greeting
    let greeting = Paragraph::new("What would you like to download today?")
        .style(Style::default().fg(theme.foreground))
        .alignment(Alignment::Left);
    frame.render_widget(greeting, chunks[0]);

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
        "Paste a YouTube URL or press Ctrl+V"
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

    // Hint text / Validation message with spinner for "Fetching..."
    let (hint_text, hint_style) = if validation_message.contains("Fetching") {
        // Show spinner animation when fetching
        let spinner = get_spinner();
        (format!("{} {}", validation_message, spinner), Style::default().fg(theme.info))
    } else {
        match is_valid {
            Some(true) => (validation_message.to_string(), Style::default().fg(theme.success)),
            Some(false) => (validation_message.to_string(), Style::default().fg(theme.error)),
            None => ("Press Enter to continue or paste a URL to start".to_string(), Style::default().fg(theme.secondary)),
        }
    };

    let hint = Paragraph::new(hint_text)
        .style(hint_style)
        .alignment(Alignment::Left);
    frame.render_widget(hint, chunks[2]);

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

    // Footer help text
    let help_text = vec![Line::from(vec![
        Span::styled("[Enter] ", Style::default().fg(theme.success).add_modifier(Modifier::BOLD)),
        Span::raw("Continue  "),
        Span::styled("[Ctrl+U] ", Style::default().fg(theme.info).add_modifier(Modifier::BOLD)),
        Span::raw("Clear  │  "),
        Span::styled("[S] ", Style::default().fg(theme.info).add_modifier(Modifier::BOLD)),
        Span::raw("Settings  "),
        Span::styled("[H] ", Style::default().fg(theme.info).add_modifier(Modifier::BOLD)),
        Span::raw("Help  "),
        Span::styled("[Q] ", Style::default().fg(theme.error).add_modifier(Modifier::BOLD)),
        Span::raw("Quit"),
    ])];

    let help = Paragraph::new(help_text)
        .alignment(Alignment::Left)
        .style(Style::default().fg(theme.secondary));
    frame.render_widget(help, chunks[4]);
}

fn get_spinner() -> &'static str {
    use std::time::{SystemTime, UNIX_EPOCH};
    let spinners = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let index = (now / 80) % spinners.len() as u128;
    spinners[index as usize]
}
