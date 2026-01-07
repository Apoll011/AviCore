use crate::ctx::runtime;
use crate::{core_id, get_ctx, publish, rt_spawn, set_ctx};
use log::{debug, error, trace};

/// Retrieves the ID of the last active listener device.
///
/// If no specific listener is stored in the device context, it defaults to the core device ID.
///
/// # Errors
///
/// Returns an error if the context is found but cannot be parsed, or if the core ID retrieval fails.
#[allow(dead_code)]
pub async fn get_last_listener() -> Result<String, Box<dyn std::error::Error>> {
    trace!("Getting last listener device ID");
    match get_ctx!(device, "avi.dialogue.listener") {
        Some(v) => {
            let id = v.as_str().ok_or("Listener context is not a string")?.to_string();
            debug!("Last listener found in context: {}", id);
            Ok(id)
        },
        None => {
            let id = core_id!()?;
            debug!("No listener found in context, defaulting to core ID: {}", id);
            Ok(id)
        },
    }
}

/// Retrieves the ID of the current speaker device.
///
/// If no specific speaker is stored in the device context, it defaults to the core device ID.
///
/// # Errors
///
/// Returns an error if the context is found but cannot be parsed, or if the core ID retrieval fails.
pub async fn get_speaker() -> Result<String, Box<dyn std::error::Error>> {
    trace!("Getting current speaker device ID");
    match get_ctx!(device, "avi.dialogue.speaker") {
        Some(v) => {
            let id = v.as_str().ok_or("Speaker context is not a string")?.to_string();
            debug!("Speaker found in context: {}", id);
            Ok(id)
        },
        None => {
            let id = core_id!()?;
            debug!("No speaker found in context, defaulting to core ID: {}", id);
            Ok(id)
        },
    }
}

/// Publishes a text message to the speaker's topic to be spoken aloud.
///
/// This function spawns an asynchronous task to handle the publication.
///
/// # Arguments
///
/// * `text` - The string content to be spoken.
///
/// TODO: Handle the case where the speaker device is offline or unavailable.
pub fn speak(text: &str, store: bool) {
    let text = text.to_string();
    trace!("Speak request: '{}' (store={})", text, store);

    if store {
        set_ctx!("utterance.last", text.clone());
    }

    rt_spawn! {
        let speaker = match get_speaker().await {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to get speaker for speak request: {e}");
                return;
            }
        };

        debug!("Publishing speak request to speaker {}: '{}'", speaker, text);
        if let Err(e) = publish!(&format!("speak/{}/text", speaker), text.into_bytes()) {
            error!("Failed to publish speak request to {}: {}", speaker, e);
        }
    }
}
/// Commands the last active listener to start listening for voice input.
///
/// This function spawns an asynchronous task to publish the start command.
#[allow(dead_code)]
pub fn listen() {
    trace!("Listen request");
    rt_spawn! {
        let listener = match get_last_listener().await {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to get listener for listen request: {e}");
                return;
            }
        };

        debug!("Publishing listen request to listener {}", listener);
        if let Err(e) = publish!(&format!("listening/{}/start", listener)) {
            error!("Failed to publish listen request to {}: {}", listener, e);
        }
    }
}
