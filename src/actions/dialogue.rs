use std::sync::Arc;
use avi_device::device::AviDevice;
use crate::actions::action::{Action};
use crate::ctx::runtime;

pub struct DialogueAction {
    device: Arc<AviDevice>,
    config: DialogueConfig,
}

pub enum DialogueCapability {
    BOTH = 0,
    SPEAKER = 1,
    LISTENER = 2,
}

pub struct DialogueConfig {
    pub capability: DialogueCapability,
}

impl DialogueAction {
    async fn register_speaker(&mut self) {
        self.device.subscribe(&format!("speak/{}/text", self.device.get_id().await.to_string()), move |_from, _topic, data| {
            let msg = String::from_utf8_lossy(&data);
            println!("Speaker: {}", msg);
        }).await.expect("Failed to subscribe to intent topic");
    }

    async fn register_listener(&mut self) {
        self.device.subscribe(&format!("listening/{}/start", self.device.get_id().await.to_string()), move |_from, _topic, data| {
            println!("Listening...");
        }).await.expect("Failed to subscribe to intent topic");
    }
}

impl Action for DialogueAction {
    type Config = DialogueConfig;
    fn new(config: Self::Config) -> Self {
        Self {
            device: Arc::clone(&runtime().device),
            config
        }
    }

    async fn register(&mut self) {
        match self.config.capability {
            DialogueCapability::BOTH => {
                self.register_speaker().await;
                self.register_listener().await;
            },
            DialogueCapability::SPEAKER => self.register_speaker().await,
            DialogueCapability::LISTENER => self.register_listener().await,
        }
    }
}