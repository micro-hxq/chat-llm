use std::time::Instant;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum Role {
    System,
    User,
    Assistant,
}


#[derive(Debug, Clone, PartialEq, Hash)]
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

    pub fn is_user(&self) -> bool {
        self.role == Role::User
    }

    pub fn is_system(&self) -> bool {
        self.role == Role::System
    }

    pub fn is_assistant(&self) -> bool {
        self.role == Role::Assistant
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
