use rhai::{Dynamic, EvalAltResult, FuncRegistration, Module, Position};
use rhai::module_resolvers::StaticModuleResolver;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use log::warn;

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    FuncRegistration::new("read").set_into_module(&mut module, read);
    FuncRegistration::new("write").set_into_module(&mut module, write);
    FuncRegistration::new("append").set_into_module(&mut module, append);
    FuncRegistration::new("exists").set_into_module(&mut module, exists);
    FuncRegistration::new("delete").set_into_module(&mut module, delete);
    FuncRegistration::new("copy").set_into_module(&mut module, copy);
    FuncRegistration::new("move").set_into_module(&mut module, move_file);
    FuncRegistration::new("list_files").set_into_module(&mut module, list_files);
    FuncRegistration::new("mkdir").set_into_module(&mut module, mkdir);
    FuncRegistration::new("basename").set_into_module(&mut module, basename);
    FuncRegistration::new("dirname").set_into_module(&mut module, dirname);

    resolver.insert("fs", module);
}

fn read(path: &str) -> Result<Dynamic, Box<EvalAltResult>> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(Dynamic::from(content)),
        Err(e) => {
            warn!("Skill: Error reading the file content: {}", e);
            Ok(Dynamic::UNIT) // Return () instead of Some/None
        }
    }
}

fn write(path: &str, content: &str) -> Result<(), Box<EvalAltResult>> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|e| {
            Box::new(EvalAltResult::ErrorRuntime(
                format!("Could not open file: {}", e).into(),
                Position::NONE,
            ))
        })?;

    write!(file, "{}", content).map_err(|e| {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("Could not write to file: {}", e).into(),
            Position::NONE,
        ))
    })?;

    Ok(())
}

fn append(path: &str, content: &str) -> Result<(), Box<EvalAltResult>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| {
            Box::new(EvalAltResult::ErrorRuntime(
                format!("Could not open file: {}", e).into(),
                Position::NONE,
            ))
        })?;

    write!(file, "{}", content).map_err(|e| {
        Box::new(EvalAltResult::ErrorRuntime(
            format!("Could not write to file: {}", e).into(),
            Position::NONE,
        ))
    })?;

    Ok(())
}

fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn delete(path: &str) -> bool {
    let path = Path::new(path);
    if path.is_dir() {
        fs::remove_dir(path).is_ok()
    } else {
        fs::remove_file(path).is_ok()
    }
}

fn copy(src: &str, dest: &str) -> bool {
    fs::copy(src, dest).is_ok()
}

fn move_file(src: &str, dest: &str) -> bool {
    fs::rename(src, dest).is_ok()
}

fn list_files(path: &str) -> Result<Vec<Dynamic>, Box<EvalAltResult>> {
    let mut files = Vec::new();

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(name) = entry.file_name().into_string() {
                        files.push(Dynamic::from(name));
                    }
                }
            }
            Ok(files)
        }
        Err(e) => Err(Box::new(EvalAltResult::ErrorRuntime(
            format!("Could not read directory: {}", e).into(),
            Position::NONE,
        ))),
    }
}

fn mkdir(path: &str) -> bool {
    fs::create_dir_all(path).is_ok()
}

fn basename(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string()
}

fn dirname(path: &str) -> String {
    Path::new(path)
        .parent()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string()
}