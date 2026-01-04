use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::ctx::runtime;
use crate::{remove_ctx, set_ctx};

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
    pub created_at: i64,      // Unix timestamp
    pub last_updated: i64,    // Unix timestamp
    pub last_interaction: i64, // Unix timestamp
}

pub struct UserManager {
    user: User,
}

impl UserManager {
    pub async fn new() -> Self {
        if let Ok(value) = runtime().device.get_ctx("avi.user").await {
            if let Ok(user) = serde_json::from_value::<User>(value) {
                set_ctx!("user", &user);
                return Self { user };
            }
        }

        let user = Self::create_default_user();
        let manager = Self { user };

        manager.save_all().await;

        manager
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
                notification_preferences: NotificationPreferences {
                    quiet_hours: None,
                },
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
        self.save_to_memory();
        self.save_to_device().await;
        self.save_to_persistent().await;
    }

    fn save_to_memory(&self) {
        set_ctx!("user", &self.user);
    }

    async fn save_to_device(&self) {
        let _ = runtime().device.update_ctx("avi.user", json!(&self.user)).await;
    }

    async fn save_to_persistent(&self) {
        set_ctx!("user", &self.user, persistent: true);
    }

    async fn auto_save(&mut self) {
        self.update_last_modified();
        self.save_all().await;
    }

    pub async fn save(&self) {
        self.save_all().await;
    }

    fn update_last_modified(&mut self) {
        self.user.metadata.last_updated = chrono::Utc::now().timestamp();
    }

    // ==================== PROFILE METHODS ====================

    pub fn get_id(&self) -> &str {
        &self.user.id
    }

    pub fn get_name(&self) -> &str {
        &self.user.profile.name
    }

    pub async fn set_name(&mut self, name: String) {
        self.user.profile.name = name;
        self.auto_save().await;
    }

    pub fn get_nickname(&self) -> Option<&str> {
        self.user.profile.nickname.as_deref()
    }

    pub async fn set_nickname(&mut self, nickname: Option<String>) {
        self.user.profile.nickname = nickname;
        self.auto_save().await;
    }

    pub fn get_language(&self) -> &str {
        &self.user.profile.language
    }

    pub async fn set_language(&mut self, language: String) {
        self.user.profile.language = language;
        self.auto_save().await;
    }

    pub fn get_timezone(&self) -> &str {
        &self.user.profile.timezone
    }

    pub async fn set_timezone(&mut self, timezone: String) {
        self.user.profile.timezone = timezone;
        self.auto_save().await;
    }

    pub fn get_location(&self) -> Option<&Location> {
        self.user.profile.location.as_ref()
    }

    pub async fn set_location(&mut self, city: Option<String>, country: String) {
        self.user.profile.location = Some(Location { city, country });
        self.auto_save().await;
    }

    pub async fn remove_location(&mut self) {
        self.user.profile.location = None;
        self.auto_save().await;
    }

    pub fn get_birthday(&self) -> Option<i64> {
        self.user.profile.birthday
    }

    pub async fn set_birthday(&mut self, timestamp: i64) {
        self.user.profile.birthday = Some(timestamp);
        self.auto_save().await;
    }

    pub async fn remove_birthday(&mut self) {
        self.user.profile.birthday = None;
        self.auto_save().await;
    }

    // ==================== PREFERENCES METHODS ====================

    pub fn get_communication_style(&self) -> &CommunicationStyle {
        &self.user.preferences.communication_style
    }

    pub async fn set_communication_style(&mut self, style: CommunicationStyle) {
        self.user.preferences.communication_style = style;
        self.auto_save().await;
    }

    pub fn get_response_length(&self) -> &ResponseLength {
        &self.user.preferences.response_length
    }

    pub async fn set_response_length(&mut self, length: ResponseLength) {
        self.user.preferences.response_length = length;
        self.auto_save().await;
    }

    pub fn get_topics_of_interest(&self) -> &[String] {
        &self.user.preferences.topics_of_interest
    }

    pub async fn add_topic_of_interest(&mut self, topic: String) {
        if !self.user.preferences.topics_of_interest.contains(&topic) {
            self.user.preferences.topics_of_interest.push(topic);
            self.auto_save().await;
        }
    }

    pub async fn remove_topic_of_interest(&mut self, topic: &str) {
        self.user.preferences.topics_of_interest.retain(|t| t != topic);
        self.auto_save().await;
    }

    pub async fn clear_topics_of_interest(&mut self) {
        self.user.preferences.topics_of_interest.clear();
        self.auto_save().await;
    }

