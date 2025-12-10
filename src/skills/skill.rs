use std::sync::{Arc};
use dyon::{error, load, Call, FnIndex, Module, Runtime};
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
    pub fn new(name: String) -> Self {
        let config =  SkillContext::from_yaml(&*Self::skill_path(name.clone())).unwrap();
        Self {
            pathname: Self::skill_path(name.clone()),
            name: name.clone(),
            module: Self::create_module(name.clone()),
            runtime: Self::create_runtime(),
            context: config,
        }
    }

    fn skill_path(name: String) -> String {
        format!("./skills/{}", name)
    }

    fn create_module(name: String) -> Arc<Module> {
        let mut dyon_module = load_module().unwrap();

        if error(load(&format!("{}/main.avi", Self::skill_path(name.clone())), &mut dyon_module)) {
            print!("{}", format!("Error loading skill {}", name))
        } else {
            println!("{}", format!("Skill {} loaded", name))
        }

        Arc::new(dyon_module)
    }

    fn create_runtime() -> Runtime {
        Runtime::new()
    }

    pub fn start(&mut self) {
        let call = Call::new("main").arg(self.context.clone());
        let f_index = self.module.find_function(&Arc::new("main".into()), 0);

        match f_index {
            FnIndex::Loaded(_f_index) => {
                if error(call.run(&mut self.runtime, &self.module)) {
                    return;
                }
            }
            _ => {
                println!("Could not find function main");
            }
        }
    }

    pub fn format_intent_name(name: String) -> String {
        name.split("@").collect::<Vec<&str>>()[1].replace(".", "_")
    }

    pub fn run_intent(&mut self, intent: crate::intent::Intent) {
        let name =  format!("intent_{}", Self::format_intent_name(intent.intent.clone().unwrap().intent_name.unwrap())).to_string();
        let call = Call::new(&name).arg(intent).arg(self.context.clone());
        let f_index = self.module.find_function(&Arc::new(name.clone()), 0);

        match f_index {
            FnIndex::Loaded(_f_index) => {
                if error(call.run(&mut self.runtime, &self.module)) {
                    return;
                }
            }
            _ => {
                println!("Could not find function `{}`", name);
            }
        }
    }
}