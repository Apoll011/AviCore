use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::ctx::RUNTIMECTX;
use crate::intent::Intent;
use crate::skills::skill::Skill;

pub struct SkillManager {
    skills: HashMap<String, Skill>,
}

impl SkillManager {
    pub fn new() -> Self {
        Self {
            skills: Self::load_skills(),
        }
    }

    pub fn load_skills() -> HashMap<String, Skill> {
        let mut skills = HashMap::new();

        let entries;
        match fs::read_dir(RUNTIMECTX.get().unwrap().skill_path.clone()) {
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