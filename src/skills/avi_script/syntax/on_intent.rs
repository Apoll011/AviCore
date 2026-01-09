use crate::dialogue::intent::Intent;
use rhai::{Dynamic, Engine, EvalAltResult, EvalContext, Expression, ImmutableString};

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
    let intent_name = inputs[0].get_string_value().unwrap().to_string();
    let block = &inputs[1];

    let i_name = context.scope().get_value::<ImmutableString>("INTENT_NAME");

    if i_name.is_some() && i_name.unwrap().eq_ignore_ascii_case(&intent_name) {
        let scope = context.scope_mut();
        scope.push_constant("name", intent_name);
        scope.push_constant("intent", scope.get_value::<Intent>("INTENT").unwrap());

        let _ = context.eval_expression_tree(block);
    }

    Ok(Dynamic::UNIT)
}
