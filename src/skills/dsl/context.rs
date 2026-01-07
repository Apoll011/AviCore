use crate::context::ContextScope;
use crate::ctx::runtime;
use crate::dialogue::intent::JsonValue;
use crate::skills::dsl::avi_dsl::ctx;
use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use serde_json::Value;
use std::result::Result;

use crate::{get_ctx, has_ctx, remove_ctx};
use std::sync::Arc;

pub fn add_functions(module: &mut Module) {
    module.ns("context");
    module.add(Arc::new("get".into()), get, Dfn::nl(vec![Str], Any));
    module.add(Arc::new("has".into()), has, Dfn::nl(vec![Str], Bool));
    module.add(Arc::new("remove".into()), remove, Dfn::nl(vec![Str], Void));
    module.add(
        Arc::new("set".into()),
        set,
        Dfn::nl(vec![Str, Any, Bool, F64], Void),
    );
}

#[allow(non_snake_case)]
pub fn get(rt: &mut Runtime) -> Result<Variable, String> {
    let key: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    match get_ctx!(skill: skill_name, &key).map(|v| PushVariable::push_var(&JsonValue(v))) {
        Some(v) => Ok(v),
        None => Ok(PushVariable::push_var(&JsonValue(Value::Null))),
    }
}

#[allow(non_snake_case)]
pub fn has(rt: &mut Runtime) -> Result<Variable, String> {
    let key: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    Ok(PushVariable::push_var(&has_ctx!(skill: skill_name, &key)))
}

#[allow(non_snake_case)]
pub fn remove(rt: &mut Runtime) -> Result<(), String> {
    let key: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    remove_ctx!(skill: skill_name, &key)?;

    Ok(())
}

#[allow(non_snake_case)]
pub fn set(rt: &mut Runtime) -> Result<(), String> {
    let ttl = rt.pop()?;
    let persistent = rt.pop()?;
    let to_store: JsonValue = rt.pop()?;
    let key: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    match runtime() {
        Ok(c) => c.context.set_skill_save(
            ContextScope::Skill(skill_name),
            key,
            to_store,
            ttl,
            persistent,
        ),
        Err(e) => return Err(e),
    }

    Ok(())
}
