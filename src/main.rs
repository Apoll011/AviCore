#[macro_use]
extern crate dyon;

mod skills;
mod intent;

use dyon::{load, Call};
use std::sync::Arc;
use dyon::{error, Runtime};
use crate::skills::dsl::avi_dsl::{load_module};
mod api;

use std::io::{stdin, stdout, Write};
use crate::api::api::Api;
use crate::intent::Intent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = Api::new();
    let mut dyon_runtime = Runtime::new();

    println!("Server replied: {:?}", api.alive().await?);

    loop {
        let mut s=String::new();
        print!("Please enter some text: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        let intent = api.intent(s.as_str()).await?;
        println!("Intent: {:?}", intent);

        run_skill(&mut dyon_runtime, "LightControl", intent);
    }

    Ok(())
}


fn run_skill(dyon_runtime: &mut Runtime, skill_name: &str, intent: Intent) {
    let mut dyon_module = load_module().unwrap();

    if error(load(&format!("./skills/{}/main.avi", skill_name), &mut dyon_module)) {
        print!("Error loading module")
    } else {
        println!("Module loaded")
    }

    let arc_module = Arc::new(dyon_module);

    let call = Call::new(&"intent_turn_on".to_string()).arg(intent);
    error(call.run(dyon_runtime, &arc_module));


    if error(dyon_runtime.run(&arc_module)) {
        return;
    }
}