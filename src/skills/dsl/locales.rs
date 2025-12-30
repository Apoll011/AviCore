use std::result::Result;
use std::sync::Arc;
use dyon::{Dfn, Module, Runtime, Variable};
use dyon::embed::PushVariable;
use dyon::Type::*;
use crate::intent::YamlValue;
use crate::ctx::{runtime};
use super::avi_dsl::ctx;

pub fn add_functions(module: &mut Module) {
    module.ns("locale");
    module.add(Arc::new("get".into()), locale, Dfn::nl(vec![Str], Any));
    module.add(Arc::new("list".into()), list_locales, Dfn::nl(vec![Str], Any));
    module.add(Arc::new("has".into()), has_locale, Dfn::nl(vec![Str], Any));
    module.add(Arc::new("current".into()), current_lang, Dfn::nl(vec![], Str));
}

#[allow(non_snake_case)]
pub fn locale(_rt: &mut Runtime) -> Result<Variable, String> {
    let id: String = _rt.pop()?;
    let skill_context = ctx(_rt)?;

    Ok(PushVariable::push_var(&skill_context.locale(&*runtime().lang, &*id).unwrap()))
}

#[allow(non_snake_case)]
pub fn current_lang(_rt: &mut Runtime) -> Result<Variable, String> {
    Ok(PushVariable::push_var(&*runtime().lang))
}


#[allow(non_snake_case)]
pub fn list_locales(rt: &mut Runtime) -> Result<Variable, String> {
    let code: String = rt.pop()?;
    let ctx = ctx(rt)?;

    let Some(lang) = ctx.languages.iter().find(|l| l.code == code) else {
        return Ok(PushVariable::push_var(&Vec::<(String, YamlValue)>::new()));
    };

    let pairs: Vec<(String, YamlValue)> = lang.lang
        .iter()
        .map(|i| (i.id.clone(), i.value.clone()))
        .collect();

    Ok(PushVariable::push_var(&pairs))
}

#[allow(non_snake_case)]
pub fn has_locale(rt: &mut Runtime) -> Result<Variable, String> {
    let id: String = rt.pop()?;
    let ctx = ctx(rt)?;

    let exists = ctx.languages.iter()
        .any(|lang| lang.lang.iter().any(|l| l.id == id));

    Ok(PushVariable::push_var(&exists))
}
