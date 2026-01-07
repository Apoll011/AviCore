use crate::dialogue::response::{ResponseValidator, ValidationError};
use crate::locale;
use crate::speak;
use log::{debug, info, trace, warn};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

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
    V::Output: std::fmt::Debug,
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

pub struct PendingReply {
    pub skill_request: String,
    pub handler: String,
    validator: Box<dyn ValidatorErasure>,
    created_at: Instant,
    retry_count: usize,
}

pub struct Replayed {
    pub parsed_output: String,
    pub pending_reply: PendingReply,
}

impl Replayed {
    pub fn new(parsed_output: String, pending_reply: PendingReply) -> Self {
        Self {
            parsed_output,
            pending_reply,
        }
    }
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

        info!(
            "Skill {} requested reply for handler {}.",
            request.skill_request, request.handler
        );
        let pending = PendingReply {
            skill_request: request.skill_request.clone(),
            handler: request.handler,
            validator: request.validator,
            created_at: Instant::now(),
            retry_count: 0,
        };

        *self.pending_reply.lock().await = Some(pending);
    }

    pub async fn cancel(&self) {
        if self.pending_reply.lock().await.is_some() {
            info!("Canceled pending reply.");
            *self.pending_reply.lock().await = None;
        }
    }

    #[allow(dead_code)]
    pub async fn has_pending(&self) -> bool {
        self.pending_reply.lock().await.is_some()
    }

    /// Processes incoming text against pending reply
    /// Returns true if text was consumed by reply handler
    pub async fn process_text(&self, text: &str) -> Result<Replayed, String> {
        trace!("Processing text for pending reply: {}", text);
        let mut pending_lock = self.pending_reply.lock().await;

        if let Some(mut pending) = pending_lock.take() {
            if pending.created_at.elapsed() > Duration::from_secs(self.config.timeout_secs) {
                info!("Reply from skill {} timed out.", pending.skill_request);
                return Err("".to_string());
            }

            if let Some(max) = self.config.max_retries
                && pending.retry_count >= max
            {
                warn!(
                    "Too many invalid attempts for skill {}. Cancelling request.",
                    pending.skill_request
                );
                speak!(locale: "to_many_replay_trys");
                return Err("Too many invalid attempts. Cancelling request.".to_string());
            }

            let cleaned = pending.validator.clear_text(text);
            debug!("Cleaned text for validation: '{}'", cleaned);

            match pending.validator.validate_erased(&cleaned) {
                Ok(parsed_output) => {
                    info!(
                        "Successfully parsed reply for skill {}: {}",
                        pending.skill_request, parsed_output
                    );
                    Ok(Replayed::new(parsed_output, pending))
                }
                Err(error) => {
                    pending.retry_count += 1;
                    let error_msg = pending.validator.get_error_txt(&error);

                    warn!(
                        "Reply for skill {} attempt {}/{} failed: {:?}",
                        pending.skill_request,
                        pending.retry_count,
                        self.config.max_retries.unwrap_or(0),
                        error
                    );

                    if let Some(max) = self.config.max_retries
                        && pending.retry_count >= max
                    {
                        info!(
                            "Too many invalid attempts for skill {}. Cancelling request.",
                            pending.skill_request
                        );
                        speak!(locale: "to_many_replay_trys");
                        return Err("Too many invalid attempts. Cancelling request.".to_string());
                    }

                    *pending_lock = Some(pending);
                    speak!(locale: error_msg.as_str());

                    match locale!(&error_msg) {
                        Some(v) => Err(v.to_string()),
                        None => Err(error_msg),
                    }
                }
            }
        } else {
            trace!("No pending reply to process text against.");
            Err("".to_string())
        }
    }
}
