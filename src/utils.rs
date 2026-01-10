use log::info;
use crate::ctx::runtime;
#[allow(unused_imports)]
use crate::skills::avi_script::engine::create_avi_script_engine;
#[allow(unused_imports)]
use log::warn;

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

pub fn generate_documentation() -> Result<(), Box<dyn std::error::Error>> {
    use rhai_autodocs::*;
    info!("Generating documentation");
    let engine = create_avi_script_engine(true)?;

    info!("Got {} functions from engine", engine.gen_fn_signatures(true).len());

    let docs = export::options()
        .include_standard_packages(true)
        .format_sections_with(export::SectionFormat::Tabs)
        .export(&engine)?;

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