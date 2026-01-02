use std::sync::Arc;
use avi_device::device::AviDevice;
use tokio::sync::Mutex;
use crate::actions::action::Action;
use crate::api::api::Api;
use crate::ctx::runtime;
use crate::dialogue::utils::speak;
use crate::skills::manager::SkillManager;
pub struct IntentAction {
    device: Arc<AviDevice>,
    api: Arc<Mutex<Api>>,
    skill_manager: Arc<Mutex<SkillManager>>,
}

pub struct IntentConfig {
    pub(crate) api: Arc<Mutex<Api>>,
    pub(crate) skill_manager: Arc<Mutex<SkillManager>>,
}

impl Action for IntentAction {
    type Config = IntentConfig;

    fn new(config: Self::Config) -> Self {
        Self {
            device: Arc::clone(&runtime().device),
            api: config.api,
            skill_manager: config.skill_manager,
        }
    }

    async fn register(&mut self) {
        let api = Arc::clone(&self.api);
        let skill_manager = Arc::clone(&self.skill_manager);

        match self.device.subscribe_async("intent/execute/text", move |_from, _topic, data| {
            let api = Arc::clone(&api);
            let skill_manager = Arc::clone(&skill_manager);

            async move {
                let msg = String::from_utf8_lossy(&data);
                let text = msg.trim();

                // Check if reply manager wants to handle this
                if runtime().reply_manager.process_text(text).await {
                    return; // Reply manager consumed the text
                }

                // Normal intent processing
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
        }).await {
            Ok(_) => println!("✓ Registered intent/execute/text handler"),
            Err(e) => eprintln!("Failed to subscribe to intent topic: {}", e)
        }

        // Cancel topic
        match self.device.subscribe_async("intent/reply/cancel", move |_from, _topic, _data| {
            async move {
                runtime().reply_manager.cancel().await;
                println!("Reply cancelled via topic");
                speak("Request cancelled.");
            }
        }).await {
            Ok(_) => println!("✓ Registered intent/reply/cancel handler"),
            Err(e) => eprintln!("Failed to subscribe to cancel topic: {}", e)
        }
    }
}