use std::time::Instant;

#[derive(Debug)]
pub enum Mode {
    Normal,
    Input,
}

#[derive(Debug)]
pub enum LlmStatus {
    Idle,
    Busy,
}


#[derive(Debug)]
pub enum Role {
    System,
    User,
    Assistant,
}


#[derive(Debug)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    pub timestamp: Instant,
}

impl ChatMessage {
    pub fn new(role: Role, content: String) -> Self {
        Self {
            role,
            content,
            timestamp: Instant::now(),
        }
    }
}


#[derive(Debug)]
pub struct Conversation {
    pub id: i64,
    pub title: String,
    pub messages: Vec<ChatMessage>,

}

impl Conversation {
    pub fn new(id: i64, title: String) -> Self {
        Self {
            id,
            title,
            messages: Vec::new(),
        }
    }
}


#[derive(Debug)]
pub struct History {
    pub conversations: Vec<Conversation>,
}

impl History {
    pub fn new() -> Self {
        Self {
            conversations: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.conversations.len()
    }
}

