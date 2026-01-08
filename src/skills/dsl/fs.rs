use dyon::{Dfn, Module, Runtime};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::result::Result;

use log::warn;
use std::sync::Arc;

pub fn add_functions(module: &mut Module) {
    use dyon::Type::*;

    module.ns("fs");
    module.add(
        Arc::new("read".into()),
        read,
        Dfn::nl(vec![Str], Option(Box::from(Str))),
    );
    module.add(
        Arc::new("write".into()),
        write,
        Dfn::nl(vec![Str, Str], Void),
    );
    module.add(
        Arc::new("append".into()),
        write,
        Dfn::nl(vec![Str, Str], Void),
    );
}

dyon_fn! { fn read(path: String) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(file) => Some(file),
        Err(e) => {
            warn!("Skill: Error reading the file content: {}", e);
            None
        },
    }
}}

pub fn write(_rt: &mut Runtime) -> Result<(), String> {
    let content: String = _rt.pop()?;
    let path: String = _rt.pop()?;

    let mut file = OpenOptions::new()
        .create(true)
        .open(path)
        .expect("Could not open log file");

    writeln!(file, "{}", content).expect("Could not write to log file");

    Ok(())
}

pub fn append(_rt: &mut Runtime) -> Result<(), String> {
    let content: String = _rt.pop()?;
    let path: String = _rt.pop()?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Could not open log file");

    writeln!(file, "{}", content).expect("Could not write to log file");

    Ok(())
}
