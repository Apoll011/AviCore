use std::collections::HashMap;

use crate::config::setting;
use crate::dialogue::languages::lang;
use avi_nlu_client::apis::configuration::Configuration;
use avi_nlu_client::apis::*;
use avi_nlu_client::models::{
    Alive, Created, Data, EngineTrain, EngineTrainType, Installed, RecognizedInput,
};
use log::trace;

fn box_err<E: std::fmt::Display>(e: E) -> Box<dyn std::error::Error> {
    Box::new(std::io::Error::other(e.to_string()))
}
/// A client for interacting with the Avi server API.
pub struct Api {
    config: Configuration,
}

impl Api {
    /// Creates a new instance of the `Api` client.
    pub fn new() -> Self {
        let url = setting::<String>("api_url").unwrap_or("http://0.0.0.0:1178".to_string());
        Self {
            config: Configuration {
                base_path: url,
                ..Default::default()
            },
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
        default_api::check_if_alive_avi_alive_get(&self.config)
            .await
            .map_err(box_err)
    }

    pub async fn avaliable_engines(&self) -> Result<Installed, Box<dyn std::error::Error>> {
        intent_api::returns_the_instaled_engines_intent_recognition_installed_get(&self.config)
            .await
            .map_err(box_err)
    }

    pub async fn get_active_intents(
        &self,
    ) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
        let mut data: HashMap<String, Vec<String>> = Default::default();

        let a = self.avaliable_engines().await?.data;

        let slot_name_mappings: HashMap<String, HashMap<String, String>> = a
            .get(&lang())
            .and_then(|v| v.get("slot_name_mappings"))
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        for key in a.keys() {
            data.insert(
                key.clone(),
                slot_name_mappings.keys().cloned().collect::<Vec<_>>(),
            );
        }

        Ok(data)
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
    pub async fn intent(&self, text: &str) -> Result<RecognizedInput, Box<dyn std::error::Error>> {
        trace!("Requesting intent recognition for: {}", text);
        intent_api::recognize_intent_from_sentence_intent_recognition_get(&self.config, text)
            .await
            .map_err(box_err)
    }

    #[allow(dead_code)]
    pub async fn train_intent_engine(
        &self,
        train_type: EngineTrainType,
    ) -> Result<EngineTrain, Box<dyn std::error::Error>> {
        intent_api::train_or_reuse_the_intent_recognition_engine_intent_recognition_engine_post(
            &self.config,
            Some(train_type),
        )
        .await
        .map_err(box_err)
    }

    pub async fn set_engine_dataset(
        &self,
        dataset: Data,
    ) -> Result<Created, Box<dyn std::error::Error>> {
        intent_api::define_the_intent_and_entities_intent_recognition_populate_post(
            &self.config,
            dataset,
        )
        .await
        .map_err(box_err)
    }

    /*
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
        Ok(serde_json::from_value(r.ok_or("No response from server")?)?)
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
        Ok(serde_json::from_value(r.ok_or("No response from server")?)?)
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
        Ok(serde_json::from_value(r.ok_or("No response from server")?)?)
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
        Ok(serde_json::from_value(r.ok_or("No response from server")?)?)
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
        let r = send_dict_to_server(&self.client, &self.get_url("/lang/parse/normalize")?, query)
            .await?;
        Ok(serde_json::from_value(r.ok_or("No response from server")?)?)
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
        Ok(serde_json::from_value(r.ok_or("No response from server")?)?)
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
        Ok(serde_json::from_value(r.ok_or("No response from server")?)?)
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
        let r = send_dict_to_server(
            &self.client,
            &self.get_url("/lang/format/nice_time")?,
            query,
        )
        .await?;
        Ok(serde_json::from_value(r.ok_or("No response from server")?)?)
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
        Ok(serde_json::from_value(r.ok_or("No response from server")?)?)
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
        Ok(serde_json::from_value(r.ok_or("No response from server")?)?)
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
        Ok(serde_json::from_value(r.ok_or("No response from server")?)?)
    }*/
}
