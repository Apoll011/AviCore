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