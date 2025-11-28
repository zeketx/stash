use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub error: Color,
    pub warning: Color,
    pub info: Color,
    pub background: Color,
    pub foreground: Color,
    pub border: Color,
    pub highlight: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            success: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Cyan,
            background: Color::Black,
            foreground: Color::White,
            border: Color::DarkGray,
            highlight: Color::LightCyan,
        }
    }

    pub fn light() -> Self {
        Self {
            primary: Color::Blue,
            secondary: Color::DarkGray,
            success: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Blue,
            background: Color::White,
            foreground: Color::Black,
            border: Color::Gray,
            highlight: Color::LightBlue,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
