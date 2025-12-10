use std::sync::OnceLock;

#[derive(Debug)]
pub struct RuntimeContext {
    pub(crate) lang: String,
}

pub(crate) static RUNTIMECTX: OnceLock<RuntimeContext> = OnceLock::new();