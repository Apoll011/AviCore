mod api;
mod intent;

use std::io::{stdin,stdout,Write};
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
