use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::Bubble;
use crate::data::ChatMessage;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Dialogue<'a> {
    messages: &'a Vec<&'a ChatMessage>,
    block: Option<Block<'a>>,
}

impl<'a> Dialogue<'a> {
    pub fn new(messages: &'a Vec<&'a ChatMessage>) -> Self {
        Self {
            messages,
            block: None,
        }
    }

    pub fn with_block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl StatefulWidget for Dialogue<'_> {
    type State = DialogueState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let inner_area = match self.block {
            Some(block) => {
                let inner = block.inner(area);
                block.render(area, buf);
                inner
            }
            None => {
                area
            }
        };
        let (width, height) = (inner_area.width, inner_area.height);
        let bubbles = self.messages.iter()
            .map(|&message| Bubble::new(message).with_width(width))
            .collect::<Vec<Bubble>>();
        let content_height = bubbles.iter().map(|bubble| bubble.height()).sum::<u16>();
        // viewport can render all messages
        if content_height <= height {
            let mut rect = inner_area;
            for bubble in bubbles {
                let bubble_height = bubble.height();
                bubble.render(rect, buf);
                rect.y += bubble_height;
                rect.height -= bubble_height;
            }
            return;
        }
        state.viewport_length = height;
        state.content_length = content_height;

        let (view_begin, view_end) = if state.pending {
            (content_height - height, content_height)
        } else {
            (state.position, state.position + height)
        };

        // viewport can't render all messages
        let mut rect = inner_area;
        let mut current_y = 0;
        for bubble in bubbles {
            if current_y > view_end {
                break;
            }
            let bubble_height = bubble.height();
            if current_y < view_begin && current_y + bubble_height > view_begin && current_y + bubble_height < view_end {
                let remaining = bubble_height - (view_begin - current_y);
                rect.y += remaining;
                rect.height -= remaining;
            } else if current_y >= view_begin && current_y + bubble_height <= view_end {
                bubble.render(rect, buf);
                rect.y += bubble_height;
                rect.height -= bubble_height;
            }
            current_y += bubble_height;
        }
        // render scrollbar widget
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let mut scrollbar_state = ScrollbarState::new(state.content_length as usize)
            .viewport_content_length(state.viewport_length as usize)
            .position(state.scroll_position() as usize);
        scrollbar.render(area, buf, &mut scrollbar_state);
    }
}


#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct DialogueState {
    pending: bool,
    position: u16,
    viewport_length: u16,
    content_length: u16,
}

impl DialogueState {
    pub fn new() -> Self {
        Self {
            pending: false,
            position: 0,
            viewport_length: 0,
            content_length: 0,
        }
    }

    pub fn with_pending(mut self, pending: bool) -> Self {
        self.pending = pending;
        self
    }

    pub fn with_offset(mut self, offset: u16) -> Self {
        self.position = offset;
        self
    }

    pub fn with_viewport_length(mut self, viewport_length: u16) -> Self {
        self.viewport_length = viewport_length;
        self
    }

    pub fn with_content_length(mut self, content_length: u16) -> Self {
        self.content_length = content_length;
        self
    }

    pub fn prev(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }

    pub fn next(&mut self) {
        if self.position < self.content_length - self.viewport_length {
            self.position += 1;
        }
    }

    pub fn first(&mut self) {
        self.position = 0
    }

    pub fn last(&mut self) {
        if self.content_length > self.viewport_length {
            self.position = self.content_length - self.viewport_length;
        }
    }

    pub fn scroll_position(&self) -> u16 {
        (self.content_length as f32 * self.position as f32
            / (self.content_length - self.viewport_length) as f32).round() as u16
    }
}