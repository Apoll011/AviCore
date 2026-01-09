use chrono::{DateTime, Utc};
use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use humantime::parse_duration;
use std::result::Result;

pub fn add_functions(module: &mut Module) {
    module.ns("time");
    module.add_str(
        "parse_duration",
        time_parse_duration,
        Dfn::nl(vec![Str], F64),
    );
    module.add_str(
        "format_date",
        time_format_date,
        Dfn::nl(vec![F64, Str], Str),
    );
    module.add_str("now", now, Dfn::nl(vec![], F64));
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
    let dt = DateTime::<Utc>::from(
        std::time::UNIX_EPOCH + std::time::Duration::from_millis(millis as u64),
    );

    match dt.format(&fmt).to_string().as_str() {
        s => Ok(PushVariable::push_var(&(s.parse::<String>().unwrap()))),
    }
}

dyon_fn! {fn now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(val) => val.as_secs() as f64 +
                   f64::from(val.subsec_nanos()) / 1.0e9,
        Err(err) => -{
            let val = err.duration();
            val.as_secs() as f64 +
            f64::from(val.subsec_nanos()) / 1.0e9
        }
    }
}}
