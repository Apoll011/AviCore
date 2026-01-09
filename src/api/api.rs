use crate::api::send::send_dict_to_server;
use crate::config::setting;
use crate::dialogue::intent::Intent;
use crate::dialogue::lang_parse::{
    ExtractDatetime, ExtractDuration, ExtractNumber, ExtractNumbers, IsFractional,
};
use log::{debug, error, trace};
use reqwest::Client;
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
pub struct Api {
    client: Client,
}

impl Api {
    /// Creates a new instance of the `Api` client.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Constructs a full URL for a given API path using the runtime's base API URL.
    ///
    /// # Arguments
    ///
    /// * `path` - The specific API endpoint path (e.g., "/avi/alive").
    fn get_url(&self, path: &str) -> Result<String, String> {
        match setting::<String>("api_url") {
            Some(path_api) => Ok(format!("{}{}", path_api, path)),
            None => Err("api_url Not defined".to_string()),
        }
    }

    /// Checks if the server is alive and retrieves basic server information.
    ///
    /// # Errors
    ///
    /// Returns an error if the server is unreachable or the response is invalid.
    #[allow(dead_code)]
    pub async fn alive(&self) -> Result<Alive, Box<dyn std::error::Error>> {
        trace!("Checking server alive status");
        let r = send_dict_to_server(&self.client, &self.get_url("/avi/alive")?, HashMap::new()).await?;
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
            }
            None => {
                error!("No response from server for /avi/alive");
                Err("No response from server".into())
            }
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
    pub async fn intent(&self, text: &str) -> Result<Intent, Box<dyn std::error::Error>> {
        trace!("Requesting intent recognition for: {}", text);
        let mut query = HashMap::new();
        query.insert("text".to_string(), text.to_string());
        let r =
            send_dict_to_server(&self.client, &self.get_url("/intent_recognition")?, query).await?;
        let response = r.response;

        match response {
            Some(v) => {
                debug!("Intent recognition response: {:?}", v);
                Ok(serde_json::from_value(v)?)
            }
            None => {
                error!("No response from server for /intent_recognition");
                Err("No response from server".into())
            }
        }
    }

    #[allow(dead_code)]
    pub async fn train_intent_engine(
        &self,
        train_type: &str,
        lang: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("type".to_string(), train_type.to_string());
        query.insert("lang".to_string(), lang.to_string());
        let r = send_dict_to_server(
            &self.client,
            &self.get_url("/intent_recognition/engine")?,
            query,
        )
        .await?;
        Ok(r.response.unwrap_or(serde_json::Value::Null))
    }

