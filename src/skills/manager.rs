use crate::ctx::runtime;
use crate::dialogue::intent::Intent;
use crate::dialogue::languages::lang;
use crate::skills::avi_script::avi_librarymanager::initialize_avi_library;
use crate::skills::skill::Skill;
use crate::utils::{config_dir, get_all_docs_on_folder};
use avi_nlu_client::models::{self, Data, Data1Inner, Entity, InputIntent};
use log::{info, warn};
use rhai::{FnPtr, Variant};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
/// Manages the lifecycle and execution of skills.
///
/// It is responsible for loading skills from the filesystem and dispatching
/// intents to the appropriate skill.
pub struct SkillManager {
    /// A collection of loaded skills, keyed by their directory name.
    skills: HashMap<String, Skill>,
}

impl SkillManager {
    /// Creates a new `SkillManager` and loads all available skills.
    pub fn new() -> Self {
        match initialize_avi_library() {
            Ok(_) => (),
            Err(e) => {
                warn!("Failed to initialize avi_library: {}", e);
            }
        }
        info!("Creating skills manager.");
        Self {
            skills: Self::load_skills(),
        }
    }

    /// Scans the skill directory and attempts to load all skills found within.
    ///
    /// # Returns
    ///
    /// A `HashMap` containing the successfully loaded skills.
    ///
    pub fn load_skills() -> HashMap<String, Skill> {
        let mut skills = HashMap::new();

        if let Ok(_c) = runtime()
            && let Ok(entries) = fs::read_dir(config_dir().join("skills"))
        {
            info!("Searching skills path {}/skills", config_dir().display());
            for entry_dir in entries {
                let entry = match entry_dir {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                let path = entry.path();
                match Self::load(path) {
                    Ok((dir, skill)) => {
                        skills.insert(dir, skill);
                    }
                    Err(e) => {
                        warn!("{}", e)
                    }
                }
            }

            skills
        } else {
            skills
        }
    }

    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Reloading skills.");
        for skill in &mut self.skills.values_mut() {
            skill.reload()?;
        }
        Ok(())
    }

    /// Loads an individual skill from a directory path.
    ///
    /// # Arguments
    ///
    /// * `path` - The filesystem path to the skill directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the path is not a directory or if the skill initialization fails.
    fn load_skill(path: PathBuf) -> Result<(String, Skill), Box<dyn std::error::Error>> {
        if !path.is_dir() {
            return Err("Not a directory".into());
        }

        let dir_name = match path.file_name() {
            Some(v) => v,
            None => return Err("Could not get directory name".into()),
        };

        let dir_name_str = match dir_name.to_str() {
            Some(v) => v,
            None => return Err("Could not get directory name as string".into()),
        };

        Ok((dir_name_str.into(), Skill::new(dir_name_str.to_string())?))
    }

    fn load(path: PathBuf) -> Result<(String, Skill), String> {
        match Self::load_skill(path.clone()) {
            Ok((dir, mut v)) => match v.start() {
                Ok(_) => {
                    info!("Loaded skill {} from {}", v.name(), path.display());
                    Ok((dir.to_string(), v))
                }
                Err(e) => Err(format!(
                    "Error loading skill from {} ({}): {}",
                    path.display(),
                    v.name(),
                    e
                )),
            },
            Err(e) => Err(format!("Error loading skill (SkillManager): {}", e)),
        }
    }

    fn merge_entities_and_intents(
        entities: Vec<Entity>,
        intents: Vec<InputIntent>,
    ) -> Vec<Data1Inner> {
        let mut data = Vec::with_capacity(entities.len() + intents.len());

        data.extend(
            entities
                .into_iter()
                .map(|e| Data1Inner::Entity(Box::new(e))),
        );

        data.extend(intents.into_iter().map(|i| Data1Inner::Intent(Box::new(i))));

        data
    }

    pub fn get_dataset(&self) -> Data {
        let mut docs_intent: Vec<InputIntent> = Default::default();
        let mut docs_entities: Vec<Entity> = Default::default();

        for skill in self
            .skills
            .values()
            .map(|skill| skill.pathname())
            .collect::<Vec<PathBuf>>()
        {
            docs_intent.append(&mut get_all_docs_on_folder(
                skill.join("intent/intents").clone(),
                None,
                ".itent".to_string(),
            ));
            docs_entities.append(&mut get_all_docs_on_folder(
                skill.join("intent/entities"),
                None,
                ".entity".to_string(),
            ));
        }

        Data {
            language: match lang().as_str() {
                "en" => models::Lang::En,
                "pt" => models::Lang::Pt,
                _ => models::Lang::En,
            },
            data: Self::merge_entities_and_intents(docs_entities, docs_intent),
        }
    }

    /// Dispatches an intent to the corresponding skill for execution.
    ///
    /// The skill name is extracted from the full intent name (format: `skill_name@intent_name`).
    ///
    /// # Arguments
    ///
    /// * `intent` - The intent object to be executed.
    ///
    /// # Errors
    ///
    /// Returns an error if the intent is malformed or if the target skill is not found.
    pub fn run_intent(&mut self, intent: Intent) -> Result<bool, Box<dyn std::error::Error>> {
        info!("Running intent {:?}", intent);
        let intent_info = match intent.intent.clone() {
            Some(v) => v.0,
            None => return Err("Intent is not defined".into()),
        };

        let full_name = intent_info.intent_name;

        let parts: Vec<&str> = full_name.split("@").collect();
        if parts.is_empty() || parts.len() != 2 {
            return Err("Intent name must contain '@' separator".into());
        }

        let skill_name = parts[0];

        match self.skills.get_mut(skill_name) {
            Some(v) => Ok(v.run_intent(intent)?),
            None => Err(format!("Skill {} not found", skill_name).into()),
        }
    }

    #[allow(dead_code)]
    pub fn run_skill_function<T: Variant + Clone>(
        &mut self,
        skill_name: &str,
        function_name: &str,
        args: Vec<T>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match self.skills.get_mut(skill_name) {
            Some(v) => Ok(v.run_function(function_name, args)?),
            None => Err(format!("Skill {} not found", skill_name).into()),
        }
    }

    pub fn run_skill_function_ptr<T: Variant + Clone>(
        &mut self,
        skill_name: &str,
        function: FnPtr,
        args: Vec<T>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match self.skills.get_mut(skill_name) {
            Some(v) => Ok(v.run_function_ptr(function, args)?),
            None => Err(format!("Skill {} not found", skill_name).into()),
        }
    }
}
