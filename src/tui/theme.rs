use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub color: Color,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            color: Color::Rgb(160, 160, 160),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}
