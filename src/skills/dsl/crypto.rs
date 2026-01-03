use dyon::Type::*;
use dyon::embed::PushVariable;
use dyon::{Dfn, Module, Runtime, Variable};
use hmac::{Hmac, KeyInit, Mac};
use sha2::{Digest, Sha256};
use std::result::Result;
use std::sync::Arc;

type HmacSha256 = Hmac<Sha256>;

pub fn add_functions(module: &mut Module) {
    module.ns("crypto");
    module.add(
        Arc::new("hash".into()),
        crypto_hash,
        Dfn::nl(vec![Str, Str], Str),
    );
    module.add(
        Arc::new("hmac".into()),
        crypto_hmac,
        Dfn::nl(vec![Str, Str, Str], Str),
    );
}

#[allow(non_snake_case)]
pub fn crypto_hash(rt: &mut Runtime) -> Result<Variable, String> {
    let algo: String = rt.pop()?; // always "sha256"
    let data: String = rt.pop()?;

    if algo != "sha256" {
        return Err("Unsupported hash algorithm".to_string());
    }

    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let out = format!("{:?}", hasher.finalize());

    Ok(PushVariable::push_var(&out))
}

#[allow(non_snake_case)]
pub fn crypto_hmac(rt: &mut Runtime) -> Result<Variable, String> {
    let algo: String = rt.pop()?; // "sha256"
    let message: String = rt.pop()?;
    let key: String = rt.pop()?;

    if algo != "sha256" {
        return Err("Unsupported HMAC algorithm".to_string());
    }

    let mut mac =
        HmacSha256::new_from_slice(key.as_bytes()).map_err(|_| "Invalid HMAC key".to_string())?;

    mac.update(message.as_bytes());
    let result = mac.finalize();
    let bytes = result.into_bytes();

    Ok(PushVariable::push_var(&hex::encode(bytes)))
}
