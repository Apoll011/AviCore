use rhai::{Dynamic, Engine, EvalAltResult, EvalContext, Expression};

pub fn add(engine: &mut Engine) -> Result<(), Box<EvalAltResult>> {
    engine.register_custom_syntax(["on_end", "$block$"], false, on_end_syntax_handler)?;
    Ok(())
}

fn on_end_syntax_handler(
    context: &mut EvalContext,
    inputs: &[Expression],
) -> Result<Dynamic, Box<EvalAltResult>> {
    let block = &inputs[0];

    if let Some(true) = context.scope().get_value::<bool>("END") {
        let _ = context.eval_expression_tree(block);
    }

    Ok(Dynamic::UNIT)
}
