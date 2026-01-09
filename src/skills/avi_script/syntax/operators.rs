use rhai::{Dynamic, Engine, EvalAltResult, ImmutableString};

pub fn add(engine: &mut Engine) -> Result<(), Box<EvalAltResult>> {
    engine
        .register_custom_operator("or", 160)?
        .register_fn(
            "or",
            |a: Dynamic, b: Dynamic| {
                if a.to_string().is_empty() { b } else { a }
            },
        );

    engine
        .register_custom_operator("@@", 160)?
        .register_fn("@@", |a: ImmutableString, b: ImmutableString| {
            format!("{}{}", a, b)
        });
    Ok(())
}
