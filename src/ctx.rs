use std::sync::OnceLock;

#[derive(Debug)]
pub struct RuntimeContext {
    pub(crate) api_url: String,
    pub(crate) lang: String,
    pub(crate) skill_path: String,
}

pub(crate) static RUNTIMECTX: OnceLock<RuntimeContext> = OnceLock::new();