use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::component::util;
use crate::data::ChatMessage;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Icon<'a> {
    name: String,
    block: Option<Block<'a>>,
}

impl Icon<'_> {
    fn width(&self) -> u16 {
        self.name.len() as u16 + 2
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Bubble<'a> {
    message: &'a ChatMessage,
    block: Option<Block<'a>>,
    icon: Icon<'a>,
    width: u16,
    height: u16,
    lines: Vec<Line<'a>>,
}

impl<'a> Bubble<'a> {
    pub fn new(message: &'a ChatMessage, width: usize) -> Self {
        let icon = Icon {
            name: if message.is_user() { "You" } else { "Bot" }.to_owned(),
            block: None,
        };
        let lines = util::split_message(&message.content, width - 4 - icon.width() as usize);
        let width = width.min(lines.iter().map(|line| line.width()).max().unwrap_or(0) + 4);
        let height = lines.len() + 2;
        Self {
            message,
            block: None,
            icon,
            width: width as u16,
            height: height as u16,
            lines,
        }
    }

    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl Widget for Bubble<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(self.icon.width()), Constraint::Min(1)].as_ref())
            .split(area);
        // icon block
        let layout_icon = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(layout[0])[0];
        Paragraph::new(self.icon.name.as_str())
            .block(self.icon.block.unwrap_or_default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .fg(Color::Green)
            )
            .render(layout_icon, buf);

        // message block
        let block = self.block.unwrap_or_default();
        Paragraph::new(self.lines)
            .block(block.padding(Padding::horizontal(1))
                .borders(Borders::ALL)
                .border_set(symbols::border::Set {
                    top_left: symbols::border::PLAIN.top_left,
                    ..symbols::border::ROUNDED
                }))
            .render(Rect {
                width: self.width.min(layout[1].width),
                height: self.height.min(layout[1].height),
                ..layout[1]
            }, buf);
    }
}
