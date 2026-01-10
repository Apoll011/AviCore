use rhai::{Dynamic, Engine, EvalAltResult, EvalContext, Expression};

pub fn add(engine: &mut Engine) -> Result<(), Box<EvalAltResult>> {
    engine.register_custom_syntax(["on_start", "$block$"], false, on_start_syntax_handler)?;
    Ok(())
}

fn on_start_syntax_handler(
    context: &mut EvalContext,
    inputs: &[Expression],
) -> Result<Dynamic, Box<EvalAltResult>> {
    let block = &inputs[0];

    if context.scope().get_value::<bool>("STARTED").is_none() {
        let scope = context.scope_mut();
        scope.push_constant("STARTED", true);

        let _ = context.eval_expression_tree(block);
    }

    Ok(Dynamic::UNIT)
}
