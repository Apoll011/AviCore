use crate::ctx::runtime;
use crate::dialogue::reply::RequestReply;
use crate::dialogue::response::BoolValidator;
use crate::dialogue::utils::speak;
use crate::user::user_name;
use crate::{get_ctx, register_skill_func, rt_spawn, speak};
use rhai::module_resolvers::StaticModuleResolver;
use rhai::{Dynamic, FuncRegistration, Module};

pub fn add(resolver: &mut StaticModuleResolver) {
    let mut module = Module::new();

    FuncRegistration::new("any_validator")
        .set_into_module(&mut module, crate::dialogue::response::AnyValidator::new);
    FuncRegistration::new("list_or_none_validator").set_into_module(
        &mut module,
        crate::dialogue::response::ListOrNoneValidator::new,
    );
    FuncRegistration::new("optional_validator").set_into_module(
        &mut module,
        crate::dialogue::response::OptionalValidator::new,
    );
    FuncRegistration::new("bool_validator").set_into_module(&mut module, BoolValidator::new);
    FuncRegistration::new("mapped_validator_string").set_into_module(
        &mut module,
        crate::dialogue::response::MappedValidator::<String>::new,
    );
    FuncRegistration::new("mapped_validator_i32").set_into_module(
        &mut module,
        crate::dialogue::response::MappedValidator::<i32>::new,
    );
    FuncRegistration::new("mapped_validator_i64").set_into_module(
        &mut module,
        crate::dialogue::response::MappedValidator::<i64>::new,
    );
    FuncRegistration::new("mapped_validator_f32").set_into_module(
        &mut module,
        crate::dialogue::response::MappedValidator::<f32>::new,
    );
    FuncRegistration::new("mapped_validator_f64").set_into_module(
        &mut module,
        crate::dialogue::response::MappedValidator::<f64>::new,
    );
    FuncRegistration::new("mapped_validator_bool").set_into_module(
        &mut module,
        crate::dialogue::response::MappedValidator::<bool>::new,
    );
    FuncRegistration::new("say").set_into_module(&mut module, say);
    FuncRegistration::new("say_once").set_into_module(&mut module, say_once);
    FuncRegistration::new("listen").set_into_module(&mut module, listen);
    FuncRegistration::new("repeat").set_into_module(&mut module, repeat);
    FuncRegistration::new("request_attention").set_into_module(&mut module, req_attention);

    register_skill_func!(&mut module, "confirm", (question_locale_id: String, handler: String), |skill_context| {
        speak!(
            &skill_context
                .languages
                .get_translation(&question_locale_id)
                .unwrap()
        );

        let validator = Dynamic::from(BoolValidator::new(false));
        handle_on_reply(handler.clone(), validator, skill_context.info.name.clone());
    });
    register_skill_func!(&mut module, "on_reply", (handler: String, validator: Dynamic), |skill_context| {
        handle_on_reply(handler.clone(), validator.clone(), skill_context.info.name.clone());
    });

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
    rt_spawn! {
        if let Ok(c) = runtime() { c.reply_manager
        .set_reply(RequestReply {
            skill_request: skill_name,
            handler,
            validator: validator.try_cast().unwrap(),
        })
        .await };
    }
}
