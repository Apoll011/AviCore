use crate::dialogue::intent::JsonValue;
use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use std::result::Result;

pub fn add_functions(module: &mut Module) {
    module.ns("json");
    module.add_str("parse", json_parse, Dfn::nl(vec![Str], Any));
    module.add_str("stringify", json_stringify, Dfn::nl(vec![Any], Str));
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
