use super::avi_dsl::ctx;
use crate::ctx::runtime;
use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use std::collections::HashMap;
use std::result::Result;
use std::sync::Arc;

pub fn add_functions(module: &mut Module) {
    module.ns("locale");
    module.add(Arc::new("get".into()), locale, Dfn::nl(vec![Str], Any));
    module.add(
        Arc::new("get_fmt".into()),
        locale_fmt,
        Dfn::nl(vec![Str, Object], Any),
    );
    module.add(
        Arc::new("list".into()),
        list_locales,
        Dfn::nl(vec![Str], Any),
    );
    module.add(Arc::new("has".into()), has_locale, Dfn::nl(vec![Str], Any));
    module.add(
        Arc::new("current".into()),
        current_lang,
        Dfn::nl(vec![], Str),
    );
}

#[allow(non_snake_case)]
pub fn locale(_rt: &mut Runtime) -> Result<Variable, String> {
    let id: String = _rt.pop()?;
    let skill_context = ctx(_rt)?;

    Ok(PushVariable::push_var(
        &skill_context
            .languages
            .locale(&*runtime().lang, &*id)
            .unwrap(),
    ))
}

#[allow(non_snake_case)]
pub fn locale_fmt(_rt: &mut Runtime) -> Result<Variable, String> {
    let obj = _rt.stack.pop();
    let id: String = _rt.pop()?;
    let skill_context = ctx(_rt)?;
    let hashmap: HashMap<String, String>;
    match obj {
        Some(Variable::Object(v)) => {
            hashmap = v
                .iter()
                .filter_map(|(k, v)| match v {
                    Variable::Str(text) => Some((k.clone().to_string(), text.as_ref().clone())),
                    Variable::F64(number, ..) => Some((k.clone().to_string(), number.to_string())),
                    Variable::Bool(bool, ..) => Some((
                        k.clone().to_string(),
                        runtime()
                            .language_system
                            .get_translation(&*bool.to_string())
                            .unwrap_or("Erro".to_string()),
                    )),
                    _ => None,
                })
                .collect();
        }
        _ => return Err(format!("Expected object, got {:?}", obj)),
    }

    Ok(PushVariable::push_var(
        &skill_context
            .languages
            .locale_fmt(&*runtime().lang, &*id, &hashmap)
            .unwrap(),
    ))
}

#[allow(non_snake_case)]
pub fn current_lang(_rt: &mut Runtime) -> Result<Variable, String> {
    Ok(PushVariable::push_var(&*runtime().lang))
}

#[allow(non_snake_case)]
pub fn list_locales(rt: &mut Runtime) -> Result<Variable, String> {
    let code: String = rt.pop()?;
    let ctx = ctx(rt)?;

    Ok(PushVariable::push_var(&ctx.languages.list(&*code)))
}

#[allow(non_snake_case)]
pub fn has_locale(rt: &mut Runtime) -> Result<Variable, String> {
    let id: String = rt.pop()?;
    let ctx = ctx(rt)?;

    Ok(PushVariable::push_var(&ctx.languages.has(&*id)))
}
