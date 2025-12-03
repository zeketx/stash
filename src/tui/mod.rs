//! Terminal User Interface layer
//!
//! This module contains the interactive TUI implementation
//! including screens, widgets, and event handling

pub mod app;
pub mod events;
pub mod runner;
pub mod screens;
pub mod terminal;
pub mod theme;
pub mod widgets;

pub use app::{App, AppState};
pub use events::{Event, EventHandler};
pub use runner::run_tui;
pub use terminal::{restore_terminal, setup_terminal};
pub use theme::Theme;
