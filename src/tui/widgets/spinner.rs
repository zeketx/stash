use std::time::{Duration, Instant};

/// Spinner widget for loading animations
#[derive(Debug, Clone)]
pub struct Spinner {
    frames: Vec<&'static str>,
    current_frame: usize,
    last_update: Instant,
    frame_duration: Duration,
}

impl Spinner {
    /// Create a new spinner with default braille frames
    pub fn new() -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            current_frame: 0,
            last_update: Instant::now(),
            frame_duration: Duration::from_millis(80),
        }
    }

    /// Create a spinner with dots
    pub fn dots() -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            current_frame: 0,
            last_update: Instant::now(),
            frame_duration: Duration::from_millis(80),
        }
    }

    /// Create a spinner with line
    pub fn line() -> Self {
        Self {
            frames: vec!["-", "\\", "|", "/"],
            current_frame: 0,
            last_update: Instant::now(),
            frame_duration: Duration::from_millis(100),
        }
    }

    /// Update the spinner and return whether it changed
    pub fn tick(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.frame_duration {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.last_update = now;
            true
        } else {
            false
        }
    }

    /// Get the current frame
    pub fn frame(&self) -> &str {
        self.frames[self.current_frame]
    }

    /// Reset the spinner to the first frame
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.last_update = Instant::now();
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

/// Blinking cursor for text input
#[derive(Debug, Clone)]
pub struct BlinkingCursor {
    visible: bool,
    last_blink: Instant,
    blink_interval: Duration,
}

impl BlinkingCursor {
    pub fn new() -> Self {
        Self {
            visible: true,
            last_blink: Instant::now(),
            blink_interval: Duration::from_millis(530),
        }
    }

    pub fn tick(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_blink) >= self.blink_interval {
            self.visible = !self.visible;
            self.last_blink = now;
            true
        } else {
            false
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn show(&mut self) {
        self.visible = true;
        self.last_blink = Instant::now();
    }

    pub fn reset(&mut self) {
        self.visible = true;
        self.last_blink = Instant::now();
    }
}

impl Default for BlinkingCursor {
    fn default() -> Self {
        Self::new()
    }
}

/// Success checkmark animation
#[derive(Debug, Clone)]
pub struct CheckmarkAnimation {
    frames: Vec<&'static str>,
    current_frame: usize,
    completed: bool,
    last_update: Instant,
    frame_duration: Duration,
}

impl CheckmarkAnimation {
    pub fn new() -> Self {
        Self {
            frames: vec!["", "✓", "✓", "✓"],
            current_frame: 0,
            completed: false,
            last_update: Instant::now(),
            frame_duration: Duration::from_millis(100),
        }
    }

    pub fn start(&mut self) {
        self.current_frame = 0;
        self.completed = false;
        self.last_update = Instant::now();
    }

    pub fn tick(&mut self) -> bool {
        if self.completed {
            return false;
        }

        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.frame_duration {
            if self.current_frame < self.frames.len() - 1 {
                self.current_frame += 1;
                self.last_update = now;
                true
            } else {
                self.completed = true;
                false
            }
        } else {
            false
        }
    }

    pub fn frame(&self) -> &str {
        self.frames[self.current_frame]
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

impl Default for CheckmarkAnimation {
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
