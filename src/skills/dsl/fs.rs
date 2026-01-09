use dyon::{Dfn, Module, Runtime, Variable};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::result::Result;

use log::warn;
use std::sync::Arc;

pub fn add_functions(module: &mut Module) {
    use dyon::Type::*;

    module.ns("fs");
    module.add_str("read", read, Dfn::nl(vec![Str], Option(Box::from(Str))));
    module.add_str("write", write, Dfn::nl(vec![Str, Str], Void));
    module.add_str("append", append, Dfn::nl(vec![Str, Str], Void));
    module.add_str("exists", exists, Dfn::nl(vec![Str], Bool));
    module.add_str("delete", delete, Dfn::nl(vec![Str], Bool));
    module.add_str("copy", copy, Dfn::nl(vec![Str, Str], Bool));
    module.add_str("move", _move, Dfn::nl(vec![Str, Str], Bool));
    module.add_str(
        "list_files",
        list_files,
        Dfn::nl(vec![Str], Array(Box::from(Str))),
    );
    module.add_str("mkdir", mkdir, Dfn::nl(vec![Str], Bool));
    module.add_str("basename", basename, Dfn::nl(vec![Str], Str));
    module.add_str("dirname", dirname, Dfn::nl(vec![Str], Str));
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
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|e| format!("Could not open file: {}", e))?;

    write!(file, "{}", content).map_err(|e| format!("Could not write to file: {}", e))?;

    Ok(())
}

pub fn append(_rt: &mut Runtime) -> Result<(), String> {
    let content: String = _rt.pop()?;
    let path: String = _rt.pop()?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("Could not open file: {}", e))?;

    write!(file, "{}", content).map_err(|e| format!("Could not write to file: {}", e))?;

    Ok(())
}

dyon_fn! { fn exists(path: String) -> bool {
    Path::new(&path).exists()
}}

dyon_fn! { fn delete(path: String) -> bool {
    let path = Path::new(&path);
    if path.is_dir() {
        fs::remove_dir(path).is_ok()
    } else {
        fs::remove_file(path).is_ok()
    }
}}

dyon_fn! { fn copy(src: String, dest: String) -> bool {
    fs::copy(src, dest).is_ok()
}}

dyon_fn! { fn _move(src: String, dest: String) -> bool {
    fs::rename(src, dest).is_ok()
}}

pub fn list_files(rt: &mut Runtime) -> Result<(), String> {
    let path: String = rt.pop()?;
    let mut files = vec![];
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(name) = entry.file_name().into_string() {
                    files.push(Variable::Str(Arc::new(name)));
                }
            }
        }
    }
    rt.push(Variable::Array(Arc::new(files)));
    Ok(())
}

dyon_fn! { fn mkdir(path: String) -> bool {
    fs::create_dir_all(path).is_ok()
}}

dyon_fn! { fn basename(path: String) -> String {
    Path::new(&path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string()
}}

dyon_fn! { fn dirname(path: String) -> String {
    Path::new(&path)
        .parent()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string()
}}
