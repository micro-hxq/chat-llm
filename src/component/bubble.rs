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
    width: u16,
    height: u16,
    icon: Option<Icon<'a>>,
    lines: Option<Vec<Line<'a>>>,
    block: Option<Block<'a>>,

}

impl<'a> Bubble<'a> {
    pub fn new(message: &'a ChatMessage) -> Self {
        Self {
            message,
            icon: None,
            lines: None,
            block: None,
            width: 0,
            height: 0,
        }
    }

    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn with_width(mut self, width: u16) -> Self {
        let icon = Icon {
            name: if self.message.is_user() { "You" } else { "Bot" }.to_owned(),
            block: None,
        };
        let width = width as usize;
        let lines = util::split_message(&self.message.content, width - 4 - icon.width() as usize);
        let width = width.min(lines.iter().map(|line| line.width()).max().unwrap_or(0) + 4);
        self.width = width as u16;
        self.height = lines.len() as u16 + 2;
        self.icon = Some(icon);
        self.lines = Some(lines);
        return self;
    }
}

impl Widget for Bubble<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let icon = self.icon.unwrap();
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(icon.width()), Constraint::Min(1)].as_ref())
            .split(area);
        // icon block
        let layout_icon = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(layout[0])[0];
        Paragraph::new(icon.name.as_str())
            .block(icon.block.unwrap_or_default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .fg(Color::Green)
            )
            .render(layout_icon, buf);

        // message block
        let block = self.block.unwrap_or_default();
        Paragraph::new(self.lines.unwrap_or_default())
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
