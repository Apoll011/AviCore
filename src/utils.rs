use crate::ctx::runtime;
#[allow(unused_imports)]
use crate::skills::avi_script::engine::create_avi_script_engine;
use log::info;
#[allow(unused_imports)]
use log::warn;

#[derive(Eq, PartialEq)]
pub enum EventType {
    TOPIC,
    EVENT,
}

impl EventType {
    pub fn from(string: &str) -> Option<EventType> {
        if string.eq_ignore_ascii_case("topic") {
            return Some(EventType::TOPIC);
        } else if string.eq_ignore_ascii_case("event") {
            return Some(EventType::EVENT);
        }
        None
    }
    pub fn name(&self) -> String {
        match self {
            EventType::TOPIC => "topic".to_string(),
            EventType::EVENT => "event".to_string(),
        }
    }
}

pub struct Event {
    pub event_type: EventType,
    pub event_name: String,
}

impl Event {
    pub fn string(&self) -> String {
        format!("{}:{}", self.event_type.name(), self.event_name)
    }

    pub fn get_event(event_string: String) -> Result<Event, String> {
        let event_vec = event_string.split(":").collect::<Vec<&str>>();
        if event_vec.len() != 2 {
            return Err("Expected topic/event:{name}".to_string());
        }
        Ok(Event {
            event_type: EventType::from(event_vec[0])
                .ok_or("Expected topic/event on the first hand side".to_string())?,
            event_name: event_vec[1].to_string(),
        })
    }
}

pub async fn core_id() -> Option<String> {
    match runtime() {
        Ok(c) => match c.device.get_core_id().await {
            Ok(v) => Some(v),
            Err(e) => {
                warn!("Error getting core id: {}", e.to_string());
                None
            }
        },
        Err(_) => None,
    }
}

pub fn generate_documentation(include_internal: bool) -> Result<(), Box<dyn std::error::Error>> {
    use rhai_autodocs::*;
    info!("Generating documentation");
    let engine = create_avi_script_engine(true, None)?;

    info!(
        "Got {} functions from engine",
        engine.gen_fn_signatures(include_internal).len()
    );

    let docs = export::options()
        .include_standard_packages(include_internal)
        .format_sections_with(export::SectionFormat::Tabs)
        .export(&*engine)?;

    info!("Trying to create dir ./docs");
    let path = "./docs";
    std::fs::remove_dir_all(path)?;
    std::fs::create_dir_all(path)?;
    info!("Created dir ./docs");

    info!("Generating glossary.");
    let glossary = generate::docusaurus_glossary()
        .with_slug("/api")
        .generate(&docs)?;

    std::fs::write(
        std::path::PathBuf::from_iter([path, "1-glossary.mdx"]),
        glossary,
    )?;
    info!("Generated glossary");

    for (name, doc) in generate::docusaurus().with_slug("/api").generate(&docs)? {
        info!("Generating doc file: {}.mdx", name);
        std::fs::write(
            std::path::PathBuf::from_iter([path, &format!("{}.mdx", &name)]),
            doc,
        )?;
    }
    info!("Generated Documentation");
    Ok(())
}

pub fn generate_dsl_definition(path: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Generating DSL definition");
    let engine = create_avi_script_engine(true, None)?;

    info!(
        "Got {} functions from engine",
        engine.gen_fn_signatures(true).len()
    );

    engine
        .definitions()
        .with_headers(true) // write headers in all files
        .include_standard_packages(true) // skip standard packages
        .write_to_dir(path.clone())?;

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rhai") {
            let mut new_path = path.clone();
            new_path.set_extension("avi");
            std::fs::rename(path, new_path)?;
        }
    }

    info!("Generated DSL definitions");
    Ok(())
}
