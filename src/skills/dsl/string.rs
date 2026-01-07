use crate::skills::dsl::dyon_helpers::{dyon_obj_into_hashmap, hashmap_value_to_string};
use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use std::result::Result;
use std::sync::Arc;
use strfmt::strfmt;
use titlecase::Titlecase;

pub fn add_functions(module: &mut Module) {
    module.ns("string");
    module.add(Arc::new("upper".into()), upper, Dfn::nl(vec![Str], Str));
    module.add(Arc::new("lower".into()), lower, Dfn::nl(vec![Str], Str));
    module.add(Arc::new("title".into()), lower, Dfn::nl(vec![Str], Str));
    module.add(
        Arc::new("split".into()),
        split,
        Dfn::nl(vec![Str, Str], Str),
    );
    module.add(
        Arc::new("join".into()),
        join,
        Dfn::nl(vec![Array(Box::from(Str)), Str], Str),
    );
    module.add(
        Arc::new("replace".into()),
        replace,
        Dfn::nl(vec![Str, Str, Str], Str),
    );
    module.add(
        Arc::new("contains".into()),
        cointains,
        Dfn::nl(vec![Str, Str], Bool),
    );
    module.add(
        Arc::new("starts_with".into()),
        starts_with,
        Dfn::nl(vec![Str, Str], Bool),
    );
    module.add(
        Arc::new("ends_with".into()),
        ends_with,
        Dfn::nl(vec![Str, Str], Bool),
    );
    module.add(
        Arc::new("substring".into()),
        substring,
        Dfn::nl(vec![Str, F64, F64], Bool),
    );
    module.add(Arc::new("length".into()), length, Dfn::nl(vec![Str], F64));
    module.add(
        Arc::new("format".into()),
        format,
        Dfn::nl(vec![Str, Object], Str),
    );
}

dyon_fn! { fn upper(text: String) -> String {
    text.to_uppercase()
}}

dyon_fn! { fn lower(text: String) -> String {
    text.to_lowercase()
}}

dyon_fn! { fn title(text: String) -> String {
    text.titlecase()
}}

dyon_fn! {fn split(text: String, pat: String) -> Vec<String> {
     text.split(&pat).collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect()
}}

dyon_fn! {fn join(text: Vec<String>, sep: String) -> String {
    text.join(&sep)
}}

dyon_fn! {fn replace(text: String, old: String, new: String) -> String {
    text.replace(&old, &new)
}}

dyon_fn! {fn cointains(text: String, subtext: String) -> bool {
    text.contains(&subtext)
}}

dyon_fn! {fn starts_with(text: String, subtext: String) -> bool {
    text.starts_with(&subtext)
}}

dyon_fn! {fn ends_with(text: String, subtext: String) -> bool {
    text.ends_with(&subtext)
}}

dyon_fn! {fn substring(text: String, start: f64, size: f64) -> String {
    text.chars().skip(start as usize).take(size as usize).collect()
}}

dyon_fn! {fn length(text: String) -> f64 {
    text.len() as f64
}}

#[allow(non_snake_case)]
pub fn format(_rt: &mut Runtime) -> Result<Variable, String> {
    let obj = _rt.stack.pop();
    let text: String = _rt.pop()?;

    match strfmt(&text, &hashmap_value_to_string(dyon_obj_into_hashmap(obj)?)) {
        Ok(v) => Ok(PushVariable::push_var(&v)),
        Err(e) => Err(format!("{}", e)),
    }
}
