use crate::dialogue::intent::Intent;
use rhai::{Dynamic, Engine, EvalAltResult, EvalContext, Expression, ImmutableString, Position};

pub fn add(engine: &mut Engine) -> Result<(), Box<EvalAltResult>> {
    engine.register_custom_syntax(
        ["on_intent", "$string$", "$block$"],
        false,
        on_intent_syntax_handler,
    )?;
    Ok(())
}

fn on_intent_syntax_handler(
    context: &mut EvalContext,
    inputs: &[Expression],
) -> Result<Dynamic, Box<EvalAltResult>> {
    let intent_name = inputs[0]
        .get_string_value()
        .ok_or(Box::new(EvalAltResult::ErrorRuntime(
            Dynamic::from("Expected Intent Name!"),
            Position::NONE,
        )))?
        .to_string();
    let block = &inputs[1];

    if let Some(intent_name_sent) = context.scope().get_value::<ImmutableString>("INTENT_NAME")
        && intent_name_sent.eq_ignore_ascii_case(&intent_name)
    {
        let scope = context.scope_mut();

        scope.push_constant("name", intent_name.clone());
        scope.push_constant(
            "intent",
            scope
                .get_value::<Intent>("INTENT")
                .ok_or("Expected Intent data")?,
        );
        let _ = context.eval_expression_tree(block);
    }

    Ok(Dynamic::UNIT)
}
