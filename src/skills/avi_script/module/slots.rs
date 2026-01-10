use crate::dialogue::intent::Intent;
use crate::skills::avi_script::helpers::{dynamic_to_json, json_to_dynamic};
use rhai::module_resolvers::StaticModuleResolver;
use rhai::{Dynamic, EvalAltResult, FuncRegistration, Map, Module, Position};

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    FuncRegistration::new("require")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Asserts that a slot exists in the current intent",
            "/// ",
            "/// # Arguments",
            "/// * `intent` - The intent to check",
            "/// * `name` - The name of the slot",
            "/// ",
            "/// # Returns",
            "/// Nothing or throws an error if the slot is missing",
        ])
        .with_params_info(&["intent: Intent", "name: &str"])
        .set_into_module(&mut module, require);

    FuncRegistration::new("exists")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Checks if a slot exists in the current intent",
            "/// ",
            "/// # Arguments",
            "/// * `intent` - The intent to check",
            "/// * `name` - The name of the slot",
            "/// ",
            "/// # Returns",
            "/// True if the slot exists, false otherwise",
        ])
        .with_params_info(&["intent: Intent", "name: &str"])
        .set_into_module(&mut module, exists);

    FuncRegistration::new("get")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Gets the value of a slot from the current intent",
            "/// ",
            "/// # Arguments",
            "/// * `intent` - The intent to check",
            "/// * `name` - The name of the slot",
            "/// ",
            "/// # Returns",
            "/// The value of the slot as a Rhai object, or UNIT if the slot is missing",
        ])
        .with_params_info(&["intent: Intent", "name: &str"])
        .set_into_module(&mut module, get);

    FuncRegistration::new("get_raw")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Gets the raw text value of a slot from the current intent",
            "/// ",
            "/// # Arguments",
            "/// * `intent` - The intent to check",
            "/// * `name` - The name of the slot",
            "/// ",
            "/// # Returns",
            "/// The raw text value of the slot, or UNIT if the slot is missing",
        ])
        .with_params_info(&["intent: Intent", "name: &str"])
        .set_into_module(&mut module, get_raw);

    FuncRegistration::new("full")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Gets the full slot object from the current intent",
            "/// ",
            "/// # Arguments",
            "/// * `intent` - The intent to check",
            "/// * `name` - The name of the slot",
            "/// ",
            "/// # Returns",
            "/// The full slot object, or UNIT if the slot is missing",
        ])
        .with_params_info(&["intent: Intent", "name: &str"])
        .set_into_module(&mut module, full);

    FuncRegistration::new("assert_equal")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Asserts that a slot's value is equal to a given value",
            "/// ",
            "/// # Arguments",
            "/// * `intent` - The intent to check",
            "/// * `name` - The name of the slot",
            "/// * `val` - The value to compare against",
            "/// ",
            "/// # Returns",
            "/// True if the slot value is equal, false otherwise",
        ])
        .with_params_info(&["intent: Intent", "name: &str", "val: Dynamic"])
        .set_into_module(&mut module, assert_equal);

    FuncRegistration::new("assert_in")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Asserts that a slot's value is in a given list or matches a value",
            "/// ",
            "/// # Arguments",
            "/// * `intent` - The intent to check",
            "/// * `name` - The name of the slot",
            "/// * `list` - The list of values or a single value to check against",
            "/// ",
            "/// # Returns",
            "/// True if the slot value is in the list or matches the value, false otherwise",
        ])
        .with_params_info(&["intent: Intent", "name: &str", "list: Dynamic"])
        .set_into_module(&mut module, assert_in);

    FuncRegistration::new("assert_in_dict")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Asserts that a slot's string value is a key in a given map",
            "/// ",
            "/// # Arguments",
            "/// * `intent` - The intent to check",
            "/// * `name` - The name of the slot",
            "/// * `dict` - The map to check for the key",
            "/// ",
            "/// # Returns",
            "/// True if the slot's string value is a key in the map, false otherwise",
        ])
        .with_params_info(&["intent: Intent", "name: &str", "dict: Dynamic"])
        .set_into_module(&mut module, assert_in_dict);

    resolver.insert("slots", module);
}

fn require(intent: Intent, name: &str) -> Result<(), Box<EvalAltResult>> {
    if intent.slots.iter().any(|s| s.slot_name == name) {
        Ok(())
    } else {
        Err(Box::new(EvalAltResult::ErrorRuntime(
            format!("Slot '{}' is required", name).into(),
            Position::NONE,
        )))
    }
}

fn exists(intent: Intent, name: &str) -> bool {
    intent.slots.iter().any(|s| s.slot_name == name)
}

fn get(intent: Intent, name: &str) -> Dynamic {
    let slot = intent.slots.iter().find(|s| s.slot_name == name);
    if let Some(slot) = slot {
        // Convert slot.value.value to Dynamic
        // You'll need to implement this conversion based on your Intent type
        json_to_dynamic(slot.value.value.clone())
    } else {
        Dynamic::UNIT
    }
}

fn get_raw(intent: Intent, name: &str) -> Dynamic {
    let slot = intent.slots.iter().find(|s| s.slot_name == name);
    if let Some(slot) = slot {
        Dynamic::from(slot.raw_value.clone())
    } else {
        Dynamic::UNIT
    }
}

fn full(intent: Intent, name: &str) -> Dynamic {
    let slot = intent.slots.iter().find(|s| s.slot_name == name);
    if let Some(slot) = slot {
        Dynamic::from(slot.clone())
    } else {
        Dynamic::UNIT
    }
}

fn assert_equal(intent: Intent, name: &str, val: Dynamic) -> bool {
    let slot = intent.slots.iter().find(|s| s.slot_name == name);
    if let Some(slot) = slot {
        let slot_val: Dynamic = json_to_dynamic(slot.value.value.clone());
        dynamic_eq(&slot_val, &val)
    } else {
        false
    }
}

fn assert_in(intent: Intent, name: &str, list: Dynamic) -> bool {
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

fn assert_in_dict(intent: Intent, name: &str, dict: Dynamic) -> bool {
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

fn dynamic_eq(a: &Dynamic, b: &Dynamic) -> bool {
    match (dynamic_to_json(a.clone()), dynamic_to_json(b.clone())) {
        (Ok(ja), Ok(jb)) => ja == jb,
        _ => false,
    }
}
