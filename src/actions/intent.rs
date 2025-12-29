use std::sync::Arc;
use avi_device::device::AviDevice;
use tokio::sync::Mutex;
use crate::actions::action::{Action};
use crate::api::api::Api;
use crate::ctx::runtime;
use crate::skills::manager::SkillManager;

/// Represents an action that handles intent execution from incoming text messages.
/// 
/// It subscribes to a specific device topic and uses the `Api` and `SkillManager`
/// to process and run the requested intent.
pub struct IntentAction {
    /// Reference to the Avi device for communication.
    device: Arc<AviDevice>,
    /// Thread-safe access to the API for intent recognition.
    api: Arc<Mutex<Api>>,
    /// Thread-safe access to the manager responsible for running skills.
    skill_manager: Arc<Mutex<SkillManager>>,
}

/// Configuration required to initialize an `IntentAction`.
pub struct IntentConfig {
    /// Thread-safe reference to the API.
    pub(crate) api: Arc<Mutex<Api>>,
    /// Thread-safe reference to the skill manager.
    pub(crate) skill_manager: Arc<Mutex<SkillManager>>
}

impl Action for IntentAction {
    type Config = IntentConfig;

    /// Creates a new instance of `IntentAction` with the provided configuration.
    /// 
    /// # Arguments
    /// 
    /// * `config` - The configuration containing API and skill manager references.
    /// 
    /// TODO: Validate if `runtime().device` is already initialized before cloning.
    fn new(config: Self::Config) -> Self {
        Self {
            device: Arc::clone(&runtime().device),
            api: config.api,
            skill_manager: config.skill_manager
        }
    }

    /// Registers the intent action by subscribing to the "intent/execute/text" topic.
    /// 
    /// When a message is received on this topic, it is processed as an intent
    /// using the API and then executed by the skill manager.
    /// 
    /// FIXME: `expect("Failed to subscribe to intent topic")` will panic the entire application if subscription fails. Consider returning a `Result`.
    async fn register(&mut self) {
        let api = Arc::clone(&self.api);
        let skill_manager = Arc::clone(&self.skill_manager);

        self.device.subscribe_async("intent/execute/text", move |_from, _topic, data| {
            let api = Arc::clone(&api);
            let skill_manager = Arc::clone(&skill_manager);

            async move {
                let msg = String::from_utf8_lossy(&data);

                let maybe_intent = match api.lock().await.intent(&*msg).await {
                    Ok(intent) => Some(intent),
                    Err(e) => {
                        println!("Failed to parse intent: {}", e);
                        None
                    }
                };

                if let Some(intent) = maybe_intent {
                    let mut mg = skill_manager.lock().await;
                    if let Err(e) = mg.run_intent(intent) {
                        println!("Error executing intent: {}", e);
                    }
                }
            }
        }).await.expect("Failed to subscribe to intent topic");
    }
}