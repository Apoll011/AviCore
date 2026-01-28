use crate::ctx::runtime;
#[allow(unused_imports)]
use crate::skills::avi_script::engine::create_avi_script_engine;
use log::error;
use log::info;
#[allow(unused_imports)]
use log::warn;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Eq, PartialEq)]
pub enum EventType {
    Topic,
    Event,
}

impl EventType {
    pub fn from(string: &str) -> Option<EventType> {
        if string.eq_ignore_ascii_case("topic") {
            return Some(EventType::Topic);
        } else if string.eq_ignore_ascii_case("event") {
            return Some(EventType::Event);
        }
        None
    }
    pub fn name(&self) -> String {
        match self {
            EventType::Topic => "topic".to_string(),
            EventType::Event => "event".to_string(),
        }
    }
}

pub struct Event {
    pub event_type: EventType,
    pub event_name: String,
}

impl Event {
    pub fn string(&self) -> String {
        format!("{}:{}", self.event_type.name(), self.event_name)
    }

    pub fn get_event(event_string: String) -> Result<Event, String> {
        let event_vec = event_string.split(":").collect::<Vec<&str>>();
        if event_vec.len() != 2 {
            return Err("Expected topic/event:{name}".to_string());
        }
        Ok(Event {
            event_type: EventType::from(event_vec[0])
                .ok_or("Expected topic/event on the first hand side".to_string())?,
            event_name: event_vec[1].to_string(),
        })
    }
}

pub async fn core_id() -> Option<String> {
    match runtime() {
        Ok(c) => match c.device.get_core_id().await {
            Ok(v) => Some(v),
            Err(e) => {
                warn!("Error getting core id: {}", e);
                None
            }
        },
        Err(_) => None,
    }
}

pub fn generate_documentation(include_internal: bool) -> Result<(), Box<dyn std::error::Error>> {
    use rhai_autodocs::*;
    info!("Generating documentation");
    let engine = create_avi_script_engine(true, None)?;

    info!(
        "Got {} functions from engine",
        engine.gen_fn_signatures(include_internal).len()
    );

    let docs = export::options()
        .include_standard_packages(include_internal)
        .format_sections_with(export::SectionFormat::Tabs)
        .export(&engine)?;

    info!("Trying to create dir ./docs");
    let path = "./docs";
    std::fs::remove_dir_all(path)?;
    std::fs::create_dir_all(path)?;
    info!("Created dir ./docs");

    for (name, doc) in generate::mdbook().generate(&docs)? {
        info!("Generating doc file: {}.md", name);
        std::fs::write(
            std::path::PathBuf::from_iter([path, &format!("{}.md", &name)]),
            doc,
        )?;
    }
    info!("Generated Documentation");
    Ok(())
}

pub fn generate_dsl_definition(path: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Generating DSL definition");
    let engine = create_avi_script_engine(true, None)?;

    info!(
        "Got {} functions from engine",
        engine.gen_fn_signatures(true).len()
    );

    engine
        .definitions()
        .with_headers(true) // write headers in all files
        .include_standard_packages(true) // skip standard packages
        .write_to_dir(path.clone())?;

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rhai") {
            let mut new_path = path.clone();
            new_path.set_extension("avi");
            std::fs::rename(path, new_path)?;
        }
    }

    info!("Generated DSL definitions");
    Ok(())
}

pub fn get_all_docs_on_folder<T: for<'a> Deserialize<'a> + Clone>(
    path: PathBuf,
    _lang: Option<String>,
    _extention: String,
) -> Vec<T> {
    let mut docs: Vec<T> = Default::default();

    if let Ok(entries) = fs::read_dir(path.clone()) {
        info!("Searching path {:?}", path);
        for entry_dir in entries {
            let entry = match entry_dir {
                Ok(v) => v,
                Err(_) => continue,
            };

            let path = entry.path();

            info!("Trying: {}", path.display());

            //if path.exte(&extention) {
            //  print!("dddsds");
            match load_value_from_file::<T>(path) {
                Ok(v) => docs.push(v.clone()),
                Err(e) => error!("Error loading file: {}", e),
                //}
            }
        }
    }

    docs
}

pub fn load_value_from_file<T: for<'a> Deserialize<'a>>(path: PathBuf) -> Result<T, String> {
    let file = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Error reading file: {}", e)),
    };
    match serde_yaml::from_str(&file) {
        Ok(f) => Ok(f),
        Err(e) => Err(format!("Error parsing file: {}", e)),
    }
}

#[allow(dead_code)]
pub struct Setup {}

#[allow(dead_code, unused)]
impl Setup {
    pub fn need_to() -> bool {
        false
    }

    pub fn setup() {
        //print, ask, setup
    }

    fn config_offline(language: String, device_type: String) {
        let folders = ["config", "skills", "lang"];
    }

    pub fn download_skill(skill_id: String) {
        todo!()
    }
    pub fn download_initial_skills() {
        todo!()
    }

    pub fn download_language(lang_id: String) {
        todo!()
    }
    pub fn setup_languages() {
        todo!()
    }

    pub fn download_dashboard() {
        todo!()
    }

    pub fn is_dashboard_avaliable() -> bool {
        todo!()
    }

    pub fn create_config_folder(offline_mode: bool, language: String, device_type: String) {
        //Create config, skills
        //let mut file = File::create(&script_path)?;
        //file.write_all(content.as_bytes())?;
        //updated_scripts.push(name.clone());
    }
}
