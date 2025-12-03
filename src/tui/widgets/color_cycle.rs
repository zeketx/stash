use ratatui::style::Color;

/// Color-cycling animation for the welcome banner
pub struct ColorCycle {
    frame: usize,
    colors: Vec<Color>,
}

impl ColorCycle {
    pub fn new() -> Self {
        Self {
            frame: 0,
            colors: vec![
                Color::Cyan,
                Color::Blue,
                Color::Magenta,
                Color::Red,
                Color::Yellow,
                Color::Green,
                Color::Cyan, // Loop back to start smoothly
            ],
        }
    }

    pub fn tick(&mut self) {
        self.frame = (self.frame + 1) % (self.colors.len() * 4); // Slower cycling
    }

    pub fn current_color(&self) -> Color {
        let index = self.frame / 4; // Divide to slow down color changes
        self.colors[index.min(self.colors.len() - 1)]
    }

    pub fn reset(&mut self) {
        self.frame = 0;
    }
}

impl Default for ColorCycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Pulsing selection animation
pub struct PulsingSelection {
    frame: usize,
    max_frames: usize,
}

impl PulsingSelection {
    pub fn new() -> Self {
        Self {
            frame: 0,
            max_frames: 20, // Full pulse cycle
        }
    }

    pub fn tick(&mut self) {
        self.frame = (self.frame + 1) % self.max_frames;
    }

    /// Returns true if the selection should be highlighted this frame
    pub fn is_highlighted(&self) -> bool {
        // Create a pulsing effect using sine-like pattern
        let half = self.max_frames / 2;
        if self.frame < half {
            self.frame % 3 != 0 // Show 2 out of 3 frames in first half
        } else {
            self.frame % 2 == 0 // Show 1 out of 2 frames in second half
        }
    }

    /// Returns intensity (0.0 to 1.0) for dimming effect
    pub fn intensity(&self) -> f32 {
        let normalized = self.frame as f32 / self.max_frames as f32;
        // Create smooth sine wave
        ((normalized * std::f32::consts::PI * 2.0).sin() + 1.0) / 2.0
    }

    pub fn reset(&mut self) {
        self.frame = 0;
    }
}

impl Default for PulsingSelection {
    fn default() -> Self {
        Self::new()
    }
}
