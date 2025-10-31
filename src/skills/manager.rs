use std::fs;
use std::collections::HashMap;
use crate::handle;
use crate::intent::Intent;
use crate::skills::fallback::{handle, FallbackType};
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

        if let Ok(entries) = fs::read_dir("./skills") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(dir_name) = path.file_name() {
                            if let Some(dir_name_str) = dir_name.to_str() {
                                let mut skill = Skill::new(dir_name_str.to_string());
                                skill.start();
                                skills.insert(dir_name_str.to_string(), skill);
                            }
                        }
                    }
                }
            }
        }

        skills
    }

    pub fn run_intent(&mut self, intent: Intent) {
        let intent_info = intent.intent.as_ref();
        if intent_info.is_none() {
            handle!(&FallbackType::NotUnderstood, &intent.input);
            return;
        }
        let intent_info = intent_info.unwrap();

        let full_name = intent_info.intent_name.as_ref();
        if full_name.is_none() {
            handle!(&FallbackType::NotUnderstood, &intent.input);
            return;
        }
        let full_name = full_name.unwrap();

        let parts: Vec<&str> = full_name.split("@").collect();
        assert!(!parts.is_empty(), "Intent name must contain '@' separator");
        let skill_name = parts[0];

        let skill = self.skills.get_mut(skill_name);
        if skill.is_none() {
            handle!(&FallbackType::NotInstalled);
            return;
        }
        let skill = skill.unwrap();

        skill.run_intent(intent);
    }
}