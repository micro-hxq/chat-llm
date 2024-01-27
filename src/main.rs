use std::io;

use human_panic::setup_panic;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use chat_llm::app::{App, AppResult};
use chat_llm::event::{Event, EventHandler};
use chat_llm::handler::handle_key_events;
use chat_llm::tui::Tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    setup_panic!();
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
