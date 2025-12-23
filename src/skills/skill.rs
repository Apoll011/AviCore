use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc};
use dyon::{error, load, load_str, Call, FnIndex, Module, Runtime};
use crate::intent::Intent;
use crate::skills::skill_context::SkillContext;
use crate::skills::dsl::avi_dsl::load_module;

pub struct Skill {
    pathname: String,
    name: String,
    module: Arc<Module>,
    runtime: Runtime,
    context: SkillContext
}

impl Skill {
    pub fn new(name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let context : SkillContext;
        match SkillContext::from_yaml(&*Self::skill_path(&name)) {
            Ok(v) => context = v,
            Err(e) => return Err(format!("Could not load skill context ({})", e.to_string()).into())
        }

        let module: Arc<Module>;
        match Self::create_module(&name, &context) {
            Ok(v) => module = v,
            Err(e) => return Err(format!("Could not load skill module ({})", e.to_string()).into())
        }

        Ok(Self {
            pathname: Self::skill_path(&name),
            name: name.clone(),
            module,
            runtime: Self::create_runtime(context.clone()),
            context,
        })
    }

    fn skill_path(name: &str) -> String {
        format!("./skills/{}", name)
    }

    fn create_module(name: &str, ctx: &SkillContext) -> Result<Arc<Module>, Box<dyn std::error::Error>> {
        let mut dyon_module;
        match load_module() {
            Some(v) => dyon_module = v,
            None => return Err("Could not load avi_dsl module".into())
        }

        let entry = ctx.info.entry.clone();


        for item in fs::read_dir( Self::skill_path(name))? {
            let item = item?;
            let path = item.path();

            let file_name = match path.file_name() {
                Some(v) => v,
                None => continue
            };

            if path.extension().and_then(|e| e.to_str()) == Some("avi") && file_name != OsStr::new(&entry)  {
                let mut m = Module::new();
                m.import_ext_prelude(&dyon_module);
                if error(load(path.to_str().unwrap(), &mut m)) {
                    return Err(format!("Error loading skill {}", name).into());
                } else {
                    dyon_module.import(&m)
                }
            }
        }

        if error(load(&format!("{}/{}", Self::skill_path(name), ctx.info.entry), &mut dyon_module)) {
            return Err(format!("Error loading skill {}", name).into());
        } else {
            println!("{}", format!("Skill {} loaded", name))
        }

        Ok(Arc::new(dyon_module))
    }

    fn create_runtime(context: SkillContext) -> Runtime {
        let mut runtime = Runtime::new();
        runtime.push(context);
        runtime
    }

    pub fn start(&mut self)  -> Result<bool, Box<dyn std::error::Error>> {
        if self.disabled() {
            return Err("Skill is disabled".into());
        }
        Ok(error(self.runtime.run(&self.module)))
    }

    pub fn format_intent_name(name: String) -> String {
        name.split("@").collect::<Vec<&str>>()[1].replace(".", "_")
    }

    pub fn run_intent(&mut self, intent: Intent) -> Result<bool, Box<dyn std::error::Error>> {
        let name;
        match intent.intent.clone() {
            Some(v) => {
                if let Some(intent_name) = v.intent_name {
                    name = format!("intent_{}", Self::format_intent_name(intent_name)).to_string();
                }
                else {
                    return Err("Intent name is not defined".into())
                }
            }
            None => return Err("Intent is not defined".into())
        }

        let call = Call::new(&name).arg(intent);
        let f_index = self.module.find_function(&Arc::new(name.clone()), 0);

        match f_index {
            FnIndex::Loaded(_f_index) => {
                Ok(error(call.run(&mut self.runtime, &self.module)))
            }
            _ => {
                Err(format!("Could not find function `{}`", name).into())
            }
        }
    }

    fn disabled(&self) -> bool {
        self.context.info.disabled
    }
}