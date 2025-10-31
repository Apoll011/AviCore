use std::sync::Arc;
use dyon::{error, load, Call, Module, Runtime};
use serde::{Deserialize, Serialize};
use crate::skills::dsl::avi_dsl::load_module;

#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    id: String,
    name: String,
    description: String,
    entry: String,
    capabilities: Vec<String>,
    permissions: Vec<String>,
    author: String,
    version: String,
}

pub struct Skill {
    pathname: String,
    name: String,
    module: Arc<Module>,
    runtime: Runtime,
    manifest: Manifest
}

impl Skill {
    pub fn new(name: String) -> Self {
        Self {
            pathname: Self::skill_path(name.clone()),
            name: name.clone(),
            module: Self::create_module(name.clone()),
            runtime: Self::create_runtime(),
            manifest: Self::load_manifest(Self::skill_path(name.clone())).expect(
                "Could not load manifest"
            ),
        }
    }

    fn skill_path(name: String) -> String {
        format!("./skills/{}", name)
    }

    fn load_manifest(pathname: String) -> Result<Manifest, serde_json::Error> {
        let manifest_path = format!("{}/manifest.json", pathname);
        let manifest_file = std::fs::File::open(manifest_path).unwrap();
        serde_json::from_reader(manifest_file)
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
        if error(self.runtime.run(&self.module)) {
            return;
        }
    }

    pub fn format_intent_name(name: String) -> String {
        name.split("@").collect::<Vec<&str>>()[1].replace(".", "_")
    }

    pub fn run_intent(&mut self, intent: crate::intent::Intent) {
        let call = Call::new(&format!("intent_{}", Self::format_intent_name(intent.intent.clone().unwrap().intent_name.unwrap())).to_string()).arg(intent);

        if error(call.run(&mut self.runtime, &self.module)) {
            return;
        }
    }
}