use ::std::ffi::OsStr;
use ::std::sync::Arc;
use ::log::info;
use dyon::{error, load, Call, FnIndex, Module, Runtime};
use dyon::embed::PushVariable;

pub mod avi_dsl;
mod array;
mod color;
mod constants;
mod context;
mod crypto;
mod dialogue;
mod dyon_helpers;
mod fs;
mod intent;
mod json;
mod linear;
mod locales;
mod log;
mod math;
mod settings;
mod skill;
mod std;
mod string;
mod thread;
mod time;
mod user;
mod utils;
mod link;
mod object;

use avi_dsl::load_module;

/// Creates and loads a Dyon module for the skill, including its dependencies.
///
/// # Arguments
///
/// * `name` - The skill name.
/// * `ctx` - The skill's configuration context.
///
/// # Errors
///
/// Returns an error if any part of the module loading process fails.
pub fn create(name: &str,
              entry: &str,
              skill_path: &str,
) -> Result<Arc<Module>, Box<dyn ::std::error::Error>> {
    let mut dyon_module = load_module();

    for item in ::std::fs::read_dir(skill_path)? {
        let item = item?;
        let path = item.path();

        let file_name = match path.file_name() {
            Some(v) => v,
            None => continue,
        };

        if path.extension().and_then(|e| e.to_str()) == Some("avi")
            && file_name != OsStr::new(&entry)
        {
            let mut m = load_module();
            m.import_ext_prelude(&dyon_module);
            if error(load(path.to_str().unwrap(), &mut m)) {
                return Err(format!("Error loading skill {}", name).into());
            } else {
                dyon_module.import(&m)
            }
        }
    }

    if error(load(
        &format!("{}/{}", skill_path, entry),
        &mut dyon_module,
    )) {
        return Err(format!("Error loading skill {}", name).into());
    } else {
        info!("Skill {} loaded!", name)
    }

    Ok(Arc::new(dyon_module))
}

/// Executes a specific function within the skill's Dyon module.
///
/// # Arguments
///
/// * `function name` - The function to be executed.
/// * `args` - The function arguments.
///
/// # Errors
///
/// Returns an error if the intent name is missing or if the corresponding Dyon function cannot be found.
pub fn run_function<T: PushVariable>(
    mut runtime: &Runtime,
    module: &Arc<Module>,
    function_name: &str,
    args: Vec<T>,
) -> Result<bool, Box<dyn ::std::error::Error>> {
    let mut call = Call::new(function_name);
    for arg in args {
        call = call.arg(arg);
    }
    let f_index = module
        .find_function(&Arc::new(function_name.to_string()), 0);

    match f_index {
        FnIndex::Loaded(_f_index) => Ok(error(call.run(&mut runtime, module))),
        _ => Err(format!("Could not find function `{}`", function_name).into()),
    }
}


/// Starts the skill by running its main module.
///
/// # Errors
///
/// Returns an error if the skill is disabled or if the runtime fails.
pub fn start(mut runtime: &Runtime, module: &Arc<Module>) -> Result<bool, Box<dyn ::std::error::Error>> {
    Ok(error(runtime.run(module)))
}
