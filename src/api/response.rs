use serde_json::Value;

#[derive(Debug)]
pub struct Response {
    pub(crate) response: Option<Value>
}

impl Response {
    pub fn new(response: Value) -> Self {
        Self {
            response: response.get("response").cloned(),
        }
    }
}
