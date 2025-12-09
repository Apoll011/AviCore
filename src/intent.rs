use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct JsonValue(pub serde_json::Value);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Default)]
pub struct YamlValue(pub serde_yaml::Value);

#[derive(Debug, Serialize, Deserialize)]
pub struct Intent {
    pub input: String,
    pub intent: Option<IntentInfo>,
    pub slots: Vec<Slot>,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct IntentInfo {
    #[serde(rename = "intentName")]
    pub intent_name: Option<String>,
    pub probability: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    #[serde(rename = "rawValue")]
    pub raw_value: String,
    pub value: SlotValue,
    pub entity: String,
    #[serde(rename = "slotName")]
    pub slot_name: String,
    pub range: SlotRange,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlotValue {
    pub kind: String, // "Custom" or Builtin type
    pub value: JsonValue, // actual resolved value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlotRange {
    pub start: usize,
    pub end: usize,
}
