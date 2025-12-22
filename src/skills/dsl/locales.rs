use std::result::Result;
use std::sync::Arc;
use dyon::{Dfn, Module, Runtime, Variable};
use dyon::embed::PushVariable;
use dyon::Type::*;
use crate::intent::YamlValue;
use serde_yaml::Value as Yaml;
use crate::ctx::RUNTIMECTX;
use super::avi_dsl::ctx;

pub fn add_functions(module: &mut Module) {
    module.ns("locale");
    module.add(Arc::new("locale".into()), locale, Dfn::nl(vec![Str], Any));
    module.add(Arc::new("list_locales".into()), list_locales, Dfn::nl(vec![Str], Any));
}

#[allow(non_snake_case)]
pub fn locale(_rt: &mut Runtime) -> Result<Variable, String> {
    let id: String = _rt.pop()?;
    let skill_context = ctx(_rt);

    match RUNTIMECTX.get() {
        Some(v) => {
            Ok(PushVariable::push_var(&skill_context.locale(&*v.lang, &*id).unwrap()))
        }
        None => {
            Ok(PushVariable::push_var(&YamlValue(Yaml::Null)))
        }
    }
}

#[allow(non_snake_case)]
pub fn list_locales(rt: &mut Runtime) -> Result<Variable, String> {
    let code: String = rt.pop()?;
    let ctx = ctx(rt);

    let Some(lang) = ctx.languages.iter().find(|l| l.code == code) else {
        return Ok(PushVariable::push_var(&Vec::<(String, YamlValue)>::new()));
    };

    let pairs: Vec<(String, YamlValue)> = lang.lang
        .iter()
        .map(|i| (i.id.clone(), i.value.clone()))
        .collect();

    Ok(PushVariable::push_var(&pairs))
}
