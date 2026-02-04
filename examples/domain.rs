use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// Message Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Text,
    Typing,
    System,
    Command,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Text => write!(f, "text"),
            MessageType::Typing => write!(f, "typing"),
            MessageType::System => write!(f, "system"),
            MessageType::Command => write!(f, "command"),
        }
    }
}

// ============================================================================
// Chat Message
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
    pub timestamp: i64,
    pub message_type: MessageType,
}

impl ChatMessage {
    pub fn new(sender: String, content: String, message_type: MessageType) -> Self {
        Self {
            sender,
            content,
            timestamp: chrono::Utc::now().timestamp(),
            message_type,
        }
    }

    pub fn text(sender: String, content: String) -> Self {
        Self::new(sender, content, MessageType::Text)
    }

    pub fn typing(sender: String) -> Self {
        Self::new(sender, String::new(), MessageType::Typing)
    }

    pub fn system(sender: String, content: String) -> Self {
        Self::new(sender, content, MessageType::System)
    }
}

// ============================================================================
// Typing Indicator
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingIndicator {
    pub peer_id: String,
    pub is_typing: bool,
    pub timestamp: i64,
}

impl TypingIndicator {
    pub fn new(peer_id: String, is_typing: bool) -> Self {
        Self {
            peer_id,
            is_typing,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

// ============================================================================
// Chat Session
// ============================================================================

use avi_p2p::StreamId;

#[derive(Debug)]
pub struct ChatSession {
    pub id: String,
    pub stream_id: StreamId,
    pub peer_id: String,
    pub started_at: i64,
    pub messages: Vec<ChatMessage>,
    pub is_active: bool,
}

impl ChatSession {
    pub fn new(stream_id: StreamId, peer_id: String) -> Self {
        let id = format!(
            "session-{}",
            uuid::Uuid::new_v4().to_string()[..8].to_string()
        );

        Self {
            id,
            stream_id,
            peer_id,
            started_at: chrono::Utc::now().timestamp(),
            messages: Vec::new(),
            is_active: true,
        }
    }

    pub fn add_message(&mut self, sender: String, content: String, message_type: MessageType) {
        let msg = ChatMessage::new(sender, content, message_type);
        self.messages.push(msg);
    }

    pub fn close(&mut self) {
        self.is_active = false;
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn get_last_message(&self) -> Option<&ChatMessage> {
        self.messages.last()
    }
}

// ============================================================================
// Session Info (for listing)
// ============================================================================

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub id: String,
    pub stream_id: StreamId,
    pub peer_id: String,
    pub started_at: i64,
    pub message_count: usize,
    pub is_active: bool,
}

impl From<&ChatSession> for SessionInfo {
    fn from(session: &ChatSession) -> Self {
        Self {
            id: session.id.clone(),
            stream_id: session.stream_id,
            peer_id: session.peer_id.clone(),
            started_at: session.started_at,
            message_count: session.message_count(),
            is_active: session.is_active,
        }
    }
}

// ============================================================================
// Subscription Info
// ============================================================================

#[derive(Debug, Clone)]
pub struct SubscriptionInfo {
    pub topic: String,
    pub message_type: Option<String>,
    pub created_at: i64,
    pub message_count: usize,
}

impl SubscriptionInfo {
    pub fn new(topic: String, message_type: Option<String>) -> Self {
        Self {
            topic,
            message_type,
            created_at: chrono::Utc::now().timestamp(),
            message_count: 0,
        }
    }
}
