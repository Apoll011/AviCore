use crate::ctx::runtime;
use crate::{get_ctx, remove_ctx, set_ctx};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub profile: UserProfile,
    pub preferences: UserPreferences,
    pub voice_data: VoiceData,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub name: String,
    pub nickname: Option<String>,
    pub language: String,
    pub timezone: String,
    pub location: Option<Location>,
    pub birthday: Option<i64>, // Unix timestamp
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub city: Option<String>,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub communication_style: CommunicationStyle,
    pub response_length: ResponseLength,
    pub topics_of_interest: Vec<String>,
    pub notification_preferences: NotificationPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Formal,
    Casual,
    Friendly,
    Professional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseLength {
    Concise,
    Balanced,
    Detailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub quiet_hours: Option<QuietHours>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuietHours {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceData {
    pub voice_profile_id: Option<String>,
    pub preferred_voice_speed: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub created_at: i64,       // Unix timestamp
    pub last_updated: i64,     // Unix timestamp
    pub last_interaction: i64, // Unix timestamp
}

use tokio::sync::RwLock;

pub struct UserManager {
    user: RwLock<User>,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            user: RwLock::new(Self::create_default_user()),
        }
    }

    fn create_default_user() -> User {
        let now = chrono::Utc::now().timestamp();

        User {
            id: uuid::Uuid::new_v4().to_string(),
            profile: UserProfile {
                name: String::from("User"),
                nickname: None,
                language: String::from("en"),
                timezone: String::from("UTC"),
                location: None,
                birthday: None,
            },
            preferences: UserPreferences {
                communication_style: CommunicationStyle::Friendly,
                response_length: ResponseLength::Balanced,
                topics_of_interest: Vec::new(),
                notification_preferences: NotificationPreferences { quiet_hours: None },
            },
            voice_data: VoiceData {
                voice_profile_id: None,
                preferred_voice_speed: 1.0,
            },
            metadata: Metadata {
                created_at: now,
                last_updated: now,
                last_interaction: now,
            },
        }
    }

    // ==================== SAVE METHODS ====================

    pub async fn save_all(&self) {
        self.save_to_memory().await;
        self.save_to_device().await;
        self.save_to_persistent().await;
    }

    async fn save_to_memory(&self) {
        set_ctx!("user", &*self.user.read().await);
    }

    pub async fn save_to_device(&self) {
        let _ = runtime()
            .device
            .update_ctx("avi.user", json!(&*self.user.read().await))
            .await;
    }

    async fn save_to_persistent(&self) {
        set_ctx!("user", &*self.user.read().await, persistent: true);
    }

    async fn auto_save(&self) {
        self.update_last_modified().await;
        self.save_all().await;
    }

    pub async fn save(&self) {
        self.save_all().await;
    }

    async fn update_last_modified(&self) {
        self.user.write().await.metadata.last_updated = chrono::Utc::now().timestamp();
    }

    pub async fn get_from_disk(&self) {
        if let Some(user) = get_ctx!("user")
            && let Ok(data) = serde_json::from_value::<User>(user)
        {
            *self.user.write().await = data;
        }
    }

    pub async fn load_from_device(&self) {
        if let Ok(value) = runtime().device.get_ctx("avi.user").await {
            if let Ok(user) = serde_json::from_value::<User>(value) {
                println!("Loaded user from device ctx: {:?}", user);
                set_ctx!("user", &user);
                *self.user.write().await = user;
                return;
            }
        }

        let user = Self::create_default_user();
        *self.user.write().await = user;
        self.save_all().await;
    }

    // ==================== PROFILE METHODS ====================

    pub async fn get_id(&self) -> String {
        self.user.read().await.id.clone()
    }

    pub async fn get_name(&self) -> String {
        self.user.read().await.profile.name.clone()
    }

    pub async fn set_name(&self, name: String) {
        self.user.write().await.profile.name = name;
        self.auto_save().await;
    }

    pub async fn get_nickname(&self) -> Option<String> {
        self.user.read().await.profile.nickname.clone()
    }

    pub async fn set_nickname(&self, nickname: Option<String>) {
        self.user.write().await.profile.nickname = nickname;
        self.auto_save().await;
    }

    pub async fn get_language(&self) -> String {
        self.user.read().await.profile.language.clone()
    }

    pub async fn set_language(&self, language: String) {
        self.user.write().await.profile.language = language;
        self.auto_save().await;
    }

    pub async fn get_timezone(&self) -> String {
        self.user.read().await.profile.timezone.clone()
    }

    pub async fn set_timezone(&self, timezone: String) {
        self.user.write().await.profile.timezone = timezone;
        self.auto_save().await;
    }

    pub async fn get_location(&self) -> Option<Location> {
        self.user.read().await.profile.location.clone()
    }

    pub async fn set_location(&self, city: Option<String>, country: String) {
        self.user.write().await.profile.location = Some(Location { city, country });
        self.auto_save().await;
    }

    pub async fn remove_location(&self) {
        self.user.write().await.profile.location = None;
        self.auto_save().await;
    }

    pub async fn get_birthday(&self) -> Option<i64> {
        self.user.read().await.profile.birthday
    }

    pub async fn set_birthday(&self, timestamp: i64) {
        self.user.write().await.profile.birthday = Some(timestamp);
        self.auto_save().await;
    }

    pub async fn remove_birthday(&self) {
        self.user.write().await.profile.birthday = None;
        self.auto_save().await;
    }

    // ==================== PREFERENCES METHODS ====================

    pub async fn get_communication_style(&self) -> CommunicationStyle {
        self.user
            .read()
            .await
            .preferences
            .communication_style
            .clone()
    }

    pub async fn set_communication_style(&self, style: CommunicationStyle) {
        self.user.write().await.preferences.communication_style = style;
        self.auto_save().await;
    }

    pub async fn get_response_length(&self) -> ResponseLength {
        self.user.read().await.preferences.response_length.clone()
    }

    pub async fn set_response_length(&self, length: ResponseLength) {
        self.user.write().await.preferences.response_length = length;
        self.auto_save().await;
    }

    pub async fn get_topics_of_interest(&self) -> Vec<String> {
        self.user
            .read()
            .await
            .preferences
            .topics_of_interest
            .clone()
    }

    pub async fn add_topic_of_interest(&self, topic: String) {
        let mut user = self.user.write().await;
        if !user.preferences.topics_of_interest.contains(&topic) {
            user.preferences.topics_of_interest.push(topic);
            drop(user);
            self.auto_save().await;
        }
    }

    pub async fn remove_topic_of_interest(&self, topic: &str) {
        self.user
            .write()
            .await
            .preferences
            .topics_of_interest
            .retain(|t| t != topic);
        self.auto_save().await;
    }

    pub async fn clear_topics_of_interest(&self) {
        self.user
            .write()
            .await
            .preferences
            .topics_of_interest
            .clear();
        self.auto_save().await;
    }

    pub async fn get_quiet_hours(&self) -> Option<QuietHours> {
        self.user
            .read()
            .await
            .preferences
            .notification_preferences
            .quiet_hours
            .clone()
    }

    pub async fn set_quiet_hours(&self, start: String, end: String) {
        self.user
            .write()
            .await
            .preferences
            .notification_preferences
            .quiet_hours = Some(QuietHours { start, end });
        self.auto_save().await;
    }

    pub async fn remove_quiet_hours(&self) {
        self.user
            .write()
            .await
            .preferences
            .notification_preferences
            .quiet_hours = None;
        self.auto_save().await;
    }

    // ==================== VOICE DATA METHODS ====================

    pub async fn get_voice_profile_id(&self) -> Option<String> {
        self.user.read().await.voice_data.voice_profile_id.clone()
    }

    pub async fn set_voice_profile_id(&self, id: Option<String>) {
        self.user.write().await.voice_data.voice_profile_id = id;
        self.auto_save().await;
    }

    pub async fn get_voice_speed(&self) -> f32 {
        self.user.read().await.voice_data.preferred_voice_speed
    }

    pub async fn set_voice_speed(&self, speed: f32) {
        self.user.write().await.voice_data.preferred_voice_speed = speed.clamp(0.5, 2.0);
        self.auto_save().await;
    }

    // ==================== METADATA METHODS ====================

    pub async fn get_created_at(&self) -> i64 {
        self.user.read().await.metadata.created_at
    }

    pub async fn get_last_updated(&self) -> i64 {
        self.user.read().await.metadata.last_updated
    }

    pub async fn get_last_interaction(&self) -> i64 {
        self.user.read().await.metadata.last_interaction
    }

    pub async fn update_last_interaction(&self) {
        self.user.write().await.metadata.last_interaction = chrono::Utc::now().timestamp();
        self.auto_save().await;
    }

    // ==================== GENERIC METHODS ====================

    pub async fn get_field(&self, path: &str) -> Option<serde_json::Value> {
        let user_json = json!(&*self.user.read().await);
        Self::get_nested_value(&user_json, path)
    }

    pub async fn set_field(&self, path: &str, value: serde_json::Value) -> Result<(), String> {
        let mut user_json = json!(&*self.user.read().await);
        Self::set_nested_value(&mut user_json, path, value)?;

        *self.user.write().await = serde_json::from_value(user_json)
            .map_err(|e| format!("Failed to deserialize: {}", e))?;

        self.auto_save().await;
        Ok(())
    }

    pub async fn get_user(&self) -> User {
        self.user.read().await.clone()
    }

    pub async fn replace_user(&self, user: User) {
        *self.user.write().await = user;
        self.auto_save().await;
    }

    pub async fn reload(&self) -> Result<(), String> {
        if let Ok(value) = runtime().device.get_ctx("avi.user").await {
            if let Ok(user) = serde_json::from_value::<User>(value) {
                *self.user.write().await = user;
                self.save_to_memory().await;
                return Ok(());
            }
        }
        Err("Failed to reload user from device ctx".to_string())
    }

    pub async fn delete_all(&self) -> Result<(), String> {
        remove_ctx!("user");

        let _ = runtime().device.delete_ctx("avi.user").await;

        Ok(())
    }

    // ==================== HELPER METHODS ====================

    fn get_nested_value(value: &serde_json::Value, path: &str) -> Option<serde_json::Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;

        for part in parts {
            current = current.get(part)?;
        }

        Some(current.clone())
    }

    fn set_nested_value(
        value: &mut serde_json::Value,
        path: &str,
        new_value: serde_json::Value,
    ) -> Result<(), String> {
        let parts: Vec<&str> = path.split('.').collect();

        if parts.is_empty() {
            return Err("Empty path".to_string());
        }

        let mut current = value;

        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                if let Some(obj) = current.as_object_mut() {
                    obj.insert(part.to_string(), new_value);
                    return Ok(());
                } else {
                    return Err(format!("Cannot set field on non-object at: {}", part));
                }
            } else {
                if !current.get(part).is_some() {
                    return Err(format!("Path not found: {}", part));
                }
                current = current.get_mut(part).unwrap();
            }
        }

        Err("Failed to set value".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn example_usage() {
        // Inicialização - busca do device ctx (mesh) → persistent → cria novo
        let user_manager = UserManager::new();

        // Profile operations
        user_manager.set_name("João Silva".to_string()).await;
        user_manager.set_nickname(Some("JJ".to_string())).await;
        user_manager.set_language("pt-BR".to_string()).await;
        user_manager
            .set_timezone("America/Sao_Paulo".to_string())
            .await;
        user_manager
            .set_location(Some("São Paulo".to_string()), "Brasil".to_string())
            .await;

        // Birthday como timestamp
        let birthday_timestamp = chrono::Utc::now().timestamp();
        user_manager.set_birthday(birthday_timestamp).await;

        // Preferences
        user_manager
            .set_communication_style(CommunicationStyle::Casual)
            .await;
        user_manager
            .set_response_length(ResponseLength::Detailed)
            .await;
        user_manager
            .add_topic_of_interest("tecnologia".to_string())
            .await;
        user_manager
            .add_topic_of_interest("música".to_string())
            .await;
        user_manager
            .set_quiet_hours("22:00".to_string(), "08:00".to_string())
            .await;

        // Voice data
        user_manager.set_voice_speed(1.2).await;

        // Generic field access
        if let Some(name) = user_manager.get_field("profile.name").await {
            println!("Nome: {}", name);
        }

        user_manager
            .set_field("profile.nickname", json!("Johnny"))
            .await
            .unwrap();

        // Update last interaction
        user_manager.update_last_interaction().await;

        // Manual save (já faz auto-save em cada operação para todos os lugares)
        user_manager.save().await;

        // Recarregar do device ctx
        // user_manager.reload().await.unwrap(); // This will fail if runtime is not fully set up in tests

        println!("User ID: {}", user_manager.get_id().await);
        println!("Name: {}", user_manager.get_name().await);
        println!("Language: {}", user_manager.get_language().await);
    }
}
