use crate::ctx::runtime;

/// Retrieves the ID of the last active listener device.
/// 
/// If no specific listener is stored in the device context, it defaults to the core device ID.
/// 
/// # Errors
/// 
/// Returns an error if the context is found but cannot be parsed, or if the core ID retrieval fails.
#[allow(dead_code)]
pub async fn get_last_listener() -> Result<String, Box<dyn std::error::Error>> {
    match runtime().device.get_ctx("avi.dialogue.listener").await {
        Ok(v) => Ok(v.as_str().ok_or("Not found!")?.parse()?),
        Err(_) => Ok(runtime().device.get_core_id().await?)
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
    match runtime().device.get_ctx("avi.dialogue.speaker").await {
        Ok(v) => Ok(v.as_str().ok_or("Not found!")?.parse()?),
        Err(_) => Ok(runtime().device.get_core_id().await?)
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
pub fn speak(text: &str) {
    let text = text.to_string();

    runtime().rt.spawn(async move {
        let speaker = match get_speaker().await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("speaker error: {e}");
                return;
            }
        };

        if let Err(e) = runtime()
            .device
            .publish(&format!("speak/{}/text", speaker), text.into_bytes())
            .await
        {
            eprintln!("publish error: {e}");
        }
    });
}

/// Commands the last active listener to start listening for voice input.
/// 
/// This function spawns an asynchronous task to publish the start command.
#[allow(dead_code)]
pub fn listen() {
    runtime().rt.spawn(async move {
        let listener = match get_last_listener().await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("listener error: {e}");
                return;
            }
        };

        if let Err(e) = runtime()
            .device
            .publish(&format!("listening/{}/start", listener), Vec::new())
            .await
        {
            eprintln!("publish error: {e}");
        }
    });
}