use reqwest::Client;
use serde_json::{Value};
use std::collections::HashMap;
use crate::api::response::Response;

pub async fn send_dict_to_server(
    url: &str,
    args: HashMap<&str, &str>,
) -> Result<Response, Box<dyn std::error::Error>> {
    let client = Client::new();

    let resp = client
        .get(url)
        .query(&args)
        .send()
        .await?;

    let json: Value = resp.json().await?;
    let response = Response::new(json);

    Ok(response)
}
