use crate::ctx::runtime;
use crate::dialogue::reply::{RequestReply, ValidatorErasure};
use crate::dialogue::response::{
    AnyValidator, BoolValidator, ListOrNoneValidator, MappedValidator, OptionalValidator,
};
use crate::dialogue::utils::speak;
use crate::skills::avi_script::helpers::get_skill_context;
use crate::user::user_name;
use crate::{get_ctx, rt_spawn, speak};
use log::error;
use rhai::plugin::*;
use rhai::{Dynamic, Position};
use std::collections::HashMap;

#[export_module]
pub mod dialogue_module {
    /// Creates a validator that accepts any input
    ///
    /// # Returns
    /// An AnyValidator object
    #[rhai_fn(global)]
    pub fn any_validator() -> AnyValidator {
        AnyValidator::new()
    }

    /// Creates a validator that accepts a list of items or nothing
    ///
    /// # Arguments
    /// * `allowed_values` - A list of accepted string values
    ///
    /// # Returns
    /// A ListOrNoneValidator object
    #[rhai_fn(global)]
    pub fn list_or_none_validator(allowed_values: Vec<String>) -> ListOrNoneValidator {
        ListOrNoneValidator::new(allowed_values)
    }

    /// Creates a validator that makes another validator optional
    ///
    /// # Returns
    /// An OptionalValidator object
    #[rhai_fn(global)]
    pub fn optional_validator() -> OptionalValidator {
        OptionalValidator::new()
    }

    /// Creates a validator that accepts boolean input (yes/no)
    ///
    /// # Arguments
    /// * `fuzzy` - Whether to use fuzzy matching
    ///
    /// # Returns
    /// A BoolValidator object
    #[rhai_fn(global)]
    pub fn bool_validator(fuzzy: bool) -> BoolValidator {
        BoolValidator::new(fuzzy)
    }

    /// Creates a validator that maps string input to values
    ///
    /// # Arguments
    /// * `map` - A map of possible inputs to their string values
    ///
    /// # Returns
    /// A MappedValidator object
    #[rhai_fn(global, return_raw)]
    pub fn mapped_validator_string(
        map: rhai::Map,
    ) -> Result<MappedValidator<String>, Box<rhai::EvalAltResult>> {
        let mut mappings = HashMap::new();
        for (k, v) in map {
            mappings.insert(
                k.to_string(),
                v.clone().try_cast::<String>().ok_or_else(|| {
                    Box::new(rhai::EvalAltResult::ErrorMismatchDataType(
                        "String".to_string(),
                        v.type_name().to_string(),
                        Position::NONE,
                    ))
                })?,
            );
        }
        Ok(MappedValidator::<String>::new(mappings))
    }

    /// Creates a validator that maps string input to i32 values
    ///
    /// # Arguments
    /// * `map` - A map of possible inputs to their i32 values
    ///
    /// # Returns
    /// A MappedValidator object
    #[rhai_fn(global, return_raw)]
    pub fn mapped_validator_i32(
        map: rhai::Map,
    ) -> Result<MappedValidator<i32>, Box<rhai::EvalAltResult>> {
        let mut mappings = HashMap::new();
        for (k, v) in map {
            mappings.insert(
                k.to_string(),
                v.as_int().map_err(|_e| {
                    Box::new(rhai::EvalAltResult::ErrorMismatchDataType(
                        "int".to_string(),
                        v.type_name().to_string(),
                        Position::NONE,
                    ))
                })? as i32,
            );
        }
        Ok(MappedValidator::<i32>::new(mappings))
    }

    /// Creates a validator that maps string input to i64 values
    ///
    /// # Arguments
    /// * `map` - A map of possible inputs to their i64 values
    ///
    /// # Returns
    /// A MappedValidator object
    #[rhai_fn(global, return_raw)]
    pub fn mapped_validator_i64(
        map: rhai::Map,
    ) -> Result<MappedValidator<i64>, Box<rhai::EvalAltResult>> {
        let mut mappings = HashMap::new();
        for (k, v) in map {
            mappings.insert(
                k.to_string(),
                v.as_int().map_err(|_e| {
                    Box::new(rhai::EvalAltResult::ErrorMismatchDataType(
                        "int".to_string(),
                        v.type_name().to_string(),
                        Position::NONE,
                    ))
                })?,
            );
        }
        Ok(MappedValidator::<i64>::new(mappings))
    }

    /// Creates a validator that maps string input to f32 values
    ///
    /// # Arguments
    /// * `map` - A map of possible inputs to their f32 values
    ///
    /// # Returns
    /// A MappedValidator object
    #[rhai_fn(global, return_raw)]
    pub fn mapped_validator_f32(
        map: rhai::Map,
    ) -> Result<MappedValidator<f32>, Box<rhai::EvalAltResult>> {
        let mut mappings = HashMap::new();
        for (k, v) in map {
            mappings.insert(
                k.to_string(),
                v.as_float().map_err(|_e| {
                    Box::new(rhai::EvalAltResult::ErrorMismatchDataType(
                        "float".to_string(),
                        v.type_name().to_string(),
                        Position::NONE,
                    ))
                })? as f32,
            );
        }
        Ok(MappedValidator::<f32>::new(mappings))
    }

