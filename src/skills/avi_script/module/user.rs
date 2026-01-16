use crate::ctx::runtime;
use rhai::plugin::*;

#[export_module]
pub mod user_module {
    use crate::dialogue::languages::lang;
    use crate::user::{Location, QuietHours, user_name};
    use chrono::{DateTime, Utc};

    /// Gets the user's name
    ///
    /// # Returns
    /// The user's name, or an empty ImmutableString if not available
    pub fn name() -> ImmutableString {
        ImmutableString::from(user_name())
    }

    /// Gets the user's nickname
    ///
    /// # Returns
    /// The user's nickname, or () if not set
    pub fn nickname() -> Option<ImmutableString> {
        Some(ImmutableString::from(runtime().ok()?.user.get_nickname()?))
    }

    /// Gets the user's ID
    ///
    /// # Returns
    /// The user's ID, or an empty ImmutableString if not available
    pub fn id() -> Option<ImmutableString> {
        Some(ImmutableString::from(runtime().ok()?.user.get_id()))
    }

    /// Gets the user's location
    ///
    /// # Returns
    /// A map with 'city' and 'country' fields, or () if not set
    pub fn location() -> Option<Location> {
        Some(runtime().ok()?.user.get_location()?)
    }

    /// Gets the user's quiet hours
    ///
    /// # Returns
    /// A map with 'start' and 'end' fields, or () if not set
    pub fn quiet_hours() -> Option<QuietHours> {
        Some(runtime().ok()?.user.get_quiet_hours()?)
    }

    /// Gets the user's birthday as a Unix timestamp
    ///
    /// # Returns
    /// The birthday timestamp
    pub fn birthday() -> Option<DateTime<Utc>> {
        Some(runtime().ok()?.user.get_birthday()?)
    }

    /// Gets the user's voice profile ID
    ///
    /// # Returns
    /// The voice profile ID, or () if not set
    pub fn voice_profile_id() -> Option<ImmutableString> {
        Some(ImmutableString::from(
            runtime().ok()?.user.get_voice_profile_id()?,
        ))
    }

    /// Gets the user's language preference
    ///
    /// # Returns
    /// The language code (e.g., "en"), defaults to "en" if not set
    pub fn language() -> ImmutableString {
        match runtime() {
            Ok(c) => ImmutableString::from(c.user.get_language()),
            Err(_) => ImmutableString::from(lang()),
        }
    }
}
