use crate::ctx::runtime;
use rhai::plugin::*;

#[export_module]
pub mod user_module {
    use crate::dialogue::languages::lang;
    use crate::user::{Location, QuietHours, user_name};

    /// Gets the user's name
    ///
    /// # Returns
    /// The user's name, or an empty string if not available
    pub fn name() -> String {
        user_name()
    }

    /// Gets the user's nickname
    ///
    /// # Returns
    /// The user's nickname, or () if not set
    pub fn nickname() -> Option<String> {
        match runtime() {
            Ok(c) => c.user.get_nickname(),
            Err(_) => None,
        }
    }

    /// Gets the user's ID
    ///
    /// # Returns
    /// The user's ID, or an empty string if not available
    pub fn id() -> Option<String> {
        match runtime() {
            Ok(c) => Some(c.user.get_id()),
            Err(_) => None,
        }
    }

    /// Gets the user's location
    ///
    /// # Returns
    /// A map with 'city' and 'country' fields, or () if not set
    pub fn location() -> Option<Location> {
        match runtime() {
            Ok(c) => c.user.get_location(),
            Err(_) => None,
        }
    }

    /// Gets the user's quiet hours
    ///
    /// # Returns
    /// A map with 'start' and 'end' fields, or () if not set
    pub fn quiet_hours() -> Option<QuietHours> {
        match runtime() {
            Ok(c) => c.user.get_quiet_hours(),
            Err(_) => None,
        }
    }

    /// Gets the user's birthday as a Unix timestamp
    ///
    /// # Returns
    /// The birthday timestamp
    pub fn birthday() -> Option<i64> {
        match runtime() {
            Ok(c) => c.user.get_birthday(),
            Err(_) => None,
        }
    }

    /// Gets the user's voice profile ID
    ///
    /// # Returns
    /// The voice profile ID, or () if not set
    pub fn voice_profile_id() -> Option<String> {
        match runtime() {
            Ok(c) => c.user.get_voice_profile_id(),
            Err(_) => None,
        }
    }

    /// Gets the user's language preference
    ///
    /// # Returns
    /// The language code (e.g., "en"), defaults to "en" if not set
    pub fn language() -> String {
        match runtime() {
            Ok(c) => c.user.get_language(),
            Err(_) => lang(),
        }
    }
}
