use crate::api::response::Response;
use log::{debug, error, trace};
use reqwest::{Client, Method, RequestBuilder};
use serde_json::Value;
use std::collections::HashMap;

/// Sends a GET request to the specified server URL with the provided arguments.
///
/// # Arguments
///
/// * `client` - The reqwest Client.
/// * `url` - The full target URL for the API request.
/// * `args` - A map of query parameters to be included in the request.
///
/// # Errors
///
/// Returns an error if the network request fails or if the response body cannot be parsed as JSON.
pub async fn send_dict_to_server(
    client: &Client,
    url: &str,
    args: HashMap<String, String>,
) -> Result<Response, Box<dyn std::error::Error>> {
    send(client, url, args, Method::GET).await
}

fn get_builder(client: &Client, url: &str, t: Method) -> RequestBuilder {
    match t {
        Method::GET => client.get(url),
        Method::POST => client.post(url),
        Method::PUT => client.put(url),
        Method::PATCH => client.patch(url),
        Method::DELETE => client.delete(url),
        Method::HEAD => client.head(url),
        _ => {
            error!("Method {} Not supported!", t);
            client.post(url)
        },
    }
}
pub async fn send(client: &Client, url: &str, args: HashMap<String, String>, method: Method) -> Result<Response, Box<dyn std::error::Error>>  {
    trace!("Sending request to server: {} with args: {:?}", url, args);

    let resp = match get_builder(client, url, method).query(&args).send().await {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to send request to {}: {}", url, e);
            return Err(e.into());
        }
    };

    let status = resp.status();
    if !status.is_success() {
        error!("Server returned error status for {}: {}", url, status);
    }

    let json: Value = match resp.json().await {
        Ok(j) => j,
        Err(e) => {
            error!("Failed to parse JSON response from {}: {}", url, e);
            return Err(e.into());
        }
    };

    debug!("Received response from {}: {:?}", url, json);
    let response = Response::new(json);

    Ok(response)
}