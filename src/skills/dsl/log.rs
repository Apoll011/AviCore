use crate::skills::dsl::avi_dsl::ctx;
use dyon::Type::{Str, Void};
use dyon::{Dfn, Module, Runtime};
use log::{debug, error, info, trace, warn};

pub fn add_functions(module: &mut Module) {
    module.ns("log");
    module.add_str("info", info, Dfn::nl(vec![Str], Void));
    module.add_str("trace", trace, Dfn::nl(vec![Str], Void));
    module.add_str("debug", debug, Dfn::nl(vec![Str], Void));
    module.add_str("warn", warn, Dfn::nl(vec![Str], Void));
    module.add_str("error", error, Dfn::nl(vec![Str], Void));
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
