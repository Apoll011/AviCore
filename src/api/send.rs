use reqwest::Client;
use serde_json::{Value};
use std::collections::HashMap;
use crate::api::response::Response;

/// Sends a GET request to the specified server URL with the provided arguments.
/// 
/// # Arguments
/// 
/// * `url` - The full target URL for the API request.
/// * `args` - A map of query parameters to be included in the request.
/// 
/// # Errors
/// 
/// Returns an error if the network request fails or if the response body cannot be parsed as JSON.
/// 
/// TODO: Consider using a POST request if the arguments grow too large or contain sensitive data.
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