    /// Creates a validator that maps string input to f64 values
    ///
    /// # Arguments
    /// * `map` - A map of possible inputs to their f64 values
    ///
    /// # Returns
    /// A MappedValidator object
    #[rhai_fn(global, return_raw)]
    pub fn mapped_validator_f64(
        map: rhai::Map,
    ) -> Result<MappedValidator<f64>, Box<rhai::EvalAltResult>> {
        let mut mappings = HashMap::new();
        for (k, v) in map {
            mappings.insert(
                k.to_string(),
                v.as_float().map_err(|_e| {
                    Box::new(rhai::EvalAltResult::ErrorMismatchDataType(
                        "float".to_string(),
                        v.type_name().to_string(),
                        Position::NONE,
                    ))
                })?,
            );
        }
        Ok(MappedValidator::<f64>::new(mappings))
    }

    /// Creates a validator that maps string input to boolean values
    ///
    /// # Arguments
    /// * `map` - A map of possible inputs to their boolean values
    ///
    /// # Returns
    /// A MappedValidator object
    #[rhai_fn(global, return_raw)]
    pub fn mapped_validator_bool(
        map: rhai::Map,
    ) -> Result<MappedValidator<bool>, Box<rhai::EvalAltResult>> {
        let mut mappings = HashMap::new();
        for (k, v) in map {
            mappings.insert(
                k.to_string(),
                v.as_bool().map_err(|_e| {
                    Box::new(rhai::EvalAltResult::ErrorMismatchDataType(
                        "bool".to_string(),
                        v.type_name().to_string(),
                        Position::NONE,
                    ))
                })?,
            );
        }
        Ok(MappedValidator::<bool>::new(mappings))
    }

    /// Speaks a given text
    ///
    /// # Arguments
    /// * `text` - The text to speak
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(global)]
    pub fn say(text: String) {
        speak(&text, true);
    }

    /// Speaks a given text only once
    ///
    /// # Arguments
    /// * `text` - The text to speak
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(global)]
    pub fn say_once(text: String) {
        speak!(&text);
    }

    /// Makes the device start listening for voice input
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(global)]
    pub fn listen() {
        device_listen();
    }

    /// Repeats the last thing spoken or heard
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(global)]
    pub fn repeat() {
        if let Some(v) = get_ctx!("utterance.last") {
            speak!(&v.to_string())
        };
    }

    /// Requests the user's attention and starts listening
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(global, name = "request_attention")]
    pub fn request_attention() {
        speak!(&format!("{}!", user_name()));
        device_listen();
    }

    /// Asks the user a yes/no question and handles the response
    ///
    /// # Arguments
    /// * `question_locale_id` - The locale ID of the question to ask
    /// * `handler` - The name of the function to call with the result
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(global, return_raw)]
    pub fn confirm(
        ctx: NativeCallContext,
        question_locale_id: String,
        handler: String,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(rhai::EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        speak!(
            &skill_context
                .languages
                .get_translation(&question_locale_id)
                .unwrap()
        );

        let validator = Dynamic::from(BoolValidator::new(false));
        handle_on_reply(handler, validator, skill_context.info.name);
        Ok(())
    }

    /// Registers a handler for the next user response
    ///
    /// # Arguments
    /// * `handler` - The name of the function to call with the response
    /// * `validator` - The validator to use for the response
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(global, return_raw)]
    pub fn on_reply(
        ctx: NativeCallContext,
        handler: String,
        validator: Dynamic,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        let skill_context = get_skill_context(&ctx)
            .map_err(|e| Box::new(rhai::EvalAltResult::ErrorRuntime(e.into(), Position::NONE)))?;
        handle_on_reply(handler, validator, skill_context.info.name);
        Ok(())
    }
}

fn device_listen() {
    todo!()
}

fn handle_on_reply(handler: String, validator: Dynamic, skill_name: String) {
    let validator_erased: Box<dyn ValidatorErasure> = if validator.is::<BoolValidator>() {
        Box::new(validator.cast::<BoolValidator>())
    } else if validator.is::<AnyValidator>() {
        Box::new(validator.cast::<AnyValidator>())
    } else if validator.is::<ListOrNoneValidator>() {
        Box::new(validator.cast::<ListOrNoneValidator>())
    } else if validator.is::<OptionalValidator>() {
        Box::new(validator.cast::<OptionalValidator>())
    } else if validator.is::<MappedValidator<String>>() {
        Box::new(validator.cast::<MappedValidator<String>>())
    } else if validator.is::<MappedValidator<i32>>() {
        Box::new(validator.cast::<MappedValidator<i32>>())
    } else if validator.is::<MappedValidator<i64>>() {
        Box::new(validator.cast::<MappedValidator<i64>>())
    } else if validator.is::<MappedValidator<f32>>() {
        Box::new(validator.cast::<MappedValidator<f32>>())
    } else if validator.is::<MappedValidator<f64>>() {
        Box::new(validator.cast::<MappedValidator<f64>>())
    } else if validator.is::<MappedValidator<bool>>() {
        Box::new(validator.cast::<MappedValidator<bool>>())
    } else {
        error!("Unknown validator type: {}", validator.type_name());
        return;
    };

    rt_spawn! {
        if let Ok(c) = runtime() { c.reply_manager
        .set_reply(RequestReply {
            skill_request: skill_name,
            handler,
            validator: validator_erased,
        })
        .await };
    }
}
