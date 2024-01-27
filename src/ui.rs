use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, BorderType, Paragraph},
};
use ratatui::prelude::*;

use crate::app::App;
use crate::util::Mode;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(3),
                Constraint::Length(3),
            ]
        )
        .split(frame.size());


    frame.render_widget(
        Paragraph::new("hello")
            .block(
                Block::default()
                    .title("Template")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
        chunks[0],
    );

    app.textarea.set_block(
        Block::default()
            .title("Input")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .fg(if app.mode == Mode::Normal { Color::Cyan } else { Color::Yellow })
            .bg(Color::Black),
    );
    set_mode(app);

    let widget = app.textarea.widget();
    frame.render_widget(widget, chunks[1]);
}

fn set_mode(app: &mut App) {
    match app.mode {
        Mode::Normal => set_normal_mode(app),
        Mode::Input => set_input_mode(app),
    }
}

fn set_normal_mode(app: &mut App) {
    app.mode = Mode::Normal;
    app.textarea.set_cursor_style(Style::default().fg(Color::Black));
}

fn set_input_mode(app: &mut App) {
    app.mode = Mode::Input;
    app.textarea.set_cursor_style(Style::default().bg(Color::Yellow));
}