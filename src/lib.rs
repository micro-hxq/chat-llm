/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

mod util;

mod config;

mod component;
mod data;
mod llm;


pub use component::Bubble;
pub use component::Dialogue;
pub use component::DialogueState;
pub use config::AppConfig;


pub mod prelude {
    pub use crate::AppConfig;
    pub use crate::Bubble;
    pub use crate::Dialogue;
    pub use crate::DialogueState;
    pub use crate::data::*;
}
