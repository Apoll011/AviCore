use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// Command Definition
// ============================================================================

#[derive(Debug, Clone)]
pub enum Command {
    // System
    Help,
    Tutorial,
    Exit,
    Clear,

    // Network
    Peers,
    Status,
    Query,

    // PubSub
    Subscribe { topic: String },
    SubscribeTyped { topic: String, type_name: String },
    Unsubscribe { topic: String },
    Publish { topic: String, message: String },
    PublishTyped { topic: String, json_data: String },
    ListSubscriptions,

    // Context
    SetContext { path: String, value: String },
    GetContext { path: String },
    SetSpeaker,

    // Intent
    Intent { message: String },

    // Stream/Chat
    Call { peer_id: String },
    Message { text: String },
    Typing,
    Hangup,
    Sessions,
    History { session_id: Option<String> },
    SwitchSession { session_id: String },

    // Advanced
    Broadcast { message: String },
    Stats,
}

impl Command {
    pub fn from_parts(parts: &[&str]) -> Result<Self, String> {
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }

        match parts[0] {
            // System
            "help" | "h" | "?" => Ok(Command::Help),
            "tutorial" | "guide" => Ok(Command::Tutorial),
            "exit" | "quit" | "q" => Ok(Command::Exit),
            "clear" | "cls" => Ok(Command::Clear),

            // Network
            "peers" | "p" => Ok(Command::Peers),
            "status" | "info" => Ok(Command::Status),
            "query" | "find" => Ok(Command::Query),

            // PubSub
            "sub" | "subscribe" => {
                if parts.len() < 2 {
                    return Err("Usage: sub <topic>".to_string());
                }
                Ok(Command::Subscribe {
                    topic: parts[1].to_string(),
                })
            }
            "sub-typed" | "subscribe-typed" => {
                if parts.len() < 3 {
                    return Err("Usage: sub-typed <topic> <type>".to_string());
                }
                Ok(Command::SubscribeTyped {
                    topic: parts[1].to_string(),
                    type_name: parts[2].to_string(),
                })
            }
            "unsub" | "unsubscribe" => {
                if parts.len() < 2 {
                    return Err("Usage: unsub <topic>".to_string());
                }
                Ok(Command::Unsubscribe {
                    topic: parts[1].to_string(),
                })
            }
            "pub" | "publish" => {
                if parts.len() < 3 {
                    return Err("Usage: pub <topic> <message>".to_string());
                }
                Ok(Command::Publish {
                    topic: parts[1].to_string(),
                    message: parts[2..].join(" "),
                })
            }
            "pub-typed" | "publish-typed" => {
                if parts.len() < 3 {
                    return Err("Usage: pub-typed <topic> <json>".to_string());
                }
                Ok(Command::PublishTyped {
                    topic: parts[1].to_string(),
                    json_data: parts[2..].join(" "),
                })
            }
            "subs" | "subscriptions" => Ok(Command::ListSubscriptions),

            // Context
            "set" => {
                if parts.len() < 3 {
                    return Err("Usage: set <path> <value>".to_string());
                }
                Ok(Command::SetContext {
                    path: parts[1].to_string(),
                    value: parts[2..].join(" "),
                })
            }
            "get" => {
                let path = if parts.len() > 1 {
                    parts[1].to_string()
                } else {
                    String::new()
                };
                Ok(Command::GetContext { path })
            }
            "speaker" => Ok(Command::SetSpeaker),

            // Intent
            "intent" => {
                if parts.len() < 2 {
                    return Err("Usage: intent <message>".to_string());
                }
                Ok(Command::Intent {
                    message: parts[1..].join(" "),
                })
            }

            // Stream/Chat
            "call" | "dial" => {
                if parts.len() < 2 {
                    return Err("Usage: call <peer_id>".to_string());
                }
                Ok(Command::Call {
                    peer_id: parts[1].to_string(),
                })
            }
            "msg" | "send" | "say" => {
                if parts.len() < 2 {
                    return Err("Usage: msg <text>".to_string());
                }
                Ok(Command::Message {
                    text: parts[1..].join(" "),
                })
            }
            "typing" => Ok(Command::Typing),
            "hangup" | "close" | "end" => Ok(Command::Hangup),
            "sessions" | "chats" => Ok(Command::Sessions),
            "history" | "hist" => {
                let session_id = if parts.len() > 1 {
                    Some(parts[1].to_string())
                } else {
                    None
                };
                Ok(Command::History { session_id })
            }
            "switch" => {
                if parts.len() < 2 {
                    return Err("Usage: switch <session_id>".to_string());
                }
                Ok(Command::SwitchSession {
                    session_id: parts[1].to_string(),
                })
            }

            // Advanced
            "broadcast" | "bc" => {
                if parts.len() < 2 {
                    return Err("Usage: broadcast <message>".to_string());
                }
                Ok(Command::Broadcast {
                    message: parts[1..].join(" "),
                })
            }
            "stats" | "statistics" => Ok(Command::Stats),

            _ => Err(format!("Unknown command: '{}'", parts[0])),
        }
    }

    /// Get command name for help display
    pub fn name(&self) -> &str {
        match self {
            Command::Help => "help",
            Command::Tutorial => "tutorial",
            Command::Exit => "exit",
            Command::Clear => "clear",
            Command::Peers => "peers",
            Command::Status => "status",
            Command::Query => "query",
            Command::Subscribe { .. } => "subscribe",
            Command::SubscribeTyped { .. } => "subscribe-typed",
            Command::Unsubscribe { .. } => "unsubscribe",
            Command::Publish { .. } => "publish",
            Command::PublishTyped { .. } => "publish-typed",
            Command::ListSubscriptions => "subscriptions",
            Command::SetContext { .. } => "set",
            Command::GetContext { .. } => "get",
            Command::SetSpeaker => "speaker",
            Command::Intent { .. } => "intent",
            Command::Call { .. } => "call",
            Command::Message { .. } => "message",
            Command::Typing => "typing",
            Command::Hangup => "hangup",
            Command::Sessions => "sessions",
            Command::History { .. } => "history",
            Command::SwitchSession { .. } => "switch",
            Command::Broadcast { .. } => "broadcast",
            Command::Stats => "stats",
        }
    }
}

