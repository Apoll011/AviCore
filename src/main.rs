#[macro_use]
extern crate dyon;

mod skills;
mod intent;
mod api;
mod ctx;

use std::io::{stdin, stdout, Write};
use crate::api::api::Api;
use crate::ctx::RuntimeContext;
use crate::ctx::RUNTIMECTX;
use crate::skills::manager::SkillManager;

fn input(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = RUNTIMECTX.set(RuntimeContext {
        api_url: "http://127.0.0.1:1178".into(),
        lang: "pt".into(),
        skill_path: "./skills".into(),
    });

    let mut api = Api::new();
    let mut manager = SkillManager::new();

    loop {
        let s = input("Please enter some text: ");
        let intent = api.intent(s.as_str()).await?;

        match manager.run_intent(intent) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e)
        }
    }

    Ok(())
}
