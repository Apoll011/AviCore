use crate::ctx::runtime;
use rhai::plugin::*;
use rhai::{Dynamic, EvalAltResult, Map, Position};

#[export_module]
pub mod user_module {
    /// Gets the user's name
    ///
    /// # Returns
    /// The user's name, or an empty string if not available
    #[rhai_fn(return_raw)]
    pub fn name() -> Result<String, Box<EvalAltResult>> {
        match runtime() {
            Ok(c) => Ok(c.user.get_name()),
            Err(e) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Failed to get runtime: {}", e).into(),
                Position::NONE,
            ))),
        }
    }

    /// Gets the user's nickname
    ///
    /// # Returns
    /// The user's nickname, or () if not set
    #[rhai_fn(return_raw)]
    pub fn nickname() -> Result<Dynamic, Box<EvalAltResult>> {
        match runtime() {
            Ok(c) => match c.user.get_nickname() {
                Some(nick) => Ok(Dynamic::from(nick)),
                None => Ok(Dynamic::UNIT),
            },
            Err(e) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Failed to get runtime: {}", e).into(),
                Position::NONE,
            ))),
        }
    }

    /// Gets the user's ID
    ///
    /// # Returns
    /// The user's ID, or an empty string if not available
    #[rhai_fn(return_raw)]
    pub fn id() -> Result<String, Box<EvalAltResult>> {
        match runtime() {
            Ok(c) => Ok(c.user.get_id()),
            Err(e) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Failed to get runtime: {}", e).into(),
                Position::NONE,
            ))),
        }
    }

    /// Gets the user's location
    ///
    /// # Returns
    /// A map with 'city' and 'country' fields, or () if not set
    #[rhai_fn(return_raw)]
    pub fn location() -> Result<Dynamic, Box<EvalAltResult>> {
        match runtime() {
            Ok(c) => match c.user.get_location() {
                Some(loc) => {
                    let mut map = Map::new();
                    map.insert("city".into(), Dynamic::from(loc.city));
                    map.insert("country".into(), Dynamic::from(loc.country));
                    Ok(Dynamic::from(map))
                }
                None => Ok(Dynamic::UNIT),
            },
            Err(e) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Failed to get runtime: {}", e).into(),
                Position::NONE,
            ))),
        }
    }

    /// Gets the user's quiet hours
    ///
    /// # Returns
    /// A map with 'start' and 'end' fields, or () if not set
    #[rhai_fn(return_raw)]
    pub fn quiet_hours() -> Result<Dynamic, Box<EvalAltResult>> {
        match runtime() {
            Ok(c) => match c.user.get_quiet_hours() {
                Some(qh) => {
                    let mut map = Map::new();
                    map.insert("start".into(), Dynamic::from(qh.start));
                    map.insert("end".into(), Dynamic::from(qh.end));
                    Ok(Dynamic::from(map))
                }
                None => Ok(Dynamic::UNIT),
            },
            Err(e) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Failed to get runtime: {}", e).into(),
                Position::NONE,
            ))),
        }
    }

    /// Gets the user's birthday as a Unix timestamp
    ///
    /// # Returns
    /// The birthday timestamp as f64, or 0.0 if not set
    #[rhai_fn(return_raw)]
    pub fn birthday() -> Result<f64, Box<EvalAltResult>> {
        match runtime() {
            Ok(c) => match c.user.get_birthday() {
                Some(timestamp) => Ok(timestamp as f64),
                None => Ok(0.0),
            },
            Err(e) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Failed to get runtime: {}", e).into(),
                Position::NONE,
            ))),
        }
    }

    /// Gets the user's voice profile ID
    ///
    /// # Returns
    /// The voice profile ID, or () if not set
    #[rhai_fn(return_raw)]
    pub fn voice_profile_id() -> Result<Dynamic, Box<EvalAltResult>> {
        match runtime() {
            Ok(c) => match c.user.get_voice_profile_id() {
                Some(id) => Ok(Dynamic::from(id)),
                None => Ok(Dynamic::UNIT),
            },
            Err(e) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Failed to get runtime: {}", e).into(),
                Position::NONE,
            ))),
        }
    }

    /// Gets the user's language preference
    ///
    /// # Returns
    /// The language code (e.g., "en"), defaults to "en" if not set
    #[rhai_fn(return_raw)]
    pub fn language() -> Result<String, Box<EvalAltResult>> {
        match runtime() {
            Ok(c) => Ok(c.user.get_language()),
            Err(e) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("Failed to get runtime: {}", e).into(),
                Position::NONE,
            ))),
        }
    }
}
