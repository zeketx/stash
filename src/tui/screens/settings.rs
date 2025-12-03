use crate::tui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

#[derive(Debug, Clone)]
pub struct SettingsState {
    pub output_dir: String,
    pub quality: String,
    pub concurrent_downloads: usize,
    pub audio_format: String,
    pub selected_index: usize,
}

impl SettingsState {
    pub fn new(output_dir: String, quality: String, concurrent_downloads: usize) -> Self {
        Self {
            output_dir,
            quality,
            concurrent_downloads,
            audio_format: "mp3".to_string(),
            selected_index: 0,
        }
    }
}

pub fn render_settings(
    frame: &mut Frame,
    theme: &Theme,
    settings: &SettingsState,
    selected_index: usize,
) {
    let area = frame.area();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(10),    // Settings list
            Constraint::Length(5),  // Instructions
        ])
        .split(area);

    // Title
    let title = Paragraph::new(vec![Line::from(vec![Span::styled(
        "⚙ Settings",
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

    // Settings items
    let items = vec![
        ListItem::new(Line::from(vec![
            Span::styled("Output Directory: ", Style::default().fg(theme.info)),
            Span::raw(&settings.output_dir),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("Default Quality: ", Style::default().fg(theme.info)),
            Span::raw(&settings.quality),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("Concurrent Downloads: ", Style::default().fg(theme.info)),
            Span::raw(settings.concurrent_downloads.to_string()),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("Audio Format: ", Style::default().fg(theme.info)),
            Span::raw(&settings.audio_format),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("Save Settings", Style::default().fg(theme.success).add_modifier(Modifier::BOLD)),
        ])),
    ];

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Settings")
                .border_style(Style::default().fg(theme.border)),
        )
        .highlight_style(
            Style::default()
                .fg(theme.highlight)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
        )
        .highlight_symbol("▶ ");

    let mut list_state = ListState::default();
    list_state.select(Some(selected_index));
    frame.render_stateful_widget(list, chunks[1], &mut list_state);

    // Instructions
    let instructions = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Navigate  "),
            Span::styled("Enter", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Edit  "),
            Span::styled("Esc", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            Span::raw(" - Back"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Note: ", Style::default().fg(theme.info).add_modifier(Modifier::BOLD)),
            Span::raw("Settings will be saved to config file when you select 'Save Settings'"),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border)),
    );
    frame.render_widget(instructions, chunks[2]);
}
