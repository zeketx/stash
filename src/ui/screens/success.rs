use crate::ui::{app::DownloadSuccess, theme::Theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_success(frame: &mut Frame, theme: &Theme, info: &DownloadSuccess) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(10),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Download Complete!")
        .style(Style::default().fg(theme.success).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // Success icon
    let icon_text = vec![Line::from(vec![Span::styled(
        "✓",
        Style::default()
            .fg(theme.success)
            .add_modifier(Modifier::BOLD),
    )])];

    let icon = Paragraph::new(icon_text).alignment(Alignment::Center);
    frame.render_widget(icon, chunks[1]);

    // File information
    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.success))
        .title(" File Information ");

    let info_text = vec![
        Line::from(vec![
            Span::styled(
                "Filename: ",
                Style::default()
                    .fg(theme.secondary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(&info.filename),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Size: ",
                Style::default()
                    .fg(theme.secondary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(&info.file_size),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Duration: ",
                Style::default()
                    .fg(theme.secondary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(&info.duration),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Location: ",
                Style::default()
                    .fg(theme.secondary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(info.save_location.to_string_lossy().to_string()),
        ]),
    ];

    let info_para = Paragraph::new(info_text)
        .block(info_block)
        .style(Style::default().fg(theme.foreground));
    frame.render_widget(info_para, chunks[2]);

    // Quick actions
    let actions_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title(" Quick Actions ");

    let actions_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[O] ",
                Style::default()
                    .fg(theme.info)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Open file"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[F] ",
                Style::default()
                    .fg(theme.info)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Open folder"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[N] ",
                Style::default()
                    .fg(theme.success)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("New download"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[Q] ",
                Style::default()
                    .fg(theme.error)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Quit"),
        ]),
    ];

    let actions_para = Paragraph::new(actions_text)
        .block(actions_block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.foreground));
    frame.render_widget(actions_para, chunks[3]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![Span::styled(
        "Press a key to continue",
        Style::default().fg(theme.secondary),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(footer, chunks[4]);
}
