use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Paste(String),
    Resize(u16, u16),
    Tick,
}

pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_rate_ms),
        }
    }

    pub fn next(&self) -> Result<Event> {
        if event::poll(self.tick_rate)? {
            match event::read()? {
                CrosstermEvent::Key(key) => Ok(Event::Key(key)),
                CrosstermEvent::Paste(text) => Ok(Event::Paste(text)),
                CrosstermEvent::Resize(w, h) => Ok(Event::Resize(w, h)),
                _ => Ok(Event::Tick),
            }
        } else {
            Ok(Event::Tick)
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new(16) // ~60 FPS
    }
}

pub fn is_quit_key(key: KeyEvent) -> bool {
    matches!(
        (key.code, key.modifiers),
        (KeyCode::Char('q'), KeyModifiers::NONE)
            | (KeyCode::Char('Q'), KeyModifiers::NONE)
            | (KeyCode::Char('c'), KeyModifiers::CONTROL)
    )
}

pub fn is_back_key(key: KeyEvent) -> bool {
    matches!(key.code, KeyCode::Esc)
}
