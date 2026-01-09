use super::avi_dsl::ctx;
use crate::dialogue::languages::lang;
use crate::skills::dsl::dyon_helpers::{dyon_obj_into_hashmap, hashmap_value_to_string};
use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use std::result::Result;

pub fn add_functions(module: &mut Module) {
    module.ns("locale");
    module.add_str("get", locale, Dfn::nl(vec![Str], Option(Box::from(Str))));
    module.add_str(
        "get_fmt",
        locale_fmt,
        Dfn::nl(vec![Str, Object], Option(Box::from(Str))),
    );
    module.add_str("list", list_locales, Dfn::nl(vec![Str], Any));
    module.add_str("has", has_locale, Dfn::nl(vec![Str], Bool));
    module.add_str("current", current_lang, Dfn::nl(vec![], Str));
}

#[allow(non_snake_case)]
pub fn locale(_rt: &mut Runtime) -> Result<Variable, String> {
    let id: String = _rt.pop()?;
    let skill_context = ctx(_rt)?;

    Ok(PushVariable::push_var(
        &skill_context.languages.get_translation(&id),
    ))
}

#[allow(non_snake_case)]
pub fn locale_fmt(_rt: &mut Runtime) -> Result<Variable, String> {
    let obj = _rt.stack.pop();
    let id: String = _rt.pop()?;
    let skill_context = ctx(_rt)?;

    Ok(PushVariable::push_var(&skill_context.languages.locale_fmt(
        &lang(),
        &id,
        &hashmap_value_to_string(dyon_obj_into_hashmap(obj)?),
    )))
}

#[allow(non_snake_case)]
pub fn current_lang(_rt: &mut Runtime) -> Result<Variable, String> {
    Ok(PushVariable::push_var(&lang()))
}

#[allow(non_snake_case)]
pub fn list_locales(rt: &mut Runtime) -> Result<Variable, String> {
    let code: String = rt.pop()?;
    let ctx = ctx(rt)?;

    Ok(PushVariable::push_var(&ctx.languages.list(&code)))
}

#[allow(non_snake_case)]
pub fn has_locale(rt: &mut Runtime) -> Result<Variable, String> {
    let id: String = rt.pop()?;
    let ctx = ctx(rt)?;

    Ok(PushVariable::push_var(&ctx.languages.has(&id)))
}
