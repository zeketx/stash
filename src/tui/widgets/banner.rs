use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub fn render_banner<'a>(color: Color) -> Paragraph<'a> {
    let banner = vec![
        Line::from(vec![Span::styled(
            r#"  ┬ ┬┌┬┐╔╦╗╦  "#,
            Style::default().fg(color),
        )]),
        Line::from(vec![Span::styled(
            r#"  └┬┘ │  ║║║  "#,
            Style::default().fg(color),
        )]),
        Line::from(vec![Span::styled(
            r#"   ┴  ┴ ═╩╝╩═╝"#,
            Style::default().fg(color),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "YouTube Downloader",
            Style::default().fg(Color::White),
        )]),
    ];

    Paragraph::new(banner).alignment(Alignment::Center)
}
