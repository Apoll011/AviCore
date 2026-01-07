use crate::ctx::runtime;
use crate::dialogue::intent::Intent;
use crate::skills::dsl::avi_dsl::load_module;
use crate::skills::skill_context::SkillContext;
use dyon::embed::PushVariable;
use dyon::{Call, FnIndex, Module, Runtime, error, load};
use std::ffi::OsStr;
use std::fs;
use std::sync::Arc;
use log::info;

/// Represents a standalone skill that can be executed by the Avi system.
///
/// A skill consists of a Dyon module, a runtime environment, and a configuration context.
pub struct Skill {
    /// The filesystem path to the skill.
    #[allow(dead_code)]
    pathname: String,
    /// The name of the skill.
    #[allow(dead_code)]
    name: String,
    /// The loaded Dyon module containing the skill's logic.
    module: Arc<Module>,
    /// The Dyon runtime used to execute the skill.
    runtime: Runtime,
    /// The configuration and state of the skill.
    context: SkillContext,
}

impl Skill {
    /// Creates a new `Skill` instance by loading its configuration and logic from the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the skill directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the skill context or module fails to load.
    pub fn new(name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let context = SkillContext::new(&Self::skill_path(&name)?)?;

        let module: Arc<Module> = match Self::create_module(&name, &context) {
            Ok(v) => v,
            Err(e) => return Err(format!("Could not load skill module ({})", e).into()),
        };

        Ok(Self {
            pathname: Self::skill_path(&name)?,
            name: name.clone(),
            module,
            runtime: Self::create_runtime(context.clone()),
            context,
        })
    }

    /// Constructs the path to a skill's directory.
    ///
    fn skill_path(name: &str) -> Result<String, String> {
        match runtime() {
            Ok(c) => Ok(format!("{}/{}", c.skill_path, name)),
            Err(e) => Err(e),
        }
    }

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
    fn create_module(
        name: &str,
        ctx: &SkillContext,
    ) -> Result<Arc<Module>, Box<dyn std::error::Error>> {
        let mut dyon_module;
        match load_module() {
            Some(v) => dyon_module = v,
            None => return Err("Could not load avi_dsl module".into()),
        }

        let entry = ctx.info.entry.clone();

        for item in fs::read_dir(Self::skill_path(name)?)? {
            let item = item?;
            let path = item.path();

            let file_name = match path.file_name() {
                Some(v) => v,
                None => continue,
            };

            if path.extension().and_then(|e| e.to_str()) == Some("avi")
                && file_name != OsStr::new(&entry)
            {
                let mut m = Module::new();
                m.import_ext_prelude(&dyon_module);
                if error(load(path.to_str().unwrap(), &mut m)) {
                    return Err(format!("Error loading skill {}", name).into());
                } else {
                    dyon_module.import(&m)
                }
            }
        }

        if error(load(
            &format!("{}/{}", Self::skill_path(name)?, ctx.info.entry),
            &mut dyon_module,
        )) {
            return Err(format!("Error loading skill {}", name).into());
        } else {
            info!("Skill {} loaded!", name)
        }

        Ok(Arc::new(dyon_module))
    }

    /// Initializes a Dyon runtime for the skill.
    fn create_runtime(context: SkillContext) -> Runtime {
        let mut runtime = Runtime::new();
        runtime.push(context);
        runtime
    }

    /// Starts the skill by running its main module.
    ///
    /// # Errors
    ///
    /// Returns an error if the skill is disabled or if the runtime fails.
    pub fn start(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if self.disabled() {
            return Err("Skill is disabled".into());
        }
        Ok(error(self.runtime.run(&self.module)))
    }

    /// Formats an intent name into a Dyon-compatible function name.
    pub fn format_intent_name(name: String) -> String {
        name.split("@").collect::<Vec<&str>>()[1].replace(".", "_")
    }

    /// Executes a specific intent within the skill's Dyon module.
    ///
    /// # Arguments
    ///
    /// * `intent` - The intent to be executed.
    ///
    /// # Errors
    ///
    /// Returns an error if the intent name is missing or if the corresponding Dyon function cannot be found.
    pub fn run_intent(&mut self, intent: Intent) -> Result<bool, Box<dyn std::error::Error>> {
        let name;
        match intent.intent.clone() {
            Some(v) => {
                if let Some(intent_name) = v.intent_name {
                    name = format!("intent_{}", Self::format_intent_name(intent_name)).to_string();
                } else {
                    return Err("Intent name is not defined".into());
                }
            }
            None => return Err("Intent is not defined".into()),
        }

        self.run_function(&name, vec![intent])
    }

    pub fn run_function<T: PushVariable>(
        &mut self,
        function_name: &str,
        args: Vec<T>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut call = Call::new(function_name);
        for arg in args {
            call = call.arg(arg);
        }
        let f_index = self
            .module
            .find_function(&Arc::new(function_name.to_string()), 0);

        match f_index {
            FnIndex::Loaded(_f_index) => Ok(error(call.run(&mut self.runtime, &self.module))),
            _ => Err(format!("Could not find function `{}`", function_name).into()),
        }
    }

    /// Checks if the skill is currently disabled.
    fn disabled(&self) -> bool {
        self.context.info.disabled
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
