use crate::actions::action::Action;
use crate::api::Api;
use crate::ctx::runtime;
use crate::dialogue::intent::{Intent, IntentInfo, Slot};
use crate::dialogue::languages::lang;
use crate::dialogue::reply::Replayed;
use crate::skills::manager::SkillManager;
use crate::{subscribe, watch_dir};
use avi_device::device::AviDevice;
use avi_nlu_client::models::{self, Alive, Data1Inner};
use log::{error, info, warn};
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
                if let Err(e) = mg.run_skill_function_ptr::<String, ()>(
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

        let maybe_intent = match api.lock().await.intent(text).await {
            Ok(intent) => Some(intent),
            Err(e) => {
                warn!("Failed to parse intent: {}", e);
                None
            }
        };

        if let Some(recognized) = maybe_intent {
            match *recognized.result {
                models::Result::Nlu(intent) => {
                    return self.process_intent(*intent).await;
                }
                _ => return false,
            }
        }
        false
    }

    async fn update_engines(&self) {
        let skill_manager = Arc::clone(&self.skill_manager);
        let api = Arc::clone(&self.api);

        let dataset = skill_manager.lock().await.get_dataset();

        match api.lock().await.set_engine_dataset(dataset).await {
            Ok(_) => info!("Updated the engine sucessfully"),
            Err(e) => error!("Error updating the engine: {}", e),
        }

        match api
            .lock()
            .await
            .train_intent_engine(models::EngineTrainType::Train)
            .await
        {
            Ok(v) => info!("Trained the engine on lang {}", v.lang),
            Err(e) => error!("Error training the engine: {}", e),
        }
    }

    async fn should_update_engine(&self) -> bool {
        let api = Arc::clone(&self.api);

        let active_intents_on_api: Vec<String> = api
            .lock()
            .await
            .get_active_intents()
            .await
            .unwrap_or_default()
            .get(&lang())
            .cloned()
            .unwrap_or_default();

        let intents_i_have = self
            .skill_manager
            .lock()
            .await
            .get_dataset()
            .data
            .iter()
            .filter_map(|item| match item {
                Data1Inner::Intent(intent) => Some(intent.name.clone()),
                _ => None,
            })
            .collect::<Vec<String>>();

        if active_intents_on_api.len() != intents_i_have.len() {
            return true;
        }

        for item in intents_i_have {
            if !active_intents_on_api.contains(&item) {
                return true;
            }
        }

        false
    }

    async fn process_intent(&self, intent: models::NluResultInput) -> bool {
        let skill_manager = Arc::clone(&self.skill_manager);

        let intent = Intent {
            input: intent.input,
            intent: Some(IntentInfo(*intent.intent)),
            slots: intent.slots.unwrap().into_iter().map(Slot).collect(),
        };
        let mut mg = skill_manager.lock().await;

        if let Err(e) = mg.run_intent(intent) {
            warn!("Error executing intent: {}", e);
        } else {
            return true;
        }
        false
    }

    async fn api_check(api: &Api, alive: Alive) {
        info!("Checking the api");

        if alive.intent_kit {
            info!("Api already has initialized engine");
            return;
        };

        let avaliable_language_engines = api
            .avaliable_engines()
            .await
            .map(|avaliable| avaliable.installed)
            .unwrap_or(vec![]);

        info!("Avaliable engines on api: {:?}", avaliable_language_engines);

        //TODO: Check if the current server language us the same as what I have
        if avaliable_language_engines.contains(&lang()) {
            match api
                .train_intent_engine(models::EngineTrainType::Reuse)
                .await
            {
                Ok(v) => info!("Reused the engine on lang {}", v.lang),
                Err(e) => error!("Error reusing the engine: {}", e),
            }
        };
    }
}

impl Action for IntentAction {
    type Config = IntentConfig;

    async fn new(config: Self::Config) -> Result<IntentAction, String> {
        let api = Api::new();

        match api.alive().await {
            Ok(alive) => {
                info!("Avi NLU API is up and running (Server v{}).", alive.version);
                Self::api_check(&api, alive).await;
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

        if self.should_update_engine().await {
            info!("Updating the engine...");
            self.update_engines().await;
        } else {
            info!("Engine has the latest intent... Ignoring...");
        }

        subscribe!("intent/execute/text", captures: [skill_manager, api, device], async: |_from, _topic, data| {
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

        subscribe!("intent/reply/cancel", async: move |_from, _topic, _data| async move {
            if let Ok(c) = runtime() { c.reply_manager.cancel().await };
        });

        subscribe!("skills/reload", captures: [skill_manager], async: |_from, _topic, _data| {
            let mut lock = skill_manager.lock().await;
            let _ = lock.reload();
        });

        if self.config.watch_skill_dir {
            let time = self.config.watch_dir_debounce_time;
            watch_dir!("./config/skills", Duration::from_secs(time), captures: [skill_manager], async: |event| {

                for path in &event.paths {
                    if path.is_dir() {
                        continue; // or handle directory logic
                    }

                    if let Some(extension) = path.extension() {
                        if extension.to_string_lossy() != "avi" {
                            return;
                        }
                    } else {
                        return;
                    }

                    let mut lock = skill_manager.lock().await;
                    let _ = lock.reload();
                    info!("Reloaded skills due to change in: {:?}", path);
                }
            });
        }
    }
}