    #[allow(dead_code)]
    pub async fn extract_numbers(
        &self,
        text: &str,
        short_scale: bool,
        ordinals: bool,
        lang: &str,
    ) -> Result<ExtractNumbers, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("text".to_string(), text.to_string());
        query.insert("short_scale".to_string(), short_scale.to_string());
        query.insert("ordinals".to_string(), ordinals.to_string());
        query.insert("lang".to_string(), lang.to_string());
        let r = send_dict_to_server(
            &self.client,
            &self.get_url("/lang/parse/extract_numbers")?,
            query,
        )
        .await?;
        Ok(serde_json::from_value(
            r.response.ok_or("No response from server")?,
        )?)
    }

    #[allow(dead_code)]
    pub async fn extract_number(
        &self,
        text: &str,
        short_scale: bool,
        ordinals: bool,
        lang: &str,
    ) -> Result<ExtractNumber, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("text".to_string(), text.to_string());
        query.insert("short_scale".to_string(), short_scale.to_string());
        query.insert("ordinals".to_string(), ordinals.to_string());
        query.insert("lang".to_string(), lang.to_string());
        let r = send_dict_to_server(
            &self.client,
            &self.get_url("/lang/parse/extract_number")?,
            query,
        )
        .await?;
        Ok(serde_json::from_value(
            r.response.ok_or("No response from server")?,
        )?)
    }

    #[allow(dead_code)]
    pub async fn extract_duration(
        &self,
        text: &str,
        lang: &str,
    ) -> Result<ExtractDuration, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("text".to_string(), text.to_string());
        query.insert("lang".to_string(), lang.to_string());
        let r = send_dict_to_server(
            &self.client,
            &self.get_url("/lang/parse/extract_duration")?,
            query,
        )
        .await?;
        Ok(serde_json::from_value(
            r.response.ok_or("No response from server")?,
        )?)
    }

    #[allow(dead_code)]
    pub async fn extract_datetime(
        &self,
        text: &str,
        lang: &str,
    ) -> Result<ExtractDatetime, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("text".to_string(), text.to_string());
        query.insert("lang".to_string(), lang.to_string());
        let r = send_dict_to_server(
            &self.client,
            &self.get_url("/lang/parse/extract_datetime")?,
            query,
        )
        .await?;
        Ok(serde_json::from_value(
            r.response.ok_or("No response from server")?,
        )?)
    }

    #[allow(dead_code)]
    pub async fn normalize(
        &self,
        text: &str,
        lang: &str,
        remove_articles: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("text".to_string(), text.to_string());
        query.insert("lang".to_string(), lang.to_string());
        query.insert("remove_articles".to_string(), remove_articles.to_string());
        let r = send_dict_to_server(
            &self.client,
            &self.get_url("/lang/parse/normalize")?,
            query,
        )
        .await?;
        Ok(serde_json::from_value(
            r.response.ok_or("No response from server")?,
        )?)
    }

    #[allow(dead_code)]
    pub async fn is_fractional(
        &self,
        input_str: &str,
        short_scale: bool,
        lang: &str,
    ) -> Result<IsFractional, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("input_str".to_string(), input_str.to_string());
        query.insert("short_scale".to_string(), short_scale.to_string());
        query.insert("lang".to_string(), lang.to_string());
        let r = send_dict_to_server(
            &self.client,
            &self.get_url("/lang/parse/is_fractional")?,
            query,
        )
        .await?;
        Ok(serde_json::from_value(
            r.response.ok_or("No response from server")?,
        )?)
    }

    #[allow(dead_code)]
    pub async fn nice_number(
        &self,
        number: f64,
        lang: &str,
        speech: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("number".to_string(), number.to_string());
        query.insert("lang".to_string(), lang.to_string());
        query.insert("speech".to_string(), speech.to_string());
        let r = send_dict_to_server(
            &self.client,
            &self.get_url("/lang/format/nice_number")?,
            query,
        )
        .await?;
        Ok(serde_json::from_value(
            r.response.ok_or("No response from server")?,
        )?)
    }

    #[allow(dead_code)]
    pub async fn nice_time(
        &self,
        dt: &str,
        lang: &str,
        speech: bool,
        use_24hour: bool,
        use_ampm: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("dt".to_string(), dt.to_string());
        query.insert("lang".to_string(), lang.to_string());
        query.insert("speech".to_string(), speech.to_string());
        query.insert("use_24hour".to_string(), use_24hour.to_string());
        query.insert("use_ampm".to_string(), use_ampm.to_string());
        let r =
            send_dict_to_server(&self.client, &self.get_url("/lang/format/nice_time")?, query).await?;
        Ok(serde_json::from_value(
            r.response.ok_or("No response from server")?,
        )?)
    }

    #[allow(dead_code)]
    pub async fn pronounce_number(
        &self,
        number: i64,
        lang: &str,
        places: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("number".to_string(), number.to_string());
        query.insert("lang".to_string(), lang.to_string());
        query.insert("places".to_string(), places.to_string());
        let r = send_dict_to_server(
            &self.client,
            &self.get_url("/lang/format/pronounce_number")?,
            query,
        )
        .await?;
        Ok(serde_json::from_value(
            r.response.ok_or("No response from server")?,
        )?)
    }

    #[allow(dead_code)]
    pub async fn nice_duration(
        &self,
        duration: i64,
        lang: &str,
        speech: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("duration".to_string(), duration.to_string());
        query.insert("lang".to_string(), lang.to_string());
        query.insert("speech".to_string(), speech.to_string());
        let r = send_dict_to_server(
            &self.client,
            &self.get_url("/lang/format/nice_duration")?,
            query,
        )
        .await?;
        Ok(serde_json::from_value(
            r.response.ok_or("No response from server")?,
        )?)
    }

    #[allow(dead_code)]
    pub async fn nice_relative_time(
        &self,
        when: &str,
        relative_to: &str,
        lang: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("when".to_string(), when.to_string());
        query.insert("relative_to".to_string(), relative_to.to_string());
        query.insert("lang".to_string(), lang.to_string());
        let r = send_dict_to_server(
            &self.client,
            &self.get_url("/lang/format/nice_relative_time")?,
            query,
        )
        .await?;
        Ok(serde_json::from_value(
            r.response.ok_or("No response from server")?,
        )?)
    }
}
