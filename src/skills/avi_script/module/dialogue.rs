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
use rhai::{Dynamic, FnPtr};
use std::collections::HashMap;

#[export_module]
pub mod dialogue_module {
    use crate::skills::avi_script::helpers::skill_context_def;
    use rhai::FnPtr;

    /// Creates a validator that accepts any input
    ///
    /// # Returns
    /// An AnyValidator object
    pub fn any_validator() -> AnyValidator {
        AnyValidator::new()
    }

    pub fn list_or_none_validator(allowed_values: Vec<String>) -> ListOrNoneValidator {
        ListOrNoneValidator::new(allowed_values)
    }

    pub fn optional_validator() -> OptionalValidator {
        OptionalValidator::new()
    }

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
    pub fn mapped_validator(map: rhai::Map) -> MappedValidator {
        let mut mappings = HashMap::new();
        for (k, v) in map {
            mappings.insert(k.to_string(), v);
        }
        MappedValidator::new(mappings)
    }

    /// Speaks a given text
    ///
    /// # Arguments
    /// * `text` - The text to speak
    ///
    /// # Returns
    /// Nothing
    pub fn say(text: ImmutableString) {
        speak(&text, true);
    }

    pub fn say_once(text: ImmutableString) {
        speak!(&text);
    }

    pub fn listen() {
        crate::dialogue::utils::listen();
    }

    pub fn repeat() {
        if let Some(v) = get_ctx!("utterance.last") {
            speak!(&v.to_string())
        };
    }

    /// Requests the user's attention and starts listening
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(name = "request_attention")]
    pub fn request_attention() {
        speak!(&format!("{}!", user_name()));
        crate::dialogue::utils::listen();
    }

    /// Asks the user a yes/no question and handles the response
    ///
    /// # Arguments
    /// * `question_locale_id` - The locale ID of the question to ask
    /// * `handler` - The name of the function to call with the result
    ///
    /// # Returns
    /// Nothing
    #[rhai_fn(return_raw)]
    pub fn confirm(
        ctx: NativeCallContext,
        question_locale_id: ImmutableString,
        handler: FnPtr,
    ) -> Result<(), Box<EvalAltResult>> {
        let skill_context = get_skill_context(&ctx).map_err(|_| {
            Box::new(EvalAltResult::ErrorRuntime(
                "Could not get the skill context".to_string().into(),
                Position::NONE,
            ))
        })?;

        speak!(
            &skill_context
                .languages
                .get_translation(&question_locale_id)
                .ok_or(Box::new(EvalAltResult::ErrorRuntime(
                    format!("Could not get translation for {}", question_locale_id).into(),
                    Position::NONE
                )))?
        );

        let handler_cloned = handler.clone();
        skill_context_def(ctx, move |skill| {
            handle_on_reply(
                handler_cloned.clone(),
                Box::new(BoolValidator::new(false)),
                skill.info.id.clone(),
            );
        });

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
    pub fn on_reply(ctx: NativeCallContext, handler: FnPtr, validator: Dynamic) {
        let validator_type = validator.type_name().to_string();
        let validator_cloned = validator.clone();
        let handler_cloned = handler.clone();

        skill_context_def(ctx, move |skill| {
            let v: Box<dyn ValidatorErasure> =
                if let Some(v) = validator_cloned.clone().try_cast::<AnyValidator>() {
                    Box::new(v)
                } else if let Some(v) = validator_cloned.clone().try_cast::<BoolValidator>() {
                    Box::new(v)
                } else if let Some(v) = validator_cloned.clone().try_cast::<ListOrNoneValidator>() {
                    Box::new(v)
                } else if let Some(v) = validator_cloned.clone().try_cast::<OptionalValidator>() {
                    Box::new(v)
                } else if let Some(v) = validator_cloned.clone().try_cast::<MappedValidator>() {
                    Box::new(v)
                } else {
                    error!("Invalid validator type: {}", validator_type);
                    return;
                };

            handle_on_reply(handler_cloned.clone(), v, skill.info.id.clone());
        });
    }
}

fn handle_on_reply(handler: FnPtr, validator: Box<dyn ValidatorErasure>, skill_name: String) {
    rt_spawn! {
        if let Ok(c) = runtime() { c.reply_manager
        .set_reply(RequestReply {
            skill_request: skill_name,
            handler,
            validator,
        })
        .await };
    }
}
