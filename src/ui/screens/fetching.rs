use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn render_fetching(frame: &mut Frame, theme: &Theme, url: &str) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Fetching Video Information")
        .style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // Main content
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.background));

    let inner = main_block.inner(chunks[1]);
    frame.render_widget(main_block, chunks[1]);

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(inner);

    // Spinner animation
    let spinner = get_spinner();
    let spinner_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            spinner,
            Style::default().fg(theme.info).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    let spinner_widget = Paragraph::new(spinner_text).alignment(Alignment::Center);
    frame.render_widget(spinner_widget, content_chunks[0]);

    // URL display
    let url_text = vec![
        Line::from(vec![
            Span::styled("URL: ", Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
            Span::raw(url),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Please wait...",
            Style::default().fg(theme.secondary),
        )]),
    ];

    let url_widget = Paragraph::new(url_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.foreground));
    frame.render_widget(url_widget, content_chunks[1]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("[Esc] ", Style::default().fg(theme.error).add_modifier(Modifier::BOLD)),
        Span::raw("Cancel"),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(footer, chunks[2]);
}

fn get_spinner() -> &'static str {
    let spinners = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let index = (now / 80) % spinners.len() as u128;
    spinners[index as usize]
}
