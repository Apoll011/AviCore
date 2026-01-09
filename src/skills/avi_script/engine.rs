use rhai::Engine;
use rhai::packages::Package;
use rhai_url::UrlPackage;

const MAX_MEMORY: usize = 10 * 1024 * 1024; // 10MB
const MAX_STACK: isize = 1024 * 1024; // 1MB

fn constraint_engine(engine: &mut Engine) {
    engine.set_max_array_size(256);
    engine.set_max_map_size(256);
    engine.set_max_functions(64);
    engine.set_max_modules(64);
}

fn register_types(engine: &mut Engine) {
    engine
        .build_type::<crate::dialogue::intent::Intent>()
        .build_type::<crate::dialogue::intent::IntentInfo>()
        .build_type::<crate::dialogue::intent::Slot>()
        .build_type::<crate::dialogue::intent::SlotRange>()
        .build_type::<crate::dialogue::intent::SlotValue>()
        .build_type::<crate::config::Setting>()
        .build_type::<crate::dialogue::lang_parse::ExtractNumbers>();
}

pub fn create_avi_script_engine() -> Result<Engine, Box<dyn std::error::Error>> {
    let mut engine = Engine::new();

    constraint_engine(&mut engine);

    register_types(&mut engine);

    super::syntax::add(&mut engine)?;

    super::module::add(&mut engine);

    super::functions::add(&mut engine);

    let url = UrlPackage::new();
    url.register_into_engine(&mut engine);

    Ok(engine)
}
