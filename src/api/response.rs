use serde_json::Value;

/// Represents a structured response from the Avi server.
#[derive(Debug)]
pub struct Response {
    /// The extracted "response" field from the server's JSON payload.
    pub response: Option<Value>,
}

impl Response {
    /// Creates a new `Response` instance by extracting the "response" field from the provided JSON value.
    ///
    /// # Arguments
    ///
    /// * `response` - The raw JSON `Value` received from the server.
    pub fn new(response: Value) -> Self {
        Self {
            response: response.get("response").cloned(),
        }
    }
}
