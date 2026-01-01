use serde::{Deserialize, Serialize};
use serde_json;

/// A wrapper around `serde_json::Value` to allow implementation of traits like `Eq`, `PartialEq`, and `Hash`.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct JsonValue(pub serde_json::Value);

/// A wrapper around `serde_yaml::Value` to allow implementation of traits like `Eq`, `PartialEq`, and `Hash`.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Default)]
pub struct YamlValue(pub serde_yaml::Value);

/// Represents a recognized intent from a natural language input.
#[derive(Debug, Serialize, Deserialize)]
pub struct Intent {
    /// The original input text.
    pub input: String,
    /// Information about the recognized intent, if any.
    pub intent: Option<IntentInfo>,
    /// A list of slots (entities) extracted from the input.
    pub slots: Vec<Slot>,
}

/// Detailed information about a recognized intent.
#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct IntentInfo {
    /// The name of the intent.
    #[serde(rename = "intentName")]
    pub intent_name: Option<String>,
    /// The confidence score of the intent recognition.
    pub probability: f64,
}

/// Represents an extracted entity (slot) from the input text.
#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    /// The raw text value of the slot as it appeared in the input.
    #[serde(rename = "rawValue")]
    pub raw_value: String,
    /// The processed value of the slot.
    pub value: SlotValue,
    /// The type of entity recognized (e.g., "time", "location").
    pub entity: String,
    /// The name of the slot as defined in the intent model.
    #[serde(rename = "slotName")]
    pub slot_name: String,
    /// The range within the original input text where the slot was found.
    pub range: SlotRange,
}

/// The processed value of a slot.
#[derive(Debug, Serialize, Deserialize)]
pub struct SlotValue {
    /// The kind of value (e.g., "Custom" or a built-in type like "Instant").
    pub kind: String, 
    /// The actual resolved value of the slot.
    pub value: JsonValue,
    /// The grain of the value (optional, e.g., for time values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grain: Option<String>,
    /// The precision of the value (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
}

/// The range of characters in the original input string.
#[derive(Debug, Serialize, Deserialize)]
pub struct SlotRange {
    /// The starting character index (inclusive).
    pub start: usize,
    /// The ending character index (exclusive).
    pub end: usize,
}
