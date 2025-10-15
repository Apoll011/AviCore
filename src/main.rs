mod api;
use std::collections::HashMap;
use crate::api::send::send_dict_to_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://127.0.0.1:1178/lang/format/pronounce_number";

    let mut args = HashMap::new();
    args.insert("number", "42");

    let r = send_dict_to_server(url, args).await?;
    println!("Server replied: {:?}", r.response);

    Ok(())
}
