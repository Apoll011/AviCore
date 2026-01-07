use crate::api::send::send_dict_to_server;
use crate::ctx::runtime;
use crate::dialogue::intent::Intent;
use log::{debug, error, trace};
use std::collections::HashMap;

/// Represents the status and basic information of the server.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Alive {
    /// Indicates if the server is currently online.
    pub alive: bool,
    /// The version of the server software.
    pub version: String,
    /// A list of languages currently installed on the server.
    pub installed_lang: Vec<String>,
}

/// A client for interacting with the Avi server API.
pub struct Api {}

impl Api {
    /// Creates a new instance of the `Api` client.
    pub fn new() -> Self {
        Self {}
    }

    /// Constructs a full URL for a given API path using the runtime's base API URL.
    ///
    /// # Arguments
    ///
    /// * `path` - The specific API endpoint path (e.g., "/avi/alive").
    fn get_url(&self, path: &str) -> Result<String, String> {
        match runtime() {
            Ok(c) => Ok(format!("{}{}", c.api_url, path).to_string()),
            Err(e) => Err(e),
        }
    }

    /// Checks if the server is alive and retrieves basic server information.
    ///
    /// # Errors
    ///
    /// Returns an error if the server is unreachable or the response is invalid.
    #[allow(dead_code)]
    pub async fn alive(&mut self) -> Result<Alive, Box<dyn std::error::Error>> {
        trace!("Checking server alive status");
        let r = send_dict_to_server(&self.get_url("/avi/alive")?, HashMap::new()).await?;
        let response = r.response;
        match response {
            Some(v) => {
                debug!("Server alive response: {:?}", v);
                Ok(Alive {
                    alive: v
                        .get("on")
                        .expect("Expected a boolean")
                        .as_bool()
                        .unwrap_or(false),
                    version: v
                        .get("version")
                        .expect("Expected a version string")
                        .as_str()
                        .unwrap_or("0.0")
                        .to_string(),
                    installed_lang: v
                        .get("lang")
                        .expect("Expected a list of installed lang's")
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|x| x.as_str().unwrap().to_string())
                        .collect(),
                })
            },
            None => {
                error!("No response from server for /avi/alive");
                Err("No response from server".into())
            },
        }
    }

    /// Sends a text message to the server for intent recognition.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to be processed for intent recognition.
    ///
    /// # Errors
    ///
    /// Returns an error if the server is unreachable or the intent cannot be parsed.
    ///
    pub async fn intent(&mut self, text: &str) -> Result<Intent, Box<dyn std::error::Error>> {
        trace!("Requesting intent recognition for: {}", text);
        let mut query = HashMap::new();
        query.insert("text", text);
        let r = send_dict_to_server(&self.get_url("/intent_recognition")?, query).await?;
        let response = r.response;

        match response {
            Some(v) => {
                debug!("Intent recognition response: {:?}", v);
                Ok(serde_json::from_value(v)?)
            },
            None => {
                error!("No response from server for /intent_recognition");
                Err("No response from server".into())
            },
        }
    }
}
