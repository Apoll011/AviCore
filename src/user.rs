use crate::ctx::runtime;
use crate::{get_ctx, remove_ctx, set_ctx};
use log::{debug, info, trace};
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::Position;
use rhai::TypeBuilder;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize, CustomType)]
pub struct User {
    pub id: String,
    pub profile: UserProfile,
    pub preferences: UserPreferences,
    pub voice_data: VoiceData,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, CustomType)]
pub struct UserProfile {
    pub name: String,
    pub nickname: Option<String>,
    pub language: String,
    pub timezone: String,
    pub location: Option<Location>,
    pub birthday: Option<DateTime<Utc>>, // Unix timestamp
}

#[derive(Debug, Clone, Serialize, Deserialize, CustomType)]
pub struct Location {
    pub city: Option<String>,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CustomType)]
pub struct UserPreferences {
    pub communication_style: CommunicationStyle,
    pub response_length: ResponseLength,
    pub topics_of_interest: Vec<String>,
    pub notification_preferences: NotificationPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum CommunicationStyle {
    Formal,
    Casual,
    Friendly,
    Professional,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ResponseLength {
    Concise,
    Balanced,
    Detailed,
}

#[derive(Debug, Clone, Serialize, Deserialize, CustomType)]
pub struct NotificationPreferences {
    pub quiet_hours: Option<QuietHours>,
}

#[derive(Debug, Clone, Serialize, Deserialize, CustomType)]
pub struct QuietHours {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, CustomType)]
pub struct VoiceData {
    pub voice_profile_id: Option<String>,
    pub preferred_voice_speed: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, CustomType)]
pub struct Metadata {
    pub created_at: DateTime<Utc>,       // Unix timestamp
    pub last_updated: DateTime<Utc>,     // Unix timestamp
    pub last_interaction: DateTime<Utc>, // Unix timestamp
}

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use rhai::CustomType;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserManager {
    user: Arc<RwLock<User>>,
}

#[allow(dead_code)]
impl UserManager {
    pub fn new() -> Self {
        info!("Creating user Manager.");
        Self {
            user: Arc::new(RwLock::new(Self::create_default_user())),
        }
    }

    fn create_default_user() -> User {
        trace!("Creating default user profile");
        let now = Utc::now();

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
        trace!("Saving user data to all stores");
        self.save_to_memory();
        self.save_to_device().await;
        self.save_to_persistent();
    }

    fn save_to_memory(&self) {
        trace!("Saving user data to memory context");
        set_ctx!("user", &*self.user.read());
    }

    pub async fn save_to_device(&self) {
        trace!("Saving user data to device context");
        let user = {
            let guard = self.user.read();
            guard.clone()
        };

        let _ = set_ctx!(device, "avi.user", user);
    }

    fn save_to_persistent(&self) {
        trace!("Saving user data to persistent context");
        set_ctx!("user", &*self.user.read(), persistent: true);
    }

    async fn auto_save(&self) {
        self.update_last_modified();
        self.save_all().await;
    }

