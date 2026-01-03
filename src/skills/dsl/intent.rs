use crate::dialogue::intent::Intent;
use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use std::result::Result;
use std::sync::Arc;

pub fn add_functions(module: &mut Module) {
    module.ns("slots");
    module.add(
        Arc::new("require".into()),
        require,
        Dfn::nl(vec![Any, Str], Void),
    ); //Gets a slot name and if the slot exists dont do nothing if the sloth dont exist stop de execution by exiting or somthing
    module.add(
        Arc::new("exists".into()),
        exists,
        Dfn::nl(vec![Any, Str], Bool),
    ); //Return true or false if a slot exists
    module.add(Arc::new("get".into()), get, Dfn::nl(vec![Any, Str], Any)); //Gets the Value inside of a slot.value.value
    module.add(
        Arc::new("get_raw".into()),
        get_raw,
        Dfn::nl(vec![Any, Str], Any),
    ); //Gets the raw value inside of a slot.raw_value
    module.add(
        Arc::new("full".into()),
        full,
        Dfn::nl(vec![Any, Str], Object),
    ); //Gets the full sloth object
    module.add(
        Arc::new("assert_equal".into()),
        assert_equal,
        Dfn::nl(vec![Any, Str, Any], Bool),
    ); //Returns true or false if the value of the slot is equal to the value passed as the 2 parameter
    module.add(
        Arc::new("assert_in".into()),
        assert_in,
        Dfn::nl(vec![Any, Str, Any], Bool),
    );
    module.add(
        Arc::new("assert_in_dict".into()),
        assert_in_dict,
        Dfn::nl(vec![Any, Str, Any], Bool),
    ); //In key
}

#[allow(non_snake_case)]
pub fn require(rt: &mut Runtime) -> Result<Variable, String> {
    let name: Arc<String> = rt.pop()?;
    let intent: Intent = rt.pop()?;
    if intent.slots.iter().any(|s| s.slot_name == *name) {
        Ok(Variable::Bool(true, None))
    } else {
        Err(format!("Slot '{}' is required", name))
    }
}

#[allow(non_snake_case)]
pub fn exists(rt: &mut Runtime) -> Result<Variable, String> {
    let name: Arc<String> = rt.pop()?;
    let intent: Intent = rt.pop()?;
    Ok(Variable::Bool(
        intent.slots.iter().any(|s| s.slot_name == *name),
        None,
    ))
}

#[allow(non_snake_case)]
pub fn get(rt: &mut Runtime) -> Result<Variable, String> {
    let name: Arc<String> = rt.pop()?;
    let intent: Intent = rt.pop()?;
    let slot = intent.slots.iter().find(|s| s.slot_name == *name);
    if let Some(slot) = slot {
        Ok(slot.value.value.push_var())
    } else {
        Ok(Variable::Option(None))
    }
}

#[allow(non_snake_case)]
pub fn get_raw(rt: &mut Runtime) -> Result<Variable, String> {
    let name: Arc<String> = rt.pop()?;
    let intent: Intent = rt.pop()?;
    let slot = intent.slots.iter().find(|s| s.slot_name == *name);
    if let Some(slot) = slot {
        Ok(Variable::Str(Arc::new(slot.raw_value.clone())))
    } else {
        Ok(Variable::Option(None))
    }
}

#[allow(non_snake_case)]
pub fn full(rt: &mut Runtime) -> Result<Variable, String> {
    let name: Arc<String> = rt.pop()?;
    let intent: Intent = rt.pop()?;
    let slot = intent.slots.iter().find(|s| s.slot_name == *name);
    if let Some(slot) = slot {
        Ok(slot.push_var())
    } else {
        Ok(Variable::Option(None))
    }
}

#[allow(non_snake_case)]
pub fn assert_equal(rt: &mut Runtime) -> Result<Variable, String> {
    let val: Variable = rt.pop()?;
    let name: Arc<String> = rt.pop()?;
    let intent: Intent = rt.pop()?;
    let slot = intent.slots.iter().find(|s| s.slot_name == *name);
    if let Some(slot) = slot {
        Ok(Variable::Bool(slot.value.value.push_var() == val, None))
    } else {
        Ok(Variable::Bool(false, None))
    }
}

#[allow(non_snake_case)]
pub fn assert_in(rt: &mut Runtime) -> Result<Variable, String> {
    let list: Variable = rt.pop()?;
    let name: Arc<String> = rt.pop()?;
    let intent: Intent = rt.pop()?;
    let slot = intent.slots.iter().find(|s| s.slot_name == *name);
    if let Some(slot) = slot {
        let slot_val = slot.value.value.push_var();
        match list {
            Variable::Array(arr) => Ok(Variable::Bool(arr.iter().any(|v| v == &slot_val), None)),
            _ => Ok(Variable::Bool(slot_val == list, None)),
        }
    } else {
        Ok(Variable::Bool(false, None))
    }
}

#[allow(non_snake_case)]
pub fn assert_in_dict(rt: &mut Runtime) -> Result<Variable, String> {
    let dict: Variable = rt.pop()?;
    let name: Arc<String> = rt.pop()?;
    let intent: Intent = rt.pop()?;
    let slot = intent.slots.iter().find(|s| s.slot_name == *name);
    if let Some(slot) = slot {
        let slot_val = slot.value.value.push_var();
        if let Variable::Str(s) = slot_val {
            match dict {
                Variable::Object(obj) => Ok(Variable::Bool(obj.contains_key(&s), None)),
                _ => Ok(Variable::Bool(false, None)),
            }
        } else {
            Ok(Variable::Bool(false, None))
        }
    } else {
        Ok(Variable::Bool(false, None))
    }
}
