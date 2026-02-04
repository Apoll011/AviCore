use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

// ============================================================================
// Typed Message Wrapper
// ============================================================================

/// A wrapper for type-safe message handling in PubSub
#[derive(Debug, Clone)]
pub struct TypedMessage<T> {
    data: Vec<u8>,
    _phantom: PhantomData<T>,
}

impl<T> TypedMessage<T>
where
    T: for<'de> Deserialize<'de>,
{
    /// Create a typed message from raw bytes
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }

    /// Deserialize the message
    pub fn deserialize(&self) -> Result<T, String> {
        serde_json::from_slice(&self.data).map_err(|e| format!("Failed to deserialize: {}", e))
    }

    /// Try to deserialize, returning None on failure
    pub fn try_deserialize(&self) -> Option<T> {
        serde_json::from_slice(&self.data).ok()
    }

    /// Get raw bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl<T> TypedMessage<T>
where
    T: Serialize,
{
    /// Create a typed message from a value
    pub fn from_value(value: &T) -> Result<Self, String> {
        let data = serde_json::to_vec(value).map_err(|e| format!("Failed to serialize: {}", e))?;

        Ok(Self {
            data,
            _phantom: PhantomData,
        })
    }
}

// ============================================================================
// Typed Subscription Handler
// ============================================================================

/// A subscription that automatically deserializes messages to a specific type
pub struct TypedSubscription<T> {
    topic: String,
    _phantom: PhantomData<T>,
}

impl<T> TypedSubscription<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    pub fn new(topic: String) -> Self {
        Self {
            topic,
            _phantom: PhantomData,
        }
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }
}

// ============================================================================
// Common Message Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonMessage {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    pub text: String,
}

impl From<String> for TextMessage {
    fn from(text: String) -> Self {
        Self { text }
    }
}

impl From<&str> for TextMessage {
    fn from(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryMessage {
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

// ============================================================================
// Message Codec Trait
// ============================================================================

/// Trait for custom message encoding/decoding
pub trait MessageCodec: Send + Sync {
    type Message;

    fn encode(&self, message: &Self::Message) -> Result<Vec<u8>, String>;
    fn decode(&self, data: &[u8]) -> Result<Self::Message, String>;
}

/// JSON codec implementation
pub struct JsonCodec<T> {
    _phantom: PhantomData<T>,
}

impl<T> JsonCodec<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for JsonCodec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MessageCodec for JsonCodec<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync,
{
    type Message = T;

    fn encode(&self, message: &Self::Message) -> Result<Vec<u8>, String> {
        serde_json::to_vec(message).map_err(|e| format!("JSON encode error: {}", e))
    }

    fn decode(&self, data: &[u8]) -> Result<Self::Message, String> {
        serde_json::from_slice(data).map_err(|e| format!("JSON decode error: {}", e))
    }
}

/// String codec for text messages
pub struct StringCodec;

impl MessageCodec for StringCodec {
    type Message = String;

    fn encode(&self, message: &Self::Message) -> Result<Vec<u8>, String> {
        Ok(message.as_bytes().to_vec())
    }

    fn decode(&self, data: &[u8]) -> Result<Self::Message, String> {
        String::from_utf8(data.to_vec()).map_err(|e| format!("UTF-8 decode error: {}", e))
    }
}

/// Binary codec (passthrough)
pub struct BinaryCodec;

impl MessageCodec for BinaryCodec {
    type Message = Vec<u8>;

    fn encode(&self, message: &Self::Message) -> Result<Vec<u8>, String> {
        Ok(message.clone())
    }

    fn decode(&self, data: &[u8]) -> Result<Self::Message, String> {
        Ok(data.to_vec())
    }
}

// ============================================================================
// Helpers
// ============================================================================

/// Parse message based on type hint
pub fn parse_typed_message<T>(data: &[u8], type_hint: &str) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    match type_hint {
        "json" => serde_json::from_slice(data).map_err(|e| format!("JSON parse error: {}", e)),
        "text" | "string" => {
            let text = String::from_utf8(data.to_vec())
                .map_err(|e| format!("UTF-8 decode error: {}", e))?;
            serde_json::from_str(&format!("\"{}\"", text))
                .map_err(|e| format!("String conversion error: {}", e))
        }
        _ => Err(format!("Unknown type hint: {}", type_hint)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestMessage {
        id: u32,
        content: String,
    }

    #[test]
    fn test_typed_message_roundtrip() {
        let original = TestMessage {
            id: 42,
            content: "Hello".to_string(),
        };

        let typed = TypedMessage::from_value(&original).unwrap();
        let deserialized = typed.deserialize().unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_json_codec() {
        let codec = JsonCodec::<TestMessage>::new();
        let message = TestMessage {
            id: 123,
            content: "Test".to_string(),
        };

        let encoded = codec.encode(&message).unwrap();
        let decoded = codec.decode(&encoded).unwrap();

        assert_eq!(message, decoded);
    }

    #[test]
    fn test_string_codec() {
        let codec = StringCodec;
        let message = "Hello, World!".to_string();

        let encoded = codec.encode(&message).unwrap();
        let decoded = codec.decode(&encoded).unwrap();

        assert_eq!(message, decoded);
    }
}
