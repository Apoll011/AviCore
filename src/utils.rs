use crate::ctx::runtime;

pub async fn get_last_listener() -> Result<String, Box<dyn std::error::Error>> {
    match runtime().device.get_ctx("avi.dialogue.listener").await {
        Ok(v) => Ok(v.to_string()),
        Err(_) => Ok(runtime().device.get_core_id().await?)
    }
}

pub async fn get_speaker() -> Result<String, Box<dyn std::error::Error>> {
    match runtime().device.get_ctx("avi.dialogue.speaker").await {
        Ok(v) => Ok(v.to_string()),
        Err(_) => Ok(runtime().device.get_core_id().await?)
    }
}