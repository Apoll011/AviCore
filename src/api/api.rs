use std::collections::HashMap;
use crate::api::send::send_dict_to_server;
use crate::ctx::RUNTIMECTX;
use crate::intent::Intent;

#[derive(Debug)]
pub struct Alive {
    pub(crate) alive: bool,
    pub(crate) version: String,
    pub(crate) installed_lang: Vec<String>,
}
pub struct Api {}

impl Api {
    pub fn new() -> Self {
        Self {}
    }
    fn get_url(&self, path: &str) -> String {
        match RUNTIMECTX.get() {
            Some(v) => {
                format!("{}{}", v.api_url, path).into()
            }
            None => {
                "http://127.0.0.1:1178".into()
            }
        }
    }

    pub async fn alive(&mut self) -> Result<Alive, Box<dyn std::error::Error>> {
        let r = send_dict_to_server(&*self.get_url("/avi/alive"), HashMap::new()).await?;
        let response = r.response;
        match response {
            Some(v) => Ok(Alive {
                    alive: v.get("on").expect("Expected a boolean").as_bool().unwrap_or(false),
                    version: v.get("version").expect("Expected a version string").to_string(), //FIXME: This is getting in as a string as I convert it to string the quotes from the previous string are being included
                    installed_lang: v.get("lang").expect("Expected a list of installed lang's").as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect(),
                }),
            None => Err("No response from server".into())
        }

    }

    pub async fn intent(&mut self, text: &str) -> Result<Intent, Box<dyn std::error::Error>> {
        let mut query = HashMap::new();
        query.insert("text", text);
        let r = send_dict_to_server(&*self.get_url("/intent_recognition"), query).await.expect(":)");
        let response = r.response;

        match response {
            Some(v) => {
                Ok(serde_json::from_value(v)?)
            },
            None => {
                Err("No response from server".into())
            }
        }

    }
}