use std::error;
use tui_textarea::TextArea;

use crate::config::AppConfig;
use crate::util::{Conversation, History, LlmStatus, Mode};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// counter
    // pub counter: u8,
    pub config: AppConfig,
    pub mode: Mode,
    pub status: LlmStatus,
    pub history: History,
    pub conversation: Conversation,

    pub textarea: TextArea<'a>,

}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let history = History::new();
        let count = history.len();
        Self {
            running: true,
            config: AppConfig::new(),
            mode: Mode::Normal,
            status: LlmStatus::Idle,
            history,
            conversation: Conversation::new(count as i64, "".to_string()),
            textarea: TextArea::default(),
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {}

    pub fn decrement_counter(&mut self) {}
}
