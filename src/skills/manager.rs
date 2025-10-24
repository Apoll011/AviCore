use std::fs;
use std::collections::HashMap;
use crate::intent::Intent;
use crate::skills::skill::Skill;

pub struct SkillManager {
    skills: HashMap<String, Skill>,
}

impl SkillManager {
    pub fn new() -> Self {
        Self {
            skills: Default::default(),
        }
    }

    pub fn load_skills(&mut self) -> HashMap<String, Skill> {
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
        let intent_info = intent.intent.as_ref()
            .expect("Intent must have intent info");
        let full_name = intent_info.intent_name.as_ref()
            .expect("Intent must have a name");

        let parts: Vec<&str> = full_name.split("@").collect();
        assert!(!parts.is_empty(), "Intent name must contain '@' separator");
        let skill_name = parts[0];

        let skill = self.skills.get_mut(skill_name)
            .unwrap_or_else(|| panic!("Skill '{}' not found", skill_name));

        skill.run_intent(intent);
    }
}