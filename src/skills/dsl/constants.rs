use std::result::Result;
use std::sync::Arc;
use dyon::{Dfn, Module, Runtime, Variable};
use dyon::embed::PushVariable;
use dyon::Type::*;
use crate::dialogue::intent::YamlValue;
use serde_yaml::Value as Yaml;
use super::avi_dsl::ctx;

pub fn add_functions(module: &mut Module) {
    module.ns("constant");
    module.add(Arc::new("get".into()), get_constant, Dfn::nl(vec![Str], Any));
    module.add(Arc::new("list".into()), list_constants, Dfn::nl(vec![], Any));
    module.add(Arc::new("has".into()), has_constant, Dfn::nl(vec![Str], Any));
}

#[allow(non_snake_case)]
pub fn get_constant(_rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = _rt.pop()?;
    let skill_context = ctx(_rt)?;

    match skill_context.config.constant(&*name) {
        Some(v) => {
            Ok(PushVariable::push_var(v))
        }
        None => {
            Ok(PushVariable::push_var(&YamlValue(Yaml::Null)))
        }
    }
}

#[allow(non_snake_case)]
pub fn list_constants(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt)?;

    Ok(PushVariable::push_var(&ctx.config.list_constants()))
}

#[allow(non_snake_case)]
pub fn has_constant(rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = rt.pop()?;
    let ctx = ctx(rt)?;

    Ok(PushVariable::push_var(&ctx.config.has_constant(&*name)))
}