    pub fn save(&self) {
        let self_clone = self.clone();
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                self_clone.auto_save().await;
            });
        });
    }

    fn update_last_modified(&self) {
        self.user.write().metadata.last_updated = Utc::now();
    }

    pub fn get_from_disk(&self) {
        if let Some(data) = get_user() {
            *self.user.write() = data;
        }
    }

    pub async fn load_from_device(&self) {
        trace!("Attempting to load user data from device");
        if let Some(user) = get_user_from_mesh().await {
            info!("Updating user data from device mesh: {}", user.id);
            set_ctx!("user", &user);
            *self.user.write() = user;
            self.save_to_memory();
            self.save_to_persistent();
            return;
        }
        debug!("No user data found on device context");
    }

    // ==================== PROFILE METHODS ====================

    pub fn get_id(&self) -> String {
        self.user.read().id.clone()
    }

    pub fn get_name(&self) -> String {
        self.user.read().profile.name.clone()
    }

    pub fn set_name(&self, name: String) {
        self.user.write().profile.name = name;
        self.save();
    }

    pub fn get_nickname(&self) -> Option<String> {
        self.user.read().profile.nickname.clone()
    }

    pub fn set_nickname(&self, nickname: Option<String>) {
        self.user.write().profile.nickname = nickname;

        self.save();
    }

    pub fn get_language(&self) -> String {
        self.user.read().profile.language.clone()
    }

    pub fn set_language(&self, language: String) {
        self.user.write().profile.language = language;

        self.save();
    }

    pub fn get_timezone(&self) -> String {
        self.user.read().profile.timezone.clone()
    }

    pub fn set_timezone(&self, timezone: String) {
        self.user.write().profile.timezone = timezone;

        self.save();
    }

    pub fn get_location(&self) -> Option<Location> {
        self.user.read().profile.location.clone()
    }

    pub fn set_location(&self, city: Option<String>, country: String) {
        self.user.write().profile.location = Some(Location { city, country });

        self.save();
    }

    pub fn remove_location(&self) {
        self.user.write().profile.location = None;

        self.save();
    }

    pub fn get_birthday(&self) -> Option<DateTime<Utc>> {
        self.user.read().profile.birthday
    }

    pub fn set_birthday(&self, date: DateTime<Utc>) {
        self.user.write().profile.birthday = Some(date);

        self.save();
    }

    pub fn remove_birthday(&self) {
        self.user.write().profile.birthday = None;

        self.save();
    }

    // ==================== PREFERENCES METHODS ====================

    pub fn get_communication_style(&self) -> CommunicationStyle {
        self.user.read().preferences.communication_style.clone()
    }

    pub fn set_communication_style(&self, style: CommunicationStyle) {
        self.user.write().preferences.communication_style = style;

        self.save();
    }

    pub fn get_response_length(&self) -> ResponseLength {
        self.user.read().preferences.response_length.clone()
    }

    pub fn set_response_length(&self, length: ResponseLength) {
        self.user.write().preferences.response_length = length;

        self.save();
    }

    pub fn get_topics_of_interest(&self) -> Vec<String> {
        self.user.read().preferences.topics_of_interest.clone()
    }

    pub fn add_topic_of_interest(&self, topic: String) {
        let mut topics_of_interest = self.user.read().preferences.topics_of_interest.clone();

        if !topics_of_interest.contains(&topic) {
            topics_of_interest.push(topic);
            self.user.write().preferences.topics_of_interest = topics_of_interest;

            self.save()
        }
    }

    pub fn remove_topic_of_interest(&self, topic: &str) {
        self.user
            .write()
            .preferences
            .topics_of_interest
            .retain(|t| t != topic);

        self.save();
    }

    pub fn clear_topics_of_interest(&self) {
        self.user.write().preferences.topics_of_interest.clear();

        self.save();
    }

    pub fn get_quiet_hours(&self) -> Option<QuietHours> {
        self.user
            .read()
            .preferences
            .notification_preferences
            .quiet_hours
            .clone()
    }

    pub fn set_quiet_hours(&self, start: String, end: String) {
        self.user
            .write()
            .preferences
            .notification_preferences
            .quiet_hours = Some(QuietHours { start, end });

        self.save();
    }

    pub fn remove_quiet_hours(&self) {
        self.user
            .write()
            .preferences
            .notification_preferences
            .quiet_hours = None;

        self.save();
    }

    // ==================== VOICE DATA METHODS ====================

    pub fn get_voice_profile_id(&self) -> Option<String> {
        self.user.read().voice_data.voice_profile_id.clone()
    }

    pub fn set_voice_profile_id(&self, id: Option<String>) {
        self.user.write().voice_data.voice_profile_id = id;

        self.save();
    }

    pub fn get_voice_speed(&self) -> f32 {
        self.user.read().voice_data.preferred_voice_speed
    }

    pub fn set_voice_speed(&self, speed: f32) {
        self.user.write().voice_data.preferred_voice_speed = speed.clamp(0.5, 2.0);

        self.save();
    }

    // ==================== METADATA METHODS ====================

    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.user.read().metadata.created_at
    }

    pub fn get_last_updated(&self) -> DateTime<Utc> {
        self.user.read().metadata.last_updated
    }

    pub fn get_last_interaction(&self) -> DateTime<Utc> {
        self.user.read().metadata.last_interaction
    }

    pub fn update_last_interaction(&self) {
        self.user.write().metadata.last_interaction = Utc::now();

        self.save();
    }

    // ==================== GENERIC METHODS ====================

    pub fn get_field(&self, path: &str) -> Option<serde_json::Value> {
        let user_json = json!(&*self.user.read());
        Self::get_nested_value(&user_json, path)
    }

    pub fn set_field(&self, path: &str, value: serde_json::Value) -> Result<(), String> {
        let mut user_json = json!(&*self.user.read());
        Self::set_nested_value(&mut user_json, path, value)?;

        let mut user = self.user.write();
        *user = serde_json::from_value(user_json)
            .map_err(|e| format!("Failed to deserialize: {}", e))?;
        drop(user);

        self.save();
        Ok(())
    }

    pub fn get_user(&self) -> User {
        self.user.read().clone()
    }

    pub fn replace_user(&self, user: User) {
        *self.user.write() = user;

        self.save();
    }

    pub async fn delete_all(&self) -> Result<(), String> {
        remove_ctx!("user")?;

        remove_ctx!(device, "avi.user")
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
                return if let Some(obj) = current.as_object_mut() {
                    obj.insert(part.to_string(), new_value);
                    Ok(())
                } else {
                    Err(format!("Cannot set field on non-object at: {}", part))
                };
            } else {
                if current.get(part).is_none() {
                    return Err(format!("Path not found: {}", part));
                }
                current = current.get_mut(part).unwrap();
            }
        }

        Err("Failed to set value".to_string())
    }
}

