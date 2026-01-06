use crate::ctx::runtime;
use crate::user::{Location, QuietHours};
use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use std::result::Result;
use std::sync::Arc;

pub fn add_functions(module: &mut Module) {
    module.ns("user");
    module.add(Arc::new("name".into()), name, Dfn::nl(vec![], Str));
    module.add(Arc::new("nickname".into()), nickname, Dfn::nl(vec![], Str));
    module.add(Arc::new("id".into()), id, Dfn::nl(vec![], Str));
    module.add(
        Arc::new("location".into()),
        location,
        Dfn::nl(vec![], Object),
    );
    module.add(
        Arc::new("quiet_hours".into()),
        quiet_hours,
        Dfn::nl(vec![], Object),
    );
    module.add(Arc::new("birthday".into()), birthday, Dfn::nl(vec![], F64));
    module.add(
        Arc::new("voice_profile_id".into()),
        voice_profile_id,
        Dfn::nl(vec![], Str),
    );
    module.add(Arc::new("language".into()), language, Dfn::nl(vec![], Str));
}

dyon_obj! {Location { city, country }}
dyon_obj! {QuietHours { start, end }}

dyon_fn! {fn name() -> String {
match runtime() {
    Ok(c) => c.user.get_name(),
    Err(_) => "".to_string()
}}}

pub fn nickname(_rt: &mut Runtime) -> Result<Variable, String> {
    Ok(PushVariable::push_var(&match runtime() {
        Ok(c) => c.user.get_nickname(),
        Err(_) => None,
    }))
}

dyon_fn! {fn id() -> String {
    match runtime() {
        Ok(c) => c.user.get_id(),
        Err(_) => "".to_string()
    }
}}

pub fn location(_rt: &mut Runtime) -> Result<Variable, String> {
    Ok(PushVariable::push_var(&match runtime() {
        Ok(c) => c.user.get_location(),
        Err(_) => None,
    }))
}

pub fn quiet_hours(_rt: &mut Runtime) -> Result<Variable, String> {
    Ok(PushVariable::push_var(&match runtime() {
        Ok(c) => c.user.get_quiet_hours(),
        Err(_) => None,
    }))
}

dyon_fn! { fn birthday() -> f64 {
    match runtime() {
            Ok(c) => match c.user.get_birthday() {
                Some(timestamp) => timestamp as f64,
                None => 0.0
            },
        Err(_) => 0.0
    }
}}

pub fn voice_profile_id(_rt: &mut Runtime) -> Result<Variable, String> {
    Ok(PushVariable::push_var(&match runtime() {
        Ok(c) => c.user.get_voice_profile_id(),
        Err(_) => None,
    }))
}

dyon_fn! { fn language() -> String {
    match runtime() {
        Ok(c) => c.user.get_language(),
        Err(_) => "en".to_string()
    }
}}
