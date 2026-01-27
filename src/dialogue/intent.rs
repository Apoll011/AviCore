use avi_nlu_client::*;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::Position;
use rhai::TypeBuilder;
use serde::{Deserialize, Serialize};
/// Represents a recognized intent from a natural language input.
#[derive(Debug, Serialize, Deserialize, Clone, CustomType)]
pub struct Intent {
    /// The original input text.
    pub input: String,
    /// Information about the recognized intent, if any.
    pub intent: Option<IntentInfo>,
    /// A list of slots (entities) extracted from the input.
    pub slots: Vec<Slot>,
}

/// Detailed information about a recognized intent.
#[derive(Debug, Serialize, Deserialize, Clone, CustomType)]
pub struct IntentInfo(pub models::Intent);

/// Represents an extracted entity (slot) from the input text.
#[derive(Debug, Serialize, Deserialize, Clone, CustomType)]
pub struct Slot(pub models::Slot);

impl From<SlotValue> for Dynamic {
    fn from(val: SlotValue) -> Self {
        Dynamic::from(val.0.value)
    }
}

/// The processed value of a slot.
#[derive(Debug, Serialize, Deserialize, Clone, CustomType)]
pub struct SlotValue(pub models::SlotValue);

/// The range of characters in the original input string.
#[derive(Debug, Serialize, Deserialize, Clone, CustomType)]
pub struct SlotRange(pub models::Range);
