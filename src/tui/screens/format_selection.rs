use crate::tui::{app::{FormatOption, VideoInfo}, theme::Theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_format_selection(
    frame: &mut Frame,
    theme: &Theme,
    video_info: &VideoInfo,
    formats: &[FormatOption],
    selected_index: usize,
) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Select Format")
        .style(Style::default().fg(theme.color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // Video info
    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.color))
        .title(" Video Information ");

    let info_text = vec![
        Line::from(vec![
            Span::styled("Title: ", Style::default().fg(theme.color).add_modifier(Modifier::BOLD)),
            Span::raw(&video_info.title),
        ]),
        Line::from(vec![
            Span::styled("Uploader: ", Style::default().fg(theme.color).add_modifier(Modifier::BOLD)),
            Span::raw(&video_info.uploader),
        ]),
        Line::from(vec![
            Span::styled("Duration: ", Style::default().fg(theme.color).add_modifier(Modifier::BOLD)),
            Span::raw(&video_info.duration),
        ]),
    ];

    let info_para = Paragraph::new(info_text)
        .block(info_block)
        .style(Style::default().fg(theme.color));
    frame.render_widget(info_para, chunks[1]);

    // Format list
    let format_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.color))
        .title(" Available Formats ");

    let items: Vec<ListItem> = formats
        .iter()
        .enumerate()
        .map(|(i, format)| {
            let is_selected = i == selected_index;
            let style = if is_selected {
                Style::default()
                    .fg(theme.color)
                    .add_modifier(Modifier::BOLD)
                    .bg(ratatui::style::Color::Rgb(160, 160, 160))
            } else {
                Style::default().fg(theme.color)
            };

            let prefix = if is_selected { "▶ " } else { "  " };
            let line = Line::from(vec![
                Span::raw(prefix),
                Span::styled(&format.label, style),
                Span::raw("  "),
                Span::styled(&format.resolution, style),
                Span::raw("  "),
                Span::styled(&format.file_size, Style::default().fg(theme.color)),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(format_block);
    frame.render_widget(list, chunks[2]);

    // Help text
    let help_text = vec![Line::from(vec![
        Span::styled("[↑/↓] ", Style::default().fg(theme.color).add_modifier(Modifier::BOLD)),
        Span::raw("Navigate  "),
        Span::styled("[Enter] ", Style::default().fg(theme.color).add_modifier(Modifier::BOLD)),
        Span::raw("Select  "),
        Span::styled("[A] ", Style::default().fg(theme.color).add_modifier(Modifier::BOLD)),
        Span::raw("Audio  "),
        Span::styled("[Esc] ", Style::default().fg(theme.color).add_modifier(Modifier::BOLD)),
        Span::raw("Back"),
    ])];

    let help = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.color));
    frame.render_widget(help, chunks[3]);
}
