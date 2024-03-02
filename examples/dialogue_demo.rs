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
    state: DialogueState,
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
        let messages = vec![
            ChatMessage::new(Role::User, "Hello, World!".to_string()),
            ChatMessage::new(Role::Assistant, "Hi there!".to_string()),
            ChatMessage::new(Role::User, "How are you?".to_string()),
            ChatMessage::new(Role::Assistant, r#"
fn main() -> Result<()> {
    let mut app = App { running: true };
    app.run()?;
    Ok(())
}"#.to_string()),
            ChatMessage::new(Role::User, "Hello, World!".to_string()),
            ChatMessage::new(Role::Assistant, "Hi there!".to_string()),
            ChatMessage::new(Role::User, "How are you?".to_string()),
            ChatMessage::new(Role::Assistant, r#"fn main() -> Result<()> {
    let mut app = App { running: true };
    app.run()?;
    Ok(())
}"#.to_string()),
            ChatMessage::new(Role::User, "The difference between ordinary and extraordinary is that little extra.".to_string()),
        ];
        let ref messages = messages.iter().collect::<Vec<&ChatMessage>>();
        let dialogue = Dialogue::new(messages)
            .with_block(Block::default().borders(Borders::ALL).title("[ Dialogue ]")
                .title_alignment(Alignment::Center));
        frame.render_stateful_widget(dialogue, frame.size(), &mut self.state);
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => self.running = false,
                        event::KeyCode::Char('j') => self.state.next(),
                        event::KeyCode::Char('k') => self.state.prev(),
                        event::KeyCode::Char('g') => self.state.first(),
                        event::KeyCode::Char('G') => self.state.last(),
                        _ => {}
                    }
                }
            }
        }
        return Ok(());
    }
}

fn main() -> Result<()> {
    let mut app = App { running: true, state: DialogueState::new() };
    app.run()?;
    Ok(())
}
