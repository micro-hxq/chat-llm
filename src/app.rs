use std::error;

use tui_textarea::{CursorMove, TextArea};

use crate::config::AppConfig;
use crate::data::{Conversation, History};
use crate::DialogueState;
use crate::util::{LlmStatus, Mode};

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

    pub dialog_state: DialogueState,
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
            dialog_state: DialogueState::new(),
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

    pub fn set_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn set_input_mode(&mut self) {
        self.mode = Mode::Input;
    }

    pub fn is_normal_mode(&self) -> bool {
        self.mode == Mode::Normal
    }

    pub fn is_input_mode(&self) -> bool {
        self.mode == Mode::Input
    }

    pub fn send_message(&mut self) {
        let text = self.textarea.lines().iter()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .next();
        match text {
            Some(_text) => {
                // send message to llm
                todo!()
            }
            None => {}
        }
        self.clear_input();
        self.status = LlmStatus::Busy;
    }

    fn clear_input(&mut self) {
        self.textarea.move_cursor(CursorMove::Head);
        self.textarea.delete_line_by_end();
    }
}