pub fn user_name() -> String {
    match runtime() {
        Ok(c) => c.user.get_name().to_string(),
        Err(_) => "User".to_string(),
    }
}

pub fn get_user() -> Option<User> {
    match get_ctx!("user") {
        Some(user) => serde_json::from_value::<User>(user).ok(),
        None => None,
    }
}
pub async fn get_user_from_mesh() -> Option<User> {
    match get_ctx!(device, "avi.user") {
        Some(user) => serde_json::from_value::<User>(user).ok(),
        None => None,
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
        user_manager.set_name("João Silva".to_string());
        user_manager.set_nickname(Some("JJ".to_string()));
        user_manager.set_language("pt-BR".to_string());
        user_manager.set_timezone("America/Sao_Paulo".to_string());
        user_manager.set_location(Some("São Paulo".to_string()), "Brasil".to_string());

        // Birthday como timestamp
        let birthday_timestamp = Utc::now();
        user_manager.set_birthday(birthday_timestamp);

        // Preferences
        user_manager.set_communication_style(CommunicationStyle::Casual);
        user_manager.set_response_length(ResponseLength::Detailed);
        user_manager.add_topic_of_interest("tecnologia".to_string());
        user_manager.add_topic_of_interest("música".to_string());
        user_manager.set_quiet_hours("22:00".to_string(), "08:00".to_string());

        // Voice data
        user_manager.set_voice_speed(1.2);

        // Generic field access
        if let Some(name) = user_manager.get_field("profile.name") {
            println!("Nome: {}", name);
        }

        user_manager
            .set_field("profile.nickname", json!("Johnny"))
            .unwrap();

        // Update last interaction
        user_manager.update_last_interaction();

        // Manual save (já faz auto-save em cada operação para todos os lugares)
        user_manager.save();

        // Recarregar do device ctx
        // user_manager.reload().await.unwrap(); // This will fail if runtime is not fully set up in tests

        println!("User ID: {}", user_manager.get_id());
        println!("Name: {}", user_manager.get_name());
        println!("Language: {}", user_manager.get_language());
    }
}
