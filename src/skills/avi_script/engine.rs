use rhai::Engine;
use rhai::packages::Package;
use rhai_url::UrlPackage;

pub fn create_avi_script_engine() -> Result<Engine, Box<dyn std::error::Error>> {
    let mut engine = Engine::new();

    super::syntax::operators::add(&mut engine)?;
    super::syntax::on_start::add(&mut engine)?;
    super::syntax::on_end::add(&mut engine)?;
    super::syntax::on_intent::add(&mut engine)?;

    super::functions::json::add(&mut engine);
    
    let url = UrlPackage::new();
    url.register_into_engine(&mut engine);

    Ok(engine)
}
