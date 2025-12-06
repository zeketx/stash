use crate::tui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_error(
    frame: &mut Frame,
    theme: &Theme,
    error_type: &str,
    message: &str,
    suggestions: &[String],
) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(6),
            Constraint::Min(5),
            Constraint::Length(7),
            Constraint::Length(3),
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Error Occurred")
        .style(Style::default().fg(theme.color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // Error icon
    let icon_text = vec![Line::from(vec![Span::styled(
        "✗",
        Style::default()
            .fg(theme.color)
            .add_modifier(Modifier::BOLD),
    )])];

    let icon = Paragraph::new(icon_text).alignment(Alignment::Center);
    frame.render_widget(icon, chunks[1]);

    // Error details
    let error_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.color))
        .title(" Error Details ");

    let error_text = vec![
        Line::from(vec![
            Span::styled(
                "Type: ",
                Style::default()
                    .fg(theme.color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(error_type, Style::default().fg(theme.color)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Message: ",
                Style::default()
                    .fg(theme.color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(message),
        ]),
    ];

    let error_para = Paragraph::new(error_text)
        .block(error_block)
        .style(Style::default().fg(theme.color));
    frame.render_widget(error_para, chunks[2]);

    // Suggestions
    if !suggestions.is_empty() {
        let suggestions_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.color))
            .title(" Troubleshooting Suggestions ");

        let items: Vec<ListItem> = suggestions
            .iter()
            .map(|suggestion| {
                let line = Line::from(vec![
                    Span::styled("• ", Style::default().fg(theme.color)),
                    Span::raw(suggestion),
                ]);
                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(suggestions_block)
            .style(Style::default().fg(theme.color));
        frame.render_widget(list, chunks[3]);
    }

    // Recovery actions
    let actions_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.color))
        .title(" Recovery Actions ");

    let actions_text = vec![
        Line::from(vec![
            Span::styled(
                "[R] ",
                Style::default()
                    .fg(theme.color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Retry download"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[N] ",
                Style::default()
                    .fg(theme.color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("New download"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[Q] ",
                Style::default()
                    .fg(theme.color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Quit"),
        ]),
    ];

    let actions_para = Paragraph::new(actions_text)
        .block(actions_block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.color));
    frame.render_widget(actions_para, chunks[4]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![Span::styled(
        "Press a key to continue",
        Style::default().fg(theme.color),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(footer, chunks[5]);
}
