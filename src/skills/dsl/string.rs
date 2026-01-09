use crate::skills::dsl::dyon_helpers::{
    dyon_obj_into_hashmap, hashmap_value_to_string, value_to_string, variable_to_json,
};
use crate::skills::dsl::std::TINVOTS;
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

    module.add_str(
        "parse_number",
        parse_number,
        Dfn::nl(vec![Str], Option(Box::new(F64))),
    );
    module.add_str("trim", trim, Dfn::nl(vec![Str], Str));
    module.add_str("trim_left", trim_left, Dfn::nl(vec![Str], Str));
    module.add_str("trim_right", trim_right, Dfn::nl(vec![Str], Str));
    module.add_str("str", _str, Dfn::nl(vec![Any], Str));
    module.add_str("str__color", str__color, Dfn::nl(vec![Vec4], Str));
    module.add_str("chars", chars, Dfn::nl(vec![Str], Array(Box::new(Str))));
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

dyon_fn! {fn parse_number(text: Arc<String>) -> std::option::Option<f64> {text.trim().parse::<f64>().ok()}}
dyon_fn! {fn trim(v: Arc<String>) -> Arc<String> {Arc::new(v.trim().into())}}
dyon_fn! {fn trim_left(v: Arc<String>) -> Arc<String> {Arc::new(v.trim_start().into())}}
dyon_fn! {fn trim_right(v: Arc<String>) -> Arc<String> {Arc::new(v.trim_end().into())}}

pub(crate) fn _str(rt: &mut Runtime) -> Result<Variable, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Variable::Str(Arc::new(value_to_string(variable_to_json(
        &v,
    )?))))
}

dyon_fn! {fn str__color(v: dyon::Vec4) -> Arc<String> {
    let v = v.0;
    let mut buf: Vec<u8> = vec![];
    let clamp = |x| {
        if x < 0.0 { 0.0 } else if x > 1.0 { 1.0 } else { x }
    };
    let r = (clamp(v[0]) * 255.0) as usize;
    let g = (clamp(v[1]) * 255.0) as usize;
    let b = (clamp(v[2]) * 255.0) as usize;
    let a = (clamp(v[3]) * 255.0) as usize;
    let map = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
               'a', 'b', 'c', 'd', 'e', 'f'];
    let (r1, r2) = (r >> 4, r & 0xf);
    let (g1, g2) = (g >> 4, g & 0xf);
    let (b1, b2) = (b >> 4, b & 0xf);
    let (a1, a2) = (a >> 4, a & 0xf);
    buf.push(b'#');
    buf.push(map[r1] as u8); buf.push(map[r2] as u8);
    buf.push(map[g1] as u8); buf.push(map[g2] as u8);
    buf.push(map[b1] as u8); buf.push(map[b2] as u8);
    if a != 255 {
        buf.push(map[a1] as u8); buf.push(map[a2] as u8);
    }
    Arc::new(String::from_utf8(buf).unwrap())
}}

pub(crate) fn chars(rt: &mut Runtime) -> Result<Variable, String> {
    let t = rt.stack.pop().expect(TINVOTS);
    let t = match rt.get(&t) {
        &Variable::Str(ref t) => t.clone(),
        x => return Err(rt.expected_arg(0, x, "str")),
    };
    Ok(Variable::Array(Arc::new(
        t.chars()
            .map(|ch| {
                let mut s = String::new();
                s.push(ch);
                Variable::Str(Arc::new(s))
            })
            .collect::<Vec<_>>(),
    )))
}
