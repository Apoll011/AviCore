use crate::dialogue::intent::Intent;
use crate::skills::avi_script::helpers::json_to_dynamic;
use rhai::plugin::*;
use rhai::{Dynamic, EvalAltResult, Map, Position};

#[export_module]
pub mod slots_module {
    /// Asserts that a slot exists in the current intent
    ///
    /// # Arguments
    /// * `intent` - The intent to check
    /// * `name` - The name of the slot
    ///
    /// # Returns
    /// Nothing or throws an error if the slot is missing
    #[rhai_fn(return_raw)]
    pub fn require(intent: Intent, name: ImmutableString) -> Result<(), Box<EvalAltResult>> {
        if intent.slots.iter().any(|s| s.slot_name == name) {
            Ok(())
        } else {
            Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Slot '{}' is required", name).into(),
                Position::NONE,
            )))
        }
    }

    pub fn exists(intent: Intent, name: ImmutableString) -> bool {
        intent.slots.iter().any(|s| s.slot_name == name)
    }

    /// Gets the value of a slot from the current intent
    ///
    /// # Arguments
    /// * `intent` - The intent to check
    /// * `name` - The name of the slot
    ///
    /// # Returns
    /// The value of the slot as a Rhai object, or UNIT if the slot is missing
    pub fn get(intent: Intent, name: ImmutableString) -> Dynamic {
        let slot = intent.slots.iter().find(|s| s.slot_name == name);
        if let Some(slot) = slot {
            json_to_dynamic(slot.value.value.clone())
        } else {
            Dynamic::UNIT
        }
    }

    /// Gets the raw text value of a slot from the current intent
    ///
    /// # Arguments
    /// * `intent` - The intent to check
    /// * `name` - The name of the slot
    ///
    /// # Returns
    /// The raw text value of the slot, or UNIT if the slot is missing
    pub fn get_raw(intent: Intent, name: ImmutableString) -> Dynamic {
        let slot = intent.slots.iter().find(|s| s.slot_name == name);
        if let Some(slot) = slot {
            Dynamic::from(slot.raw_value.clone())
        } else {
            Dynamic::UNIT
        }
    }

    /// Gets the full slot object from the current intent
    ///
    /// # Arguments
    /// * `intent` - The intent to check
    /// * `name` - The name of the slot
    ///
    /// # Returns
    /// The full slot object, or UNIT if the slot is missing
    pub fn full(intent: Intent, name: ImmutableString) -> Dynamic {
        let slot = intent.slots.iter().find(|s| s.slot_name == name);
        if let Some(slot) = slot {
            Dynamic::from(slot.clone())
        } else {
            Dynamic::UNIT
        }
    }

    /// Asserts that a slot's value is equal to a given value
    ///
    /// # Arguments
    /// * `intent` - The intent to check
    /// * `name` - The name of the slot
    /// * `val` - The value to compare against
    ///
    /// # Returns
    /// True if the slot value is equal, false otherwise
    pub fn assert_equal(intent: Intent, name: ImmutableString, val: Dynamic) -> bool {
        let slot = intent.slots.iter().find(|s| s.slot_name == name);
        if let Some(slot) = slot {
            let slot_val: Dynamic = json_to_dynamic(slot.value.value.clone());
            dynamic_eq(&slot_val, &val)
        } else {
            false
        }
    }

    /// Asserts that a slot's value is in a given list or matches a value
    ///
    /// # Arguments
    /// * `intent` - The intent to check
    /// * `name` - The name of the slot
    /// * `list` - The list of values or a single value to check against
    ///
    /// # Returns
    /// True if the slot value is in the list or matches the value, false otherwise
    pub fn assert_in(intent: Intent, name: ImmutableString, list: Dynamic) -> bool {
        let slot = intent.slots.iter().find(|s| s.slot_name == name);
        if let Some(slot) = slot {
            let slot_val: Dynamic = json_to_dynamic(slot.value.value.clone());

            if let Some(arr) = list.clone().try_cast::<Vec<Dynamic>>() {
                arr.iter().any(|v| dynamic_eq(v, &slot_val))
            } else {
                dynamic_eq(&slot_val, &list)
            }
        } else {
            false
        }
    }

    /// Asserts that a slot's string value is a key in a given map
    ///
    /// # Arguments
    /// * `intent` - The intent to check
    /// * `name` - The name of the slot
    /// * `dict` - The map to check for the key
    ///
    /// # Returns
    /// True if the slot's string value is a key in the map, false otherwise
    pub fn assert_in_dict(intent: Intent, name: ImmutableString, dict: Dynamic) -> bool {
        let slot = intent.slots.iter().find(|s| s.slot_name == name);
        if let Some(slot) = slot {
            let slot_val: Dynamic = json_to_dynamic(slot.value.value.clone());

            if let Some(s) = slot_val.try_cast::<String>() {
                if let Some(obj) = dict.try_cast::<Map>() {
                    obj.keys().any(|k| k.contains(&s))
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn dynamic_eq(a: &Dynamic, b: &Dynamic) -> bool {
    use crate::skills::avi_script::helpers::dynamic_to_json;
    match (dynamic_to_json(a.clone()), dynamic_to_json(b.clone())) {
        (Ok(ja), Ok(jb)) => ja == jb,
        _ => false,
    }
}
