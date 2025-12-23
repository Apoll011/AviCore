use std::result::Result;
use std::sync::Arc;
use dyon::{Dfn, Module, Runtime, Variable};
use dyon::embed::PushVariable;
use dyon::Type::*;
use chrono::{Utc, DateTime};
use humantime::parse_duration;

pub fn add_functions(module: &mut Module) {
    module.ns("time");
    module.add(Arc::new("parse_duration".into()), time_parse_duration, Dfn::nl(vec![Str], F64));
    module.add(Arc::new("format_date".into()), time_format_date, Dfn::nl(vec![F64, Str], Str));
}

#[allow(non_snake_case)]
pub fn time_parse_duration(rt: &mut Runtime) -> Result<Variable, String> {
    let s: String = rt.pop()?;
    match parse_duration(&s) {
        Ok(d) => Ok(PushVariable::push_var(&(d.as_millis() as f64))),
        Err(e) => Err(format!("Duration parse error: {}", e)),
    }
}

#[allow(non_snake_case)]
pub fn time_format_date(rt: &mut Runtime) -> Result<Variable, String> {
    let fmt: String = rt.pop()?;
    let millis: f64 = rt.pop()?; // input from DSL: time object = millis
    let dt = DateTime::<Utc>::from(std::time::UNIX_EPOCH + std::time::Duration::from_millis(millis as u64));

    match dt.format(&fmt).to_string().as_str() {
        s => Ok(PushVariable::push_var(&(s.parse::<String>().unwrap()))),
    }
}
