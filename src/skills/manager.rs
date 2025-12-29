use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::ctx::{runtime};
use crate::intent::Intent;
use crate::skills::skill::Skill;

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
    /// TODO: Consider adding a way to reload skills without restarting the application.
    pub fn load_skills() -> HashMap<String, Skill> {
        let mut skills = HashMap::new();

        let entries;
        match fs::read_dir(runtime().skill_path.clone()) {
            Ok(v) => entries = v,
            Err(_) => return skills
        }

        for entry_dir in entries {
            let entry;
            match entry_dir {
                Ok(v) => entry = v,
                Err(_) => continue
            }

            let path = entry.path();

            match Self::load_skill(path) {
                Ok((dir, mut v)) => {
                    if v.start().is_ok() {
                        skills.insert(dir.to_string(), v);
                    }
                },
                Err(e) => println!("Error loading skill: {}", e)
            }
        }

        skills
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

        let dir_name;
        match path.file_name() {
            Some(v) => dir_name = v,
            None => return Err("Could not get directory name".into())
        }

        let dir_name_str;
        match dir_name.to_str() {
            Some(v) => dir_name_str = v,
            None => return Err("Could not get directory name as string".into())
        }

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
    /// 
    /// FIXME: The check `parts.is_empty() && parts.len() != 2` is logically flawed; it will never be true.
    pub fn run_intent(&mut self, intent: Intent) -> Result<bool, Box<dyn std::error::Error>> {
        let intent_info;
        match intent.intent.clone() {
            Some(v) => intent_info = v,
            None => return Err("Intent is not defined".into())
        }

        let full_name;
        match intent_info.intent_name {
            Some(v) => full_name = v,
            None => return Err("Intent name is not defined".into())
        }

        let parts: Vec<&str> = full_name.split("@").collect();
        if parts.is_empty() && parts.len() != 2 {
            return Err("Intent name must contain '@' separator".into());
        }

        let skill_name = parts[0];

        match self.skills.get_mut(skill_name) {
            Some(v) => {
                Ok(v.run_intent(intent)?)
            }
            None => {
                Err(format!("Skill {} not found", skill_name).into())
            }
        }
    }
}