    pub fn get_quiet_hours(&self) -> Option<&QuietHours> {
        self.user.preferences.notification_preferences.quiet_hours.as_ref()
    }

    pub async fn set_quiet_hours(&mut self, start: String, end: String) {
        self.user.preferences.notification_preferences.quiet_hours =
            Some(QuietHours { start, end });
        self.auto_save().await;
    }

    pub async fn remove_quiet_hours(&mut self) {
        self.user.preferences.notification_preferences.quiet_hours = None;
        self.auto_save().await;
    }

    // ==================== VOICE DATA METHODS ====================

    pub fn get_voice_profile_id(&self) -> Option<&str> {
        self.user.voice_data.voice_profile_id.as_deref()
    }

    pub async fn set_voice_profile_id(&mut self, id: Option<String>) {
        self.user.voice_data.voice_profile_id = id;
        self.auto_save().await;
    }

    pub fn get_voice_speed(&self) -> f32 {
        self.user.voice_data.preferred_voice_speed
    }

    pub async fn set_voice_speed(&mut self, speed: f32) {
        self.user.voice_data.preferred_voice_speed = speed.clamp(0.5, 2.0);
        self.auto_save().await;
    }

    // ==================== METADATA METHODS ====================

    pub fn get_created_at(&self) -> i64 {
        self.user.metadata.created_at
    }

    pub fn get_last_updated(&self) -> i64 {
        self.user.metadata.last_updated
    }

    pub fn get_last_interaction(&self) -> i64 {
        self.user.metadata.last_interaction
    }

    pub async fn update_last_interaction(&mut self) {
        self.user.metadata.last_interaction = chrono::Utc::now().timestamp();
        self.auto_save().await;
    }

    // ==================== GENERIC METHODS ====================

    pub fn get_field(&self, path: &str) -> Option<serde_json::Value> {
        let user_json = json!(&self.user);
        Self::get_nested_value(&user_json, path)
    }

    pub async fn set_field(&mut self, path: &str, value: serde_json::Value) -> Result<(), String> {
        let mut user_json = json!(&self.user);
        Self::set_nested_value(&mut user_json, path, value)?;

        self.user = serde_json::from_value(user_json)
            .map_err(|e| format!("Failed to deserialize: {}", e))?;

        self.auto_save().await;
        Ok(())
    }

    pub fn get_user(&self) -> &User {
        &self.user
    }

    pub fn get_user_mut(&mut self) -> &mut User {
        &mut self.user
    }

    pub async fn replace_user(&mut self, user: User) {
        self.user = user;
        self.auto_save().await;
    }

    pub async fn reload(&mut self) -> Result<(), String> {
        if let Ok(value) = runtime().device.get_ctx("avi.user").await {
            if let Ok(user) = serde_json::from_value::<User>(value) {
                self.user = user;
                self.save_to_memory();
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
        let mut user_manager = UserManager::new().await;

        // Profile operations
        user_manager.set_name("João Silva".to_string()).await;
        user_manager.set_nickname(Some("JJ".to_string())).await;
        user_manager.set_language("pt-BR".to_string()).await;
        user_manager.set_timezone("America/Sao_Paulo".to_string()).await;
        user_manager.set_location(Some("São Paulo".to_string()), "Brasil".to_string()).await;

        // Birthday como timestamp
        let birthday_timestamp = chrono::Utc::now().timestamp();
        user_manager.set_birthday(birthday_timestamp).await;

        // Preferences
        user_manager.set_communication_style(CommunicationStyle::Casual).await;
        user_manager.set_response_length(ResponseLength::Detailed).await;
        user_manager.add_topic_of_interest("tecnologia".to_string()).await;
        user_manager.add_topic_of_interest("música".to_string()).await;
        user_manager.set_quiet_hours("22:00".to_string(), "08:00".to_string()).await;

        // Voice data
        user_manager.set_voice_speed(1.2).await;

        // Generic field access
        if let Some(name) = user_manager.get_field("profile.name") {
            println!("Nome: {}", name);
        }

        user_manager.set_field("profile.nickname", json!("Johnny")).await.unwrap();

        // Update last interaction
        user_manager.update_last_interaction().await;

        // Manual save (já faz auto-save em cada operação para todos os lugares)
        user_manager.save().await;

        // Recarregar do device ctx
        user_manager.reload().await.unwrap();

        println!("User ID: {}", user_manager.get_id());
        println!("Name: {}", user_manager.get_name());
        println!("Language: {}", user_manager.get_language());
    }
}