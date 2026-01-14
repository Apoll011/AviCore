use crate::actions::action::Action;
use crate::api::Api;
use crate::ctx::runtime;
use crate::dialogue::reply::Replayed;
use crate::skills::manager::SkillManager;
use crate::{subscribe, watch_dir};
use avi_device::device::AviDevice;
use log::{info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub struct IntentAction {
    device: Arc<AviDevice>,
    api: Arc<Mutex<Api>>,
    skill_manager: Arc<Mutex<SkillManager>>,
    config: IntentConfig,
}

pub struct IntentConfig {
    pub watch_skill_dir: bool,
    pub watch_dir_debounce_time: u64,
}

impl IntentAction {
    pub async fn process_reply_text(text: &str) -> Result<Replayed, String> {
        match runtime() {
            Ok(c) => c.reply_manager.process_text(text).await,
            Err(e) => Err(e),
        }
    }

    pub async fn parse_as_reply(&self, text: &str) -> bool {
        let skill_manager: Arc<Mutex<SkillManager>> = Arc::clone(&self.skill_manager);

        match IntentAction::process_reply_text(text).await {
            Ok(replay) => {
                let mut mg = skill_manager.lock().await;
                if let Err(e) = mg.run_skill_function_ptr(
                    &replay.pending_reply.skill_request,
                    replay.pending_reply.handler,
                    vec![replay.parsed_output],
                ) {
                    warn!("Error executing replay: {}", e);
                }
                true
            }
            Err(e) => {
                if !e.is_empty() {
                    warn!("Error processing replay: {}", e);
                    return true;
                }
                false
            }
        }
    }

    pub async fn parse_as_intent(&self, text: &str) -> bool {
        let api = Arc::clone(&self.api);
        let skill_manager = Arc::clone(&self.skill_manager);

        let maybe_intent = match api.lock().await.intent(text).await {
            Ok(intent) => Some(intent),
            Err(e) => {
                warn!("Failed to parse intent: {}", e);
                None
            }
        };

        if let Some(intent) = maybe_intent {
            let mut mg = skill_manager.lock().await;

            if let Err(e) = mg.run_intent(intent) {
                warn!("Error executing intent: {}", e);
            } else {
                return true;
            }
        }
        false
    }
}

impl Action for IntentAction {
    type Config = IntentConfig;

    async fn new(config: Self::Config) -> Result<IntentAction, String> {
        let api = Api::new();

        match api.alive().await {
            Ok(_) => {
                info!("Avi NLU API is up and running.");
            }
            Err(_) => {
                return Err("Avi NLU API is not running. Skipping intent actions.".to_string());
            }
        };

        Ok(Self {
            device: Arc::clone(&runtime()?.device),
            api: Arc::new(Mutex::new(api)),
            skill_manager: Arc::new(Mutex::new(SkillManager::new())),
            config,
        })
    }

    async fn register(&mut self) {
        let device = Arc::clone(&self.device);
        let api = Arc::clone(&self.api);
        let skill_manager = Arc::clone(&self.skill_manager);

        let _ = subscribe!("intent/execute/text", captures: [skill_manager, api, device], async: |_from, _topic, data| {
                let msg = String::from_utf8_lossy(&data);
                let text = msg.trim();

                let intent_action = IntentAction {
                    device: Arc::clone(&device),
                    api,
                    skill_manager,
                    config: IntentConfig { watch_skill_dir: false, watch_dir_debounce_time: 10 }
                };

                if !intent_action.parse_as_reply(text).await {
                    let _ = intent_action.parse_as_intent(text).await;
                }
        });

        let _ = subscribe!("intent/reply/cancel", async: move |_from, _topic, _data| async move {
            if let Ok(c) = runtime() { c.reply_manager.cancel().await };
        });

        let _ = subscribe!("skills/reload", captures: [skill_manager], async: |_from, _topic, _data| {
            let mut lock = skill_manager.lock().await;
            let _ = lock.reload();
        });

        if self.config.watch_skill_dir {
            let time = self.config.watch_dir_debounce_time;
            watch_dir!("./config/skills", Duration::from_secs(time), captures: [skill_manager], async: |event| {
                if event.path.is_dir() {
                    return
                }

                match event.path.extension() {
                    Some(extension) => {if !extension.eq("avi") { return }}
                    None => return
                }

                let mut lock = skill_manager.lock().await;
                let _ = lock.reload();
                info!("Reloaded skills due to change in: {:?}", event.path);
            });
        }
    }
}
