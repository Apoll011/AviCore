use log::error;
use crate::ctx::runtime;
use crate::dialogue::reply::{RequestReply, ValidatorErasure};
use crate::dialogue::utils::speak;
use crate::user::user_name;
use crate::{get_ctx, register_skill_func, rt_spawn, speak};
use rhai::module_resolvers::StaticModuleResolver;
use rhai::{Dynamic, FuncRegistration, Module};
use crate::dialogue::response::{AnyValidator, BoolValidator, ListOrNoneValidator, MappedValidator, OptionalValidator};

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    FuncRegistration::new("any_validator")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Creates a validator that accepts any input",
            "/// ",
            "/// # Returns",
            "/// An AnyValidator object"
        ])
        .with_params_info(&[] as &[&str])
        .set_into_module(&mut module, AnyValidator::new);

    FuncRegistration::new("list_or_none_validator")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Creates a validator that accepts a list of items or nothing",
            "/// ",
            "/// # Returns",
            "/// A ListOrNoneValidator object"
        ])
        .with_params_info(&[] as &[&str])
        .set_into_module(&mut module, ListOrNoneValidator::new);

    FuncRegistration::new("optional_validator")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Creates a validator that makes another validator optional",
            "/// ",
            "/// # Arguments",
            "/// * `validator` - The validator to make optional",
            "/// ",
            "/// # Returns",
            "/// An OptionalValidator object"
        ])
        .with_params_info(&["validator: Dynamic"])
        .set_into_module(&mut module, OptionalValidator::new);

    FuncRegistration::new("bool_validator")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Creates a validator that accepts boolean input (yes/no)",
            "/// ",
            "/// # Arguments",
            "/// * `fuzzy` - Whether to use fuzzy matching",
            "/// ",
            "/// # Returns",
            "/// A BoolValidator object"
        ])
        .with_params_info(&["fuzzy: bool"])
        .set_into_module(&mut module, BoolValidator::new);

    FuncRegistration::new("mapped_validator_string")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Creates a validator that maps string input to values",
            "/// ",
            "/// # Arguments",
            "/// * `map` - A map of possible inputs to their string values",
            "/// ",
            "/// # Returns",
            "/// A MappedValidator object"
        ])
        .with_params_info(&["map: Map"])
        .set_into_module(&mut module, MappedValidator::<String>::new);

    FuncRegistration::new("mapped_validator_i32")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Creates a validator that maps string input to i32 values",
            "/// ",
            "/// # Arguments",
            "/// * `map` - A map of possible inputs to their i32 values",
            "/// ",
            "/// # Returns",
            "/// A MappedValidator object"
        ])
        .with_params_info(&["map: Map"])
        .set_into_module(&mut module, MappedValidator::<i32>::new);

    FuncRegistration::new("mapped_validator_i64")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Creates a validator that maps string input to i64 values",
            "/// ",
            "/// # Arguments",
            "/// * `map` - A map of possible inputs to their i64 values",
            "/// ",
            "/// # Returns",
            "/// A MappedValidator object"
        ])
        .with_params_info(&["map: Map"])
        .set_into_module(&mut module, MappedValidator::<i64>::new);

    FuncRegistration::new("mapped_validator_f32")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Creates a validator that maps string input to f32 values",
            "/// ",
            "/// # Arguments",
            "/// * `map` - A map of possible inputs to their f32 values",
            "/// ",
            "/// # Returns",
            "/// A MappedValidator object"
        ])
        .with_params_info(&["map: Map"])
        .set_into_module(&mut module, MappedValidator::<f32>::new);

    FuncRegistration::new("mapped_validator_f64")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Creates a validator that maps string input to f64 values",
            "/// ",
            "/// # Arguments",
            "/// * `map` - A map of possible inputs to their f64 values",
            "/// ",
            "/// # Returns",
            "/// A MappedValidator object"
        ])
        .with_params_info(&["map: Map"])
        .set_into_module(&mut module, MappedValidator::<f64>::new);

    FuncRegistration::new("mapped_validator_bool")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Creates a validator that maps string input to boolean values",
            "/// ",
            "/// # Arguments",
            "/// * `map` - A map of possible inputs to their boolean values",
            "/// ",
            "/// # Returns",
            "/// A MappedValidator object"
        ])
        .with_params_info(&["map: Map"])
        .set_into_module(&mut module, MappedValidator::<bool>::new);

    FuncRegistration::new("say")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Speaks a given text",
            "/// ",
            "/// # Arguments",
            "/// * `text` - The text to speak",
            "/// ",
            "/// # Returns",
            "/// Nothing"
        ])
        .with_params_info(&["text: String"])
        .set_into_module(&mut module, say);

    FuncRegistration::new("say_once")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Speaks a given text only once",
            "/// ",
            "/// # Arguments",
            "/// * `text` - The text to speak",
            "/// ",
            "/// # Returns",
            "/// Nothing"
        ])
        .with_params_info(&["text: String"])
        .set_into_module(&mut module, say_once);

    FuncRegistration::new("listen")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Makes the device start listening for voice input",
            "/// ",
            "/// # Returns",
            "/// Nothing"
        ])
        .with_params_info(&[] as &[&str])
        .set_into_module(&mut module, listen);

    FuncRegistration::new("repeat")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Repeats the last thing spoken or heard",
            "/// ",
            "/// # Returns",
            "/// Nothing"
        ])
        .with_params_info(&[] as &[&str])
        .set_into_module(&mut module, repeat);

    FuncRegistration::new("request_attention")
        .with_namespace(rhai::FnNamespace::Global)
        .with_comments(&[
            "/// Requests the user's attention and starts listening",
            "/// ",
            "/// # Returns",
            "/// Nothing"
        ])
        .with_params_info(&[] as &[&str])
        .set_into_module(&mut module, req_attention);

    register_skill_func!(
        &mut module,
        "confirm",
        (question_locale_id: String, handler: String),
        &[
            "/// Asks the user a yes/no question and handles the response",
            "/// ",
            "/// # Arguments",
            "/// * `question_locale_id` - The locale ID of the question to ask",
            "/// * `handler` - The name of the function to call with the result",
            "/// ",
            "/// # Returns",
            "/// Nothing"
        ],
        &["question_locale_id: String", "handler: String"],
        |skill_context| {
            speak!(
                &skill_context
                    .languages
                    .get_translation(&question_locale_id)
                    .unwrap()
            );

            let validator = Dynamic::from(BoolValidator::new(false));
            handle_on_reply(handler.clone(), validator, skill_context.info.name.clone());
        }
    );
    register_skill_func!(
        &mut module,
        "on_reply",
        (handler: String, validator: Dynamic),
        &[
            "/// Registers a handler for the next user response",
            "/// ",
            "/// # Arguments",
            "/// * `handler` - The name of the function to call with the response",
            "/// * `validator` - The validator to use for the response",
            "/// ",
            "/// # Returns",
            "/// Nothing"
        ],
        &["handler: String", "validator: Dynamic"],
        |skill_context| {
            handle_on_reply(handler.clone(), validator.clone(), skill_context.info.name.clone());
        }
    );

    resolver.insert("dialogue", module);
}

fn say(text: String) {
    speak(&text, true);
}

fn req_attention() {
    speak!(&format!("{}!", user_name()));
    device_listen();
}

fn device_listen() {
    todo!()
}

fn say_once(text: String) {
    speak!(&text);
}

fn listen() {
    device_listen();
}

fn repeat() {
    if let Some(v) = get_ctx!("utterance.last") {
        speak!(&v.to_string())
    };
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
