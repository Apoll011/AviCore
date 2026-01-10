use rhai::{Dynamic, EvalAltResult, FuncRegistration, Module, Position};
use rhai::module_resolvers::StaticModuleResolver;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use log::warn;

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    FuncRegistration::new("read")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Reads the entire contents of a file as a string",
            "/// ",
            "/// # Arguments",
            "/// * `path` - The path to the file to read",
            "/// ",
            "/// # Returns",
            "/// The file contents as a string, or UNIT if the file could not be read"
        ])
        .with_params_info(&["path: &str"])
        .set_into_module(&mut module, read);

    FuncRegistration::new("write")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Writes a string to a file, overwriting its contents",
            "/// ",
            "/// # Arguments",
            "/// * `path` - The path to the file to write",
            "/// * `content` - The string to write to the file",
            "/// ",
            "/// # Returns",
            "/// Nothing"
        ])
        .with_params_info(&["path: &str", "content: &str"])
        .set_into_module(&mut module, write);

    FuncRegistration::new("append")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Appends a string to the end of a file",
            "/// ",
            "/// # Arguments",
            "/// * `path` - The path to the file to append to",
            "/// * `content` - The string to append to the file",
            "/// ",
            "/// # Returns",
            "/// Nothing"
        ])
        .with_params_info(&["path: &str", "content: &str"])
        .set_into_module(&mut module, append);

    FuncRegistration::new("exists")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Checks if a path exists",
            "/// ",
            "/// # Arguments",
            "/// * `path` - The path to check",
            "/// ",
            "/// # Returns",
            "/// True if the path exists, false otherwise"
        ])
        .with_params_info(&["path: &str"])
        .set_into_module(&mut module, exists);

    FuncRegistration::new("delete")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Deletes a file or an empty directory",
            "/// ",
            "/// # Arguments",
            "/// * `path` - The path to delete",
            "/// ",
            "/// # Returns",
            "/// True if the deletion was successful, false otherwise"
        ])
        .with_params_info(&["path: &str"])
        .set_into_module(&mut module, delete);

    FuncRegistration::new("copy")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Copies a file from one path to another",
            "/// ",
            "/// # Arguments",
            "/// * `src` - The source path",
            "/// * `dest` - The destination path",
            "/// ",
            "/// # Returns",
            "/// True if the copy was successful, false otherwise"
        ])
        .with_params_info(&["src: &str", "dest: &str"])
        .set_into_module(&mut module, copy);

    FuncRegistration::new("move")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Moves or renames a file or directory",
            "/// ",
            "/// # Arguments",
            "/// * `src` - The source path",
            "/// * `dest` - The destination path",
            "/// ",
            "/// # Returns",
            "/// True if the move was successful, false otherwise"
        ])
        .with_params_info(&["src: &str", "dest: &str"])
        .set_into_module(&mut module, move_file);

    FuncRegistration::new("list_files")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Lists the names of files and directories in a given path",
            "/// ",
            "/// # Arguments",
            "/// * `path` - The path to list",
            "/// ",
            "/// # Returns",
            "/// A list of file and directory names"
        ])
        .with_params_info(&["path: &str"])
        .set_into_module(&mut module, list_files);

    FuncRegistration::new("mkdir")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Creates a directory and all its parent directories if they don't exist",
            "/// ",
            "/// # Arguments",
            "/// * `path` - The path of the directory to create",
            "/// ",
            "/// # Returns",
            "/// True if the directory was created successfully, false otherwise"
        ])
        .with_params_info(&["path: &str"])
        .set_into_module(&mut module, mkdir);

    FuncRegistration::new("basename")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Gets the last component of a path",
            "/// ",
            "/// # Arguments",
            "/// * `path` - The path to process",
            "/// ",
            "/// # Returns",
            "/// The last component of the path"
        ])
        .with_params_info(&["path: &str"])
        .set_into_module(&mut module, basename);

    FuncRegistration::new("dirname")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Gets the parent directory of a path",
            "/// ",
            "/// # Arguments",
            "/// * `path` - The path to process",
            "/// ",
            "/// # Returns",
            "/// The parent directory of the path"
        ])
        .with_params_info(&["path: &str"])
        .set_into_module(&mut module, dirname);

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
