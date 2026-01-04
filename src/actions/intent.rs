use crate::actions::action::Action;
use crate::api::api::Api;
use crate::ctx::runtime;
use crate::skills::manager::SkillManager;
use crate::speak;
use avi_device::device::AviDevice;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct IntentAction {
    device: Arc<AviDevice>,
    api: Arc<Mutex<Api>>,
    skill_manager: Arc<Mutex<SkillManager>>,
}

pub struct IntentConfig {}

impl Action for IntentAction {
    type Config = IntentConfig;

    fn new(_config: Self::Config) -> Self {
        Self {
            device: Arc::clone(&runtime().device),
            api: Arc::new(Mutex::new(Api::new())),
            skill_manager: Arc::new(Mutex::new(SkillManager::new())),
        }
    }

    async fn register(&mut self) {
        let api = Arc::clone(&self.api);
        let skill_manager = Arc::clone(&self.skill_manager);

        match self
            .device
            .subscribe_async("intent/execute/text", move |_from, _topic, data| {
                let api = Arc::clone(&api);
                let skill_manager = Arc::clone(&skill_manager);

                async move {
                    let msg = String::from_utf8_lossy(&data);
                    let text = msg.trim();
                    let mut processed = false;

                    match runtime().reply_manager.process_text(text).await {
                        Ok(replay) => {
                            let mut mg = skill_manager.lock().await;
                            if let Err(e) = mg.run_skill_function(
                                &replay.pending_reply.skill_request,
                                &replay.pending_reply.handler,
                                vec![replay.parsed_output],
                            ) {
                                println!("Error executing replay: {}", e);
                            }
                            processed = true;
                        }
                        Err(e) => {
                            if !e.is_empty() {
                                speak!(&e);
                            }
                        }
                    }

                    if processed {
                        return;
                    }

                    let maybe_intent = match api.lock().await.intent(&msg).await {
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
            })
            .await
        {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to subscribe to intent topic: {}", e),
        }

        // Cancel topic
        match self
            .device
            .subscribe_async(
                "intent/reply/cancel",
                move |_from, _topic, _data| async move {
                    runtime().reply_manager.cancel().await;
                    speak!("Request cancelled.");
                },
            )
            .await
        {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to subscribe to cancel topic: {}", e),
        }
    }
}
