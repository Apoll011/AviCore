use crate::skills::avi_script::helpers::get_skill_name;
use log::{debug, error, info, trace, warn};
use rhai::module_resolvers::StaticModuleResolver;
use rhai::{EvalAltResult, FuncRegistration, Module, NativeCallContext};

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    FuncRegistration::new("info")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Logs an informational message",
            "/// ",
            "/// # Arguments",
            "/// * `message` - The message to log",
            "/// ",
            "/// # Returns",
            "/// Nothing",
        ])
        .with_params_info(&["message: &str"])
        .set_into_module(&mut module, log_info);

    FuncRegistration::new("trace")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Logs a trace-level message for detailed debugging",
            "/// ",
            "/// # Arguments",
            "/// * `message` - The message to log",
            "/// ",
            "/// # Returns",
            "/// Nothing",
        ])
        .with_params_info(&["message: &str"])
        .set_into_module(&mut module, log_trace);

    FuncRegistration::new("debug")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Logs a debug-level message",
            "/// ",
            "/// # Arguments",
            "/// * `message` - The message to log",
            "/// ",
            "/// # Returns",
            "/// Nothing",
        ])
        .with_params_info(&["message: &str"])
        .set_into_module(&mut module, log_debug);

    FuncRegistration::new("warn")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Logs a warning message",
            "/// ",
            "/// # Arguments",
            "/// * `message` - The message to log",
            "/// ",
            "/// # Returns",
            "/// Nothing",
        ])
        .with_params_info(&["message: &str"])
        .set_into_module(&mut module, log_warn);

    FuncRegistration::new("error")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Logs an error message",
            "/// ",
            "/// # Arguments",
            "/// * `message` - The message to log",
            "/// ",
            "/// # Returns",
            "/// Nothing",
        ])
        .with_params_info(&["message: &str"])
        .set_into_module(&mut module, log_error);

    resolver.insert("log", module);
}

fn log_info(ctx: NativeCallContext, text: &str) -> Result<(), Box<EvalAltResult>> {
    let skill_name = get_skill_name(&ctx)?;
    info!("Skill {} - {}", skill_name, text);
    Ok(())
}

fn log_trace(ctx: NativeCallContext, text: &str) -> Result<(), Box<EvalAltResult>> {
    let skill_name = get_skill_name(&ctx)?;
    trace!("Skill {} - {}", skill_name, text);
    Ok(())
}

fn log_debug(ctx: NativeCallContext, text: &str) -> Result<(), Box<EvalAltResult>> {
    let skill_name = get_skill_name(&ctx)?;
    debug!("Skill {} - {}", skill_name, text);
    Ok(())
}

fn log_warn(ctx: NativeCallContext, text: &str) -> Result<(), Box<EvalAltResult>> {
    let skill_name = get_skill_name(&ctx)?;
    warn!("Skill {} - {}", skill_name, text);
    Ok(())
}

fn log_error(ctx: NativeCallContext, text: &str) -> Result<(), Box<EvalAltResult>> {
    let skill_name = get_skill_name(&ctx)?;
    error!("Skill {} - {}", skill_name, text);
    Ok(())
}
