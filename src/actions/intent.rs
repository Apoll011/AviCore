use std::sync::Arc;
use avi_device::device::AviDevice;
use tokio::sync::Mutex;
use crate::actions::action::Action;
use crate::api::api::Api;
use crate::skills::manager::SkillManager;

pub struct IntentAction {
    device: Arc<AviDevice>,
    api: Arc<Mutex<Api>>,
    skill_manager: Arc<Mutex<SkillManager>>,
}

impl Action for IntentAction {
    fn new(device: &Arc<AviDevice>, api: &Arc<Mutex<Api>>, skill_manager: &Arc<Mutex<SkillManager>>) -> Self {
        Self {
            device: Arc::clone(device),
            api: Arc::clone(api),
            skill_manager: Arc::clone(skill_manager)
        }
    }

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