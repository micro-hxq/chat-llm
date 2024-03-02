use std::time::Duration;

use color_eyre::Result;
use crossterm::event;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::*;

use chat_llm::prelude::*;
use tui::Tui;

mod tui;

struct App {
    running: bool,
}

impl App {
    fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;

        while self.running {
            tui.draw(|frame| self.ui(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn ui(&mut self, frame: &mut Frame) {
        let message = ChatMessage::new(Role::User, r#"
fn main() -> Result<()> {
    let mut app = App { running: true };
    app.run()?;
    Ok(())
}"#.to_string());
        let bubble = Bubble::new(&message).with_width(frame.size().width);
        frame.render_widget(bubble, frame.size());
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => self.running = false,
                    _ => {}
                }
            }
        }
        return Ok(());
    }
}

fn main() -> Result<()> {
    let mut app = App { running: true };
    app.run()?;
    Ok(())
}
