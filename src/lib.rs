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


pub use component::Bubble;
pub use config::AppConfig;


pub mod prelude {
    pub use crate::AppConfig;
    pub use crate::Bubble;
    pub use crate::data::*;
}
