use crate::ctx::runtime;
use crate::dialogue::intent::Intent;
use crate::skills::skill::Skill;
use dyon::embed::PushVariable;
use log::{info, warn};
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

        if let Ok(c) = runtime()
            && let Ok(entries) = fs::read_dir(c.skill_path.clone())
        {
            info!("Searching skills path {}", c.skill_path);
            for entry_dir in entries {
                let entry = match entry_dir {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                let path = entry.path();

                match Self::load_skill(path.clone()) {
                    Ok((dir, mut v)) => {
                        match v.start() {
                            Ok(_) => {
                                info!("Loaded skill {} from {}", v.name(), path.display());
                                skills.insert(dir.to_string(), v);
                            }
                            Err(e) => warn!("Error loading skill from {} ({}): {}", path.display(), v.name(), e),
                        }
                    }
                    Err(e) => warn!("Error loading skill (SkillManager): {}", e),
                }
            }

            skills
        } else {
            skills
        }
    }

    pub fn reload(&mut self) {
        info!("Reloading skills.");
        self.skills = Self::load_skills();
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
            Some(v) => v,
            None => return Err("Intent is not defined".into()),
        };

        let full_name = match intent_info.intent_name {
            Some(v) => v,
            None => return Err("Intent name is not defined".into()),
        };

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

    pub fn run_skill_function<T: PushVariable>(
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
}
