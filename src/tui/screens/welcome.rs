use crate::tui::{theme::Theme, widgets::render_banner};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_welcome(frame: &mut Frame, theme: &Theme, banner_color: Color, pulse_intensity: f32) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    // Title with pulsing effect
    let title_color = if pulse_intensity > 0.7 {
        theme.success
    } else {
        theme.primary
    };
    let title = Paragraph::new("Welcome!")
        .style(Style::default().fg(title_color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // Main content
    let main_area = centered_rect(60, 70, chunks[1]);
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.background));

    frame.render_widget(main_block, main_area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(2),
            Constraint::Min(8),
        ])
        .split(main_area);

    // Banner with color-cycling animation
    let banner = render_banner(banner_color);
    frame.render_widget(banner, inner[0]);

    // Version
    let version = Paragraph::new(vec![Line::from(vec![Span::styled(
        "v0.1.0",
        Style::default().fg(theme.secondary),
    )])])
    .alignment(Alignment::Center);
    frame.render_widget(version, inner[1]);

    // Menu
    let menu_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("[Enter] ", Style::default().fg(theme.success).add_modifier(Modifier::BOLD)),
            Span::raw("Start Download"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("[S] ", Style::default().fg(theme.info).add_modifier(Modifier::BOLD)),
            Span::raw("Settings"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("[H] ", Style::default().fg(theme.info).add_modifier(Modifier::BOLD)),
            Span::raw("Help"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("[Q] ", Style::default().fg(theme.error).add_modifier(Modifier::BOLD)),
            Span::raw("Quit"),
        ]),
    ];

    let menu = Paragraph::new(menu_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.foreground));
    frame.render_widget(menu, inner[2]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![Span::styled(
        "Press any key to continue",
        Style::default().fg(theme.secondary),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(footer, chunks[2]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
