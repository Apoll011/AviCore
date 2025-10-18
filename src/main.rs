#[macro_use]
extern crate dyon;

mod skills;
mod intent;

use dyon::{load, Call};
use std::sync::Arc;
use crate::skills::dsl::avi_dsl::{load_module, Person};
/*
mod api;

use std::io::{stdin, stdout, Write};
use crate::api::api::Api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = Api::new();

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


    }

    Ok(())
}
*/


fn main() {
    use dyon::{error, Runtime};

    let mut dyon_runtime = Runtime::new();
    let mut dyon_module = load_module().unwrap();

    if error(load("./skills/LightControl/main.avi", &mut dyon_module)) {
        print!("Error loading module")
    } else {
        println!("Module loaded")
    }

    let arc_module = Arc::new(dyon_module);

    let call = Call::new("intent_light_on").arg(Person { first_name: "Tiago".to_string(), last_name: "Ines".to_string(), age: 18 });
    error(call.run(&mut Runtime::new(), &arc_module));


    if error(dyon_runtime.run(&arc_module)) {
        return;
    }
}