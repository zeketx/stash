use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_help(frame: &mut Frame, theme: &Theme) {
    let area = frame.area();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(10),    // Content
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    // Title
    let title = Paragraph::new(vec![Line::from(vec![Span::styled(
        "Help & Keyboard Shortcuts",
        Style::default()
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD),
    )])])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border)),
    );
    frame.render_widget(title, chunks[0]);

    // Content area - split into two columns
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Global shortcuts
    let global_shortcuts = vec![
        Line::from(vec![
            Span::styled("Global Shortcuts", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("q", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Quit application"),
        ]),
        Line::from(vec![
            Span::styled("h or ?", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Show this help screen"),
        ]),
        Line::from(vec![
            Span::styled("  Note:", Style::default().fg(theme.secondary)),
            Span::raw(" Help key disabled in URL input to allow typing"),
        ]),
        Line::from(vec![
            Span::styled("Esc", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Go back/Cancel"),
        ]),
        Line::from(vec![
            Span::styled("Ctrl+C", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Interrupt/Quit"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Welcome Screen", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Enter", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Start new download"),
        ]),
        Line::from(vec![
            Span::styled("s", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Open settings"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("URL Input Screen", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Enter", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Fetch video info"),
        ]),
        Line::from(vec![
            Span::styled("Ctrl+U", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Clear input"),
        ]),
    ];

    let global_para = Paragraph::new(global_shortcuts)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(global_para, content_chunks[0]);

    // Context-specific shortcuts
    let context_shortcuts = vec![
        Line::from(vec![
            Span::styled("Format Selection", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Navigate formats"),
        ]),
        Line::from(vec![
            Span::styled("Enter", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Start download"),
        ]),
        Line::from(vec![
            Span::styled("a", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Quick select audio"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Success Screen", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("n", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - New download"),
        ]),
        Line::from(vec![
            Span::styled("o", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Open file"),
        ]),
        Line::from(vec![
            Span::styled("f", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Open folder"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Error Screen", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("r", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Retry download"),
        ]),
        Line::from(vec![
            Span::styled("n", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - New download"),
        ]),
    ];

    let context_para = Paragraph::new(context_shortcuts)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(context_para, content_chunks[1]);

    // Footer
    let footer = Paragraph::new(vec![Line::from(vec![
        Span::raw("Press "),
        Span::styled("Esc", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
        Span::raw(" to close this help screen"),
    ])])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border)),
    );
    frame.render_widget(footer, chunks[2]);
}
