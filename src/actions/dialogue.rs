use crate::actions::action::Action;
use crate::ctx::runtime;
use avi_device::device::AviDevice;
use std::sync::Arc;

/// Manages dialogue-related interactions such as speaking and listening.
pub struct DialogueAction {
    /// Reference to the Avi device.
    device: Arc<AviDevice>,
    /// Configuration determining which capabilities (speaker/listener) are active.
    config: DialogueConfig,
}

/// Defines the operational capabilities of the dialogue system.
pub enum DialogueCapability {
    /// Both speaker and listener capabilities are enabled.
    #[allow(dead_code)]
    BOTH = 0,
    /// Only speaker capability is enabled.
    SPEAKER = 1,
    /// Only listener capability is enabled.
    #[allow(dead_code)]
    LISTENER = 2,
}

/// Configuration for the `DialogueAction`.
pub struct DialogueConfig {
    /// The capability set for this dialogue action.
    pub capability: DialogueCapability,
}

impl DialogueAction {
    /// Subscribes to the speaker topic for the current device.
    ///
    /// TODO: Implement actual audio output or interaction with a text-to-speech system instead of just printing to console.
    async fn register_speaker(&mut self) {
        self.device
            .subscribe(
                &format!("speak/{}/text", self.device.get_id().await.to_string()),
                move |_from, _topic, data| {
                    let msg = String::from_utf8_lossy(&data);
                    println!("Speaker: {}", msg);
                },
            )
            .await
            .expect("Failed to subscribe to intent topic");
    }

    /// Subscribes to the listener topic for the current device.
    ///
    /// TODO: Implement actual voice recognition or interaction with a speech-to-text system.
    async fn register_listener(&mut self) {
        self.device
            .subscribe(
                &format!("listening/{}/start", self.device.get_id().await.to_string()),
                move |_from, _topic, _data| {
                    println!("Listening...");
                },
            )
            .await
            .expect("Failed to subscribe to intent topic");
    }
}

impl Action for DialogueAction {
    type Config = DialogueConfig;

    /// Creates a new instance of `DialogueAction` with the provided configuration.
    fn new(config: Self::Config) -> Self {
        Self {
            device: Arc::clone(&runtime().device),
            config,
        }
    }

    /// Registers the dialogue action based on its configured capabilities.
    async fn register(&mut self) {
        match self.config.capability {
            DialogueCapability::BOTH => {
                self.register_speaker().await;
                self.register_listener().await;
            }
            DialogueCapability::SPEAKER => self.register_speaker().await,
            DialogueCapability::LISTENER => self.register_listener().await,
        }
    }
}
