use crate::dialogue::intent::JsonValue;
use crate::skills::dsl::avi_dsl::ctx;
use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use std::result::Result;
use std::sync::Arc;

pub fn add_functions(module: &mut Module) {
    module.ns("json");
    module.add(
        Arc::new("parse".into()),
        json_parse,
        Dfn::nl(vec![Str], Any),
    );
    module.add(
        Arc::new("stringify".into()),
        json_stringify,
        Dfn::nl(vec![Any], Str),
    );
    module.add(Arc::new("dir".into()), dir, Dfn::nl(vec![], Str));
}

#[allow(non_snake_case)]
pub fn json_parse(rt: &mut Runtime) -> Result<Variable, String> {
    let s: String = rt.pop()?;
    match serde_json::from_str::<JsonValue>(&s) {
        Ok(v) => Ok(PushVariable::push_var(&v)),
        Err(e) => Err(format!("JSON parse error: {}", e)),
    }
}

#[allow(non_snake_case)]
pub fn json_stringify(rt: &mut Runtime) -> Result<Variable, String> {
    let obj: JsonValue = rt.pop()?;
    match serde_json::to_string(&obj) {
        Ok(s) => Ok(PushVariable::push_var(&s)),
        Err(e) => Err(format!("JSON stringify error: {}", e)),
    }
}

#[allow(non_snake_case)]
pub fn dir(_rt: &mut Runtime) -> Result<Variable, String> {
    let skill_context = ctx(_rt)?;
    Ok(PushVariable::push_var(&skill_context.path))
}