// ============================================================================
// Command Executor (for future extensibility)
// ============================================================================

pub trait CommandExecutor: Send + Sync {
    fn execute(&self, command: Command) -> Result<(), String>;
}

// ============================================================================
// Command Registry (for plugin-style extensions)
// ============================================================================

pub struct CommandRegistry {
    executors: HashMap<String, Arc<dyn CommandExecutor>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            executors: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, executor: Arc<dyn CommandExecutor>) {
        self.executors.insert(name, executor);
    }

    pub fn execute(&self, command_name: &str, command: Command) -> Result<(), String> {
        if let Some(executor) = self.executors.get(command_name) {
            executor.execute(command)
        } else {
            Err(format!("No executor found for command: {}", command_name))
        }
    }

    pub fn has_executor(&self, command_name: &str) -> bool {
        self.executors.contains_key(command_name)
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parsing() {
        let parts = vec!["sub", "test-topic"];
        let cmd = Command::from_parts(&parts).unwrap();
        assert!(matches!(cmd, Command::Subscribe { .. }));

        let parts = vec!["msg", "hello", "world"];
        let cmd = Command::from_parts(&parts).unwrap();
        if let Command::Message { text } = cmd {
            assert_eq!(text, "hello world");
        } else {
            panic!("Expected Message command");
        }
    }

    #[test]
    fn test_aliases() {
        let parts = vec!["h"];
        assert!(matches!(
            Command::from_parts(&parts).unwrap(),
            Command::Help
        ));

        let parts = vec!["p"];
        assert!(matches!(
            Command::from_parts(&parts).unwrap(),
            Command::Peers
        ));
    }
}
