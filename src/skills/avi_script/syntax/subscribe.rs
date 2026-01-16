use crate::utils::{Event, EventType};
use rhai::{Dynamic, Engine, EvalAltResult, EvalContext, Expression, ImmutableString, Position};

pub fn add(engine: &mut Engine) -> Result<(), Box<EvalAltResult>> {
    engine.register_custom_syntax(
        [
            "subscribe",
            "$ident$",
            "$string$",
            "as",
            "<",
            "$ident$",
            ">",
            "$block$",
        ],
        true,
        on_sub_syntax_handler,
    )?;
    Ok(())
}

fn on_sub_syntax_handler(
    context: &mut EvalContext,
    inputs: &[Expression],
) -> Result<Dynamic, Box<EvalAltResult>> {
    let e_type = inputs[0]
        .get_string_value()
        .ok_or(Box::new(EvalAltResult::ErrorRuntime(
            Dynamic::from("Expected Event type Name!"),
            Position::NONE,
        )))?;
    let event = inputs[1]
        .get_string_value()
        .ok_or(Box::new(EvalAltResult::ErrorRuntime(
            Dynamic::from("Expected Event Name!"),
            Position::NONE,
        )))?;
    let ident = inputs[2]
        .get_string_value()
        .ok_or(Box::new(EvalAltResult::ErrorRuntime(
            Dynamic::from("Expected Identifier Name!"),
            Position::NONE,
        )))?;
    let func = &inputs[3];

    let event_name = context.scope().get_value::<ImmutableString>("EVENT_NAME");
    let event_data = context.scope().get_value::<Dynamic>("EVENT_DATA");
    let event_from = context.scope().get_value::<Dynamic>("EVENT_FROM");

    let expected_type =
        EventType::from(e_type).ok_or(Box::new(EvalAltResult::ErrorCustomSyntax(
            "Expected topic or event".to_string(),
            vec![e_type.to_string()],
            inputs[0].position(),
        )))?;

    if let Some(event_name) = event_name {
        let received_event = Event::get_event(event_name.to_string())?;
        if received_event.event_name.eq_ignore_ascii_case(event)
            && received_event.event_type != expected_type
        {
            return Ok(Dynamic::UNIT);
        }

        let scope = context.scope_mut();
        scope.push_constant(ident, event_data.ok_or("Expected Data")?);
        scope.push_constant("from", event_from.ok_or("Expected Data")?);
        let _ = context.eval_expression_tree(func);
    };

    Ok(Dynamic::UNIT)
}
