#[macro_use]
extern crate dyon;

mod skills;
mod intent;
mod api;

use std::io::{stdin, stdout, Write};
use crate::api::api::Api;
use crate::skills::skill::Skill;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = Api::new();
    let mut skill = Skill::new("saudation".to_string());
    skill.start();

    loop {
        let mut s = String::new();
        print!("Please enter some text: ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        let intent = api.intent(s.as_str()).await?;
        println!("Intent: {:?}", intent);

        skill.run_intent(intent);
    }

    Ok(())
}


/*
use std::fs;
use std::collections::HashMap;
use crate::intent::Intent;
struct SkillManager {
    skills: HashMap<String, Skill>,
}

impl SkillManager {
    fn load_skills() -> HashMap<String, Skill> {
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

    fn run_intent(&mut self, intent: Intent) {
        let name = intent.intent.clone().unwrap().intent_name.unwrap();
        let skill_name = name.split("@").collect::<Vec<&str>>()[0];
        let mut skill = self.skills.get(skill_name).unwrap();

        skill.run_intent(intent);

    }
}

*/