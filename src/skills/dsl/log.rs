use std::sync::Arc;
use dyon::{Dfn, Module, Runtime};
use dyon::Type::{Str, Void};
use log::{debug, error, info, trace, warn};
use crate::skills::dsl::avi_dsl::ctx;

pub fn add_functions(module: &mut Module) {
    module.ns("log");
    module.add(Arc::new("info".into()), info, Dfn::nl(vec![Str], Void));
    module.add(Arc::new("trace".into()), trace, Dfn::nl(vec![Str], Void));
    module.add(Arc::new("debug".into()), debug, Dfn::nl(vec![Str], Void));
    module.add(Arc::new("warn".into()), warn, Dfn::nl(vec![Str], Void));
    module.add(Arc::new("error".into()), error, Dfn::nl(vec![Str], Void));
}

#[allow(non_snake_case)]
pub fn info(rt: &mut Runtime) -> Result<(), String> {
    let text: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    info!("Skill {} - {}", skill_name, text);
    Ok(())
}

#[allow(non_snake_case)]
pub fn trace(rt: &mut Runtime) -> Result<(), String> {
    let text: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    trace!("Skill {} - {}", skill_name, text);
    Ok(())
}

#[allow(non_snake_case)]
pub fn debug(rt: &mut Runtime) -> Result<(), String> {
    let text: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    debug!("Skill {} - {}", skill_name, text);
    Ok(())
}

#[allow(non_snake_case)]
pub fn warn(rt: &mut Runtime) -> Result<(), String> {
    let text: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    warn!("Skill {} - {}", skill_name, text);
    Ok(())
}

#[allow(non_snake_case)]
pub fn error(rt: &mut Runtime) -> Result<(), String> {
    let text: String = rt.pop()?;
    let skill_name = ctx(rt)?.info.id.clone();

    error!("Skill {} - {}", skill_name, text);
    Ok(())
}