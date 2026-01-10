use rhai::Engine;

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
        .build_type::<crate::dialogue::lang_parse::ExtractNumbers>()
        .build_type::<crate::dialogue::lang_parse::ExtractNumber>()
        .build_type::<crate::dialogue::lang_parse::ExtractDuration>()
        .build_type::<crate::dialogue::lang_parse::ExtractDatetime>()
        .build_type::<crate::dialogue::lang_parse::IsFractional>()
        .build_type::<crate::dialogue::response::AnyValidator>()
        .build_type::<crate::dialogue::response::ListOrNoneValidator>()
        .build_type::<crate::dialogue::response::OptionalValidator>()
        .build_type::<crate::dialogue::response::BoolValidator>()
        .build_type::<crate::dialogue::response::MappedValidator>()
        .build_type::<crate::user::User>()
        .build_type::<crate::user::UserProfile>()
        .build_type::<crate::user::Location>()
        .build_type::<crate::user::UserPreferences>()
        .build_type::<crate::user::NotificationPreferences>()
        .build_type::<crate::user::QuietHours>()
        .build_type::<crate::user::VoiceData>()
        .build_type::<crate::user::Metadata>()
        .build_type::<crate::skills::skill_context::Manifest>()
        .build_type::<crate::skills::skill_context::SkillContext>();
}

pub fn create_avi_script_engine(static_m: bool) -> Result<Engine, Box<dyn std::error::Error>> {
    let mut engine = Engine::new();

    constraint_engine(&mut engine);

    register_types(&mut engine);

    if static_m {
        super::module::add_static_modules(&mut engine);
    } else {
        super::module::add(&mut engine);
    }

    super::syntax::add(&mut engine)?;

    Ok(engine)
}
