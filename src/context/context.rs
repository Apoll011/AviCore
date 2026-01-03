use crate::dialogue::intent::JsonValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextValue {
    pub value: serde_json::Value,
    pub expires_at: Option<u64>, // timestamp in seconds
    pub created_at: u64,
}

impl ContextValue {
    pub fn new(value: serde_json::Value, ttl: Option<Duration>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let expires_at = ttl.map(|d| now + d.as_secs());

        Self {
            value,
            expires_at,
            created_at: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            return now >= expires_at;
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContextScope {
    Global,
    Skill(String),
}

impl ContextScope {
    pub fn to_string_key(&self) -> String {
        match self {
            ContextScope::Global => "global".to_string(),
            ContextScope::Skill(name) => format!("skill_{}", name),
        }
    }
}

pub struct ContextManager {
    memory_store: Arc<RwLock<HashMap<ContextScope, HashMap<String, ContextValue>>>>,
    persistence_path: PathBuf,
}

impl ContextManager {
    pub fn new<P: AsRef<Path>>(persistence_path: P) -> Self {
        let path = persistence_path.as_ref().to_path_buf();
        if !path.exists() {
            fs::create_dir_all(&path).expect("Failed to create persistence directory");
        }

        Self {
            memory_store: Arc::new(RwLock::new(HashMap::new())),
            persistence_path: path,
        }
    }

    pub fn set(
        &self,
        scope: ContextScope,
        key: String,
        value: serde_json::Value,
        ttl: Option<Duration>,
        persistent: bool,
    ) {
        let ctx_value = ContextValue::new(value, ttl);

        // Save to memory
        {
            let mut store = self.memory_store.write().unwrap();
            store
                .entry(scope.clone())
                .or_default()
                .insert(key.clone(), ctx_value.clone());
        }

        // Save to persistent storage if requested
        if persistent {
            self.save_persistent(&scope, &key, &ctx_value);
        }
    }

    pub fn get(&self, scope: &ContextScope, key: &str) -> Option<serde_json::Value> {
        // Try memory first
        if let Some(value) = self.get_memory(scope, key) {
            return Some(value);
        }

        // Try persistent storage
        if let Some(ctx_value) = self.load_persistent(scope, key) {
            if !ctx_value.is_expired() {
                // Cache it back to memory
                let mut store = self.memory_store.write().unwrap();
                store
                    .entry(scope.clone())
                    .or_default()
                    .insert(key.to_string(), ctx_value.clone());
                return Some(ctx_value.value);
            } else {
                // Clean up expired persistent value
                self.delete_persistent(scope, key);
            }
        }

        None
    }
    pub fn set_skill_save(
        &self,
        scope: ContextScope,
        key: String,
        value: JsonValue,
        ttl: f64,
        persistent: bool,
    ) {
        let ttl_duration = if ttl == 0.0 {
            None
        } else {
            Some(Duration::from_secs_f64(ttl))
        };

        self.set(
            scope,
            key,
            serde_json::json!(value),
            ttl_duration,
            persistent,
        );
    }

    pub fn remove(&self, scope: &ContextScope, key: &str) {
        self.delete_persistent(scope, key);
    }

    pub fn has(&self, scope: &ContextScope, key: &str) -> bool {
        self.get(scope, key).is_some()
    }

    pub fn get_memory(&self, scope: &ContextScope, key: &str) -> Option<serde_json::Value> {
        let store = self.memory_store.read().unwrap();
        if let Some(scope_map) = store.get(scope)
            && let Some(ctx_value) = scope_map.get(key)
            && !ctx_value.is_expired()
        {
            return Some(ctx_value.value.clone());
        }
        None
    }

    fn get_scope_path(&self, scope: &ContextScope) -> PathBuf {
        self.persistence_path.join(scope.to_string_key())
    }

    fn save_persistent(&self, scope: &ContextScope, key: &str, value: &ContextValue) {
        let scope_path = self.get_scope_path(scope);
        if !scope_path.exists() {
            fs::create_dir_all(&scope_path).expect("Failed to create scope directory");
        }

        let file_path = scope_path.join(format!("{}.json", key));
        let content = serde_json::to_string(value).expect("Failed to serialize context value");
        fs::write(file_path, content).expect("Failed to write persistent context");
    }

    fn load_persistent(&self, scope: &ContextScope, key: &str) -> Option<ContextValue> {
        let file_path = self.get_scope_path(scope).join(format!("{}.json", key));
        if file_path.exists() {
            let content = fs::read_to_string(file_path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    fn delete_persistent(&self, scope: &ContextScope, key: &str) {
        let file_path = self.get_scope_path(scope).join(format!("{}.json", key));
        if file_path.exists() {
            let _ = fs::remove_file(file_path);
        }
    }

    pub fn cleanup_expired(&self) {
        let mut store = self.memory_store.write().unwrap();
        for scope_map in store.values_mut() {
            scope_map.retain(|_, v| !v.is_expired());
        }

        // Also cleanup persistent files
        if let Ok(entries) = fs::read_dir(&self.persistence_path) {
            for entry in entries.flatten() {
                if entry.path().is_dir()
                    && let Ok(sub_entries) = fs::read_dir(entry.path())
                {
                    for sub_entry in sub_entries.flatten() {
                        if let Ok(content) = fs::read_to_string(sub_entry.path())
                            && let Ok(ctx_value) = serde_json::from_str::<ContextValue>(&content)
                            && ctx_value.is_expired()
                        {
                            let _ = fs::remove_file(sub_entry.path());
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::thread;
    use tempfile::tempdir;

    #[test]
    fn test_memory_storage() {
        let dir = tempdir().unwrap();
        let manager = ContextManager::new(dir.path());
        let scope = ContextScope::Global;
        let key = "test_key".to_string();
        let value = json!({"foo": "bar"});

        manager.set(scope.clone(), key.clone(), value.clone(), None, false);

        assert_eq!(manager.get_memory(&scope, &key), Some(value.clone()));
        assert_eq!(manager.get(&scope, &key), Some(value));
    }

    #[test]
    fn test_persistent_storage() {
        let dir = tempdir().unwrap();
        let manager = ContextManager::new(dir.path());
        let scope = ContextScope::Skill("test_skill".to_string());
        let key = "persistent_key".to_string();
        let value = json!(42);

        manager.set(scope.clone(), key.clone(), value.clone(), None, true);

        // Clear memory to force load from disk
        {
            let mut store = manager.memory_store.write().unwrap();
            store.clear();
        }

        assert_eq!(manager.get_memory(&scope, &key), None);
        assert_eq!(manager.get(&scope, &key), Some(value.clone()));
        // After get, it should be back in memory
        assert_eq!(manager.get_memory(&scope, &key), Some(value));
    }

    #[test]
    fn test_ttl_expiration() {
        let dir = tempdir().unwrap();
        let manager = ContextManager::new(dir.path());
        let scope = ContextScope::Global;
        let key = "expiring_key".to_string();
        let value = json!("will_expire");

        // Use 2 seconds TTL to avoid precision issues with seconds-based TTL
        manager.set(
            scope.clone(),
            key.clone(),
            value.clone(),
            Some(Duration::from_secs(2)),
            false,
        );

        assert_eq!(manager.get(&scope, &key), Some(value));

        thread::sleep(Duration::from_secs(3));

        assert_eq!(manager.get(&scope, &key), None);
    }

    #[test]
    fn test_scoping() {
        let dir = tempdir().unwrap();
        let manager = ContextManager::new(dir.path());
        let global_scope = ContextScope::Global;
        let skill_scope = ContextScope::Skill("my_skill".to_string());
        let key = "key".to_string();

        manager.set(
            global_scope.clone(),
            key.clone(),
            json!("global"),
            None,
            false,
        );
        manager.set(
            skill_scope.clone(),
            key.clone(),
            json!("skill"),
            None,
            false,
        );

        assert_eq!(manager.get(&global_scope, &key), Some(json!("global")));
        assert_eq!(manager.get(&skill_scope, &key), Some(json!("skill")));
    }
}
