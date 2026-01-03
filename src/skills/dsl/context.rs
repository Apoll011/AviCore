use std::result::Result;
use std::sync::Arc;
use dyon::{Dfn, Module, Runtime, Variable};
use dyon::embed::PushVariable;
use dyon::Type::*;
use sha2::{Sha256, Digest};
use hmac::{Hmac, KeyInit, Mac};
use serde_json::Value;
use crate::context::context::ContextScope;
use crate::ctx::runtime;
use crate::dialogue::intent::JsonValue;
use crate::skills::dsl::avi_dsl::ctx;

type HmacSha256 = Hmac<Sha256>;

pub fn add_functions(module: &mut Module) {
    module.ns("context");
    module.add(Arc::new("get".into()), get, Dfn::nl(vec![Str], Any));
    module.add(Arc::new("has".into()), has, Dfn::nl(vec![Str], Bool));
    module.add(Arc::new("remove".into()), has, Dfn::nl(vec![Str], Void));
    module.add(Arc::new("set".into()), set, Dfn::nl(vec![Str, Any, Bool, F64], Void));
}

#[allow(non_snake_case)]
pub fn get(rt: &mut Runtime) -> Result<Variable, String> {
    let key: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.name.clone();

    match runtime().context.get(&ContextScope::Skill(skill_name), &key).map(|v| PushVariable::push_var(&JsonValue(v))) {
        Some(v) => Ok(v),
        None => Ok(PushVariable::push_var(&JsonValue(Value::Null)))
    }
}

#[allow(non_snake_case)]
pub fn has(rt: &mut Runtime) -> Result<Variable, String> {
    let key: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.name.clone();

    Ok(PushVariable::push_var(&runtime().context.has(&ContextScope::Skill(skill_name), &key)))
}

#[allow(non_snake_case)]
pub fn has(rt: &mut Runtime) -> Result<(), String> {
    let key: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.name.clone();

    runtime().context.remove(&ContextScope::Skill(skill_name), &key);

    Ok(())
}

#[allow(non_snake_case)]
pub fn set(rt: &mut Runtime) -> Result<(), String> {
    let ttl = rt.pop()?;
    let persistent = rt.pop()?;
    let to_store: JsonValue = rt.pop()?;
    let key: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.name.clone();

    runtime().context.set_skill_save(ContextScope::Skill(skill_name), key, to_store, ttl, persistent);

    Ok(())
}
