use rhai::{Engine, EvalAltResult};

mod on_end;
mod on_intent;
mod on_start;
mod operators;

pub fn add(engine: &mut Engine) -> Result<(), Box<EvalAltResult>> {
    operators::add(engine)?;
    on_start::add(engine)?;
    on_end::add(engine)?;
    on_intent::add(engine)?;

    Ok(())
}