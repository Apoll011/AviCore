use std::process::Command;
use dyon::Type::*;
use dyon::{Dfn, Module};
use std::result::Result;
use std::sync::Arc;
use uuid::Uuid;

pub fn add_functions(module: &mut Module) {
    module.ns("util");
    module.add(
        Arc::new("uuid".into()),
        uuid,
        Dfn::nl(vec![], Str),
    );
    module.add(
        Arc::new("timestamp".into()),
        timestamp,
        Dfn::nl(vec![], F64),
    );
    module.add(
        Arc::new("assert".into()),
        assert,
        Dfn::nl(vec![Bool, Str], Void),
    );
    module.add(
        Arc::new("cmd".into()),
        cmd,
        Dfn::nl(vec![Str], F64),
    );
    module.add(
        Arc::new("os".into()),
        os,
        Dfn::nl(vec![], Str),
    );
    module.add(
        Arc::new("env".into()),
        env,
        Dfn::nl(vec![Str, Str], Str),
    );
}

dyon_fn! { fn uuid() -> String {
    Uuid::new_v4().to_string()
}}

dyon_fn! { fn timestamp() -> f64 {
    chrono::Utc::now().timestamp() as f64
}}

pub fn assert(_rt: &mut ::dyon::Runtime) -> Result<(), String> {
    let msg: String = _rt.pop()?;
    let condition: bool = _rt.pop()?;

    if condition {
        return Err(msg);
    }
    Ok(())
}

dyon_fn! {fn cmd(cmd: String) -> f64 {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", &cmd])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect("failed to execute process")
    };
    match output.status.code() {
        Some(v) => v as f64,
        None => -1f64
    }
}}

dyon_fn! {fn os() -> String {
    std::env::consts::OS.to_string()
}}

dyon_fn! {fn env(name: String, default: String) -> String {
    std::env::var(name).unwrap_or_else(|_| default)
}}