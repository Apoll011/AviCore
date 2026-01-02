use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use crate::dialogue::response::{ResponseValidator, ValidationError};
use crate::dialogue::utils::speak;

pub struct ReplyManager {
    pending_reply: Arc<Mutex<Option<PendingReply>>>,
    config: ReplyConfig,
}

pub trait ValidatorErasure: Send + Sync {
    fn validate_erased(&self, text: &str) -> Result<String, ValidationError>;
    fn clear_text(&self, text: &str) -> String;
    fn get_error_txt(&self, error: &ValidationError) -> String;
}

impl<V: ResponseValidator + Send + Sync> ValidatorErasure for V
where
    V::Output: std::fmt::Debug
{
    fn validate_erased(&self, text: &str) -> Result<String, ValidationError> {
        self.validate_and_parse(text)
            .map(|output| format!("{:?}", output))
    }

    fn clear_text(&self, text: &str) -> String {
        ResponseValidator::clear_text(self, text)
    }

    fn get_error_txt(&self, error: &ValidationError) -> String {
        ResponseValidator::get_error_txt(self, error)
    }
}

pub struct RequestReply {
    pub skill_request: String,
    pub handler: String,
    pub validator: Box<dyn ValidatorErasure>,
}

struct PendingReply {
    skill_request: String,
    handler: String,
    validator: Box<dyn ValidatorErasure>,
    created_at: Instant,
    retry_count: usize,
}

#[derive(Debug, Clone)]
pub struct ReplyConfig {
    pub timeout_secs: u64,
    pub max_retries: Option<usize>,
}

impl Default for ReplyConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            max_retries: Some(3),
        }
    }
}

impl ReplyManager {
    pub fn new(config: Option<ReplyConfig>) -> Self {
        Self {
            pending_reply: Arc::new(Mutex::new(None)),
            config: config.unwrap_or_default(),
        }
    }

    pub async fn set_reply(&self, request: RequestReply) {
        self.cancel().await;

        let pending = PendingReply {
            skill_request: request.skill_request.clone(),
            handler: request.handler,
            validator: request.validator,
            created_at: Instant::now(),
            retry_count: 0,
        };

        *self.pending_reply.lock().await = Some(pending);
        speak(&request.skill_request);
    }

    pub async fn cancel(&self) {
        *self.pending_reply.lock().await = None;
    }

    pub async fn has_pending(&self) -> bool {
        self.pending_reply.lock().await.is_some()
    }

    /// Processes incoming text against pending reply
    /// Returns true if text was consumed by reply handler
    pub async fn process_text(&self, text: &str) -> bool {
        let mut pending_lock = self.pending_reply.lock().await;

        if let Some(mut pending) = pending_lock.take() {
            // Check timeout
            if pending.created_at.elapsed() > Duration::from_secs(self.config.timeout_secs) {
                println!("Reply timed out after {} seconds", self.config.timeout_secs);
                speak("Request timed out. Please try again.");
                return true;
            }

            // Check max retries
            if let Some(max) = self.config.max_retries {
                if pending.retry_count >= max {
                    println!("Max retries ({}) exceeded", max);
                    speak("Too many invalid attempts. Cancelling request.");
                    return true;
                }
            }

            // Validate
            let cleaned = pending.validator.clear_text(text);

            match pending.validator.validate_erased(&cleaned) {
                Ok(parsed_output) => {
                    // Success! Call handler
                    println!("✓ Validation successful");
                    println!("  Handler: {}", pending.handler);
                    println!("  Parsed output: {}", parsed_output);
                    speak(&format!("Received: {}", parsed_output));
                    true
                }
                Err(error) => {
                    // Failed, retry
                    pending.retry_count += 1;
                    let error_msg = pending.validator.get_error_txt(&error);

                    println!("✗ Validation failed (attempt {}/{})",
                             pending.retry_count,
                             self.config.max_retries.map_or("∞".to_string(), |m| m.to_string())
                    );

                    // Check if should continue
                    if let Some(max) = self.config.max_retries {
                        if pending.retry_count >= max {
                            speak("Too many invalid attempts. Cancelling request.");
                            return true;
                        }
                    }

                    speak(&error_msg);
                    *pending_lock = Some(pending);
                    true
                }
            }
        } else {
            false // No pending reply
        }
    }
}