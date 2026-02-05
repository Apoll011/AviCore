use crate::domain::{ChatSession, SessionInfo};
use avi_p2p::StreamId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// Session Manager
// ============================================================================

pub struct SessionManager {
    sessions: RwLock<HashMap<StreamId, Arc<RwLock<ChatSession>>>>,
    session_by_id: RwLock<HashMap<String, Arc<RwLock<ChatSession>>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            session_by_id: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new chat session
    pub async fn create_session(
        &self,
        stream_id: StreamId,
        peer_id: String,
    ) -> Option<Arc<RwLock<ChatSession>>> {
        let session = Arc::new(RwLock::new(ChatSession::new(stream_id, peer_id)));

        let session_id = session.read().await.id.clone();

        self.sessions
            .write()
            .await
            .insert(stream_id, session.clone());
        self.session_by_id
            .write()
            .await
            .insert(session_id, session.clone());

        Some(session)
    }

    /// Get a session by stream ID
    pub async fn get_session_by_stream(
        &self,
        stream_id: StreamId,
    ) -> Option<Arc<RwLock<ChatSession>>> {
        self.sessions.read().await.get(&stream_id).cloned()
    }

    /// Get a session by session ID
    pub async fn get_session(&self, session_id: &str) -> Option<Arc<RwLock<ChatSession>>> {
        self.session_by_id.read().await.get(session_id).cloned()
    }

    /// Close a session
    pub async fn close_session(&self, stream_id: StreamId) {
        if let Some(session) = self.sessions.write().await.remove(&stream_id) {
            let session_id = session.read().await.id.clone();
            self.session_by_id.write().await.remove(&session_id);

            session.write().await.close();
        }
    }

    /// Get all active sessions
    pub async fn get_all_sessions(&self) -> Vec<SessionInfo> {
        let sessions = self.sessions.read().await;
        let mut result = Vec::new();

        for session in sessions.values() {
            let session_guard = session.read().await;
            result.push(SessionInfo::from(&*session_guard));
        }

        result
    }

    /// Get session count
    pub async fn session_count(&self) -> usize {
        self.sessions.read().await.len()
    }

    /// Clear all sessions
    pub async fn clear_all(&self) {
        self.sessions.write().await.clear();
        self.session_by_id.write().await.clear();
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_creation() {
        let manager = SessionManager::new();
        let stream_id = StreamId::default();
        let peer_id = "test-peer".to_string();

        let session = manager.create_session(stream_id, peer_id.clone()).await;
        assert!(session.is_some());

        let retrieved = manager.get_session_by_stream(stream_id).await;
        assert!(retrieved.is_some());

        if let Some(s) = retrieved {
            let guard = s.read().await;
            assert_eq!(guard.peer_id, peer_id);
            assert!(guard.is_active);
        }
    }

    #[tokio::test]
    async fn test_session_closure() {
        let manager = SessionManager::new();
        let stream_id = StreamId::default();
        let peer_id = "test-peer".to_string();

        manager.create_session(stream_id, peer_id).await;
        assert_eq!(manager.session_count().await, 1);

        manager.close_session(stream_id).await;
        assert_eq!(manager.session_count().await, 0);
    }
}
