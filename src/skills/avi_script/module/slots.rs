use crate::dialogue::intent::Intent;
use rhai::{Dynamic, EvalAltResult, FuncRegistration, Module, Position, Map};
use rhai::module_resolvers::StaticModuleResolver;
use crate::skills::avi_script::helpers::{json_to_dynamic, dynamic_to_json};

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    FuncRegistration::new("require").set_into_module(&mut module, require);
    FuncRegistration::new("exists").set_into_module(&mut module, exists);
    FuncRegistration::new("get").set_into_module(&mut module, get);
    FuncRegistration::new("get_raw").set_into_module(&mut module, get_raw);
    FuncRegistration::new("full").set_into_module(&mut module, full);
    FuncRegistration::new("assert_equal").set_into_module(&mut module, assert_equal);
    FuncRegistration::new("assert_in").set_into_module(&mut module, assert_in);
    FuncRegistration::new("assert_in_dict").set_into_module(&mut module, assert_in_dict);

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
                obj.keys().any(|k| {k.contains(&s)})
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