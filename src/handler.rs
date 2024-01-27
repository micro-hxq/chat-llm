use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, AppResult};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if app.is_normal_mode() {
        handle_normal_mode_key_event(key_event, app)?;
    } else {
        handle_input_mode_key_event(key_event, app)?;
    }
    Ok(())
}


fn handle_normal_mode_key_event(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char('i') => {
            app.set_input_mode();
        }
        _ => {}
    }
    Ok(())
}

fn handle_input_mode_key_event(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc => {
            app.set_normal_mode();
        }
        KeyCode::Enter => {
            app.set_normal_mode();
            app.send_message()
        }
        _ => {
            app.textarea.input(key_event);
        }
    }
    Ok(())
}