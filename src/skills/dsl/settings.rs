use super::avi_dsl::ctx;
use crate::dialogue::intent::YamlValue;
use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use serde_yaml::Value as Yaml;
use std::result::Result;
use std::sync::Arc;

pub fn add_functions(module: &mut Module) {
    module.ns("setting");
    module.add(
        Arc::new("get".into()),
        get_setting,
        Dfn::nl(vec![Str], Option(Box::from(Any))),
    );
    module.add(
        Arc::new("full".into()),
        get_setting_full,
        Dfn::nl(vec![Str], Any),
    );
    module.add(Arc::new("list".into()), list_settings, Dfn::nl(vec![], Any));
    module.add(
        Arc::new("has".into()),
        has_setting,
        Dfn::nl(vec![Str], Bool),
    );
}

#[allow(non_snake_case)]
pub fn get_setting(_rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = _rt.pop()?;
    let skill_context = ctx(_rt)?;

    match skill_context.config.setting(&name) {
        Some(v) => Ok(PushVariable::push_var(&v.value)),
        None => Ok(Variable::Option(None)),
    }
}

#[allow(non_snake_case)]
pub fn get_setting_full(rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = rt.pop()?;
    let ctx = ctx(rt)?;

    Ok(PushVariable::push_var(&ctx.config.get_setting_full(&name)))
}

#[allow(non_snake_case)]
pub fn list_settings(rt: &mut Runtime) -> Result<Variable, String> {
    let ctx = ctx(rt)?;

    Ok(PushVariable::push_var(&ctx.config.list_settings()))
}

#[allow(non_snake_case)]
pub fn has_setting(rt: &mut Runtime) -> Result<Variable, String> {
    let name: String = rt.pop()?;
    let ctx = ctx(rt)?;

    Ok(PushVariable::push_var(&ctx.config.has_setting(&name)))
}
