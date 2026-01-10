use crate::ctx::runtime;
use log::{error, warn};

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

#[cfg(feature = "docs")]
pub fn generate_documentation() -> Result<(), Box<dyn std::error::Error>> {
    use rhai_autodocs::*;

    let engine = crate::skills::avi_script::engine::create_avi_script_engine()?;

    let docs = export::options()
        .include_standard_packages(true)
        .order_items_with(export::ItemsOrder::ByIndex)
        .format_sections_with(export::SectionFormat::Tabs)
        .export(&engine)?;

    println!("\n=== Engine Registration Debug ===");
    let signatures = engine.gen_fn_signatures(false);
    println!("Total registered functions: {}", signatures.len());

    // Show first 20 functions
    println!("\nFirst 20 functions:");
    for (i, sig) in signatures.iter().take(20).enumerate() {
        println!("  {}: {}", i + 1, sig);
    }


    let path = "./docs";
    std::fs::create_dir_all(path)?;

    let glossary = generate::docusaurus_glossary()
        .with_slug("/api")
        .generate(&docs)?;

    std::fs::write(
        std::path::PathBuf::from_iter([path, "1-glossary.mdx"]),
        glossary,
    )?;

    for (name, doc) in generate::docusaurus()
        .with_slug("/api")
        .generate(&docs)?
    {
        println!("Generating doc file: {}.mdx", name);
        std::fs::write(
            std::path::PathBuf::from_iter([path, &format!("{}.mdx", &name)]),
            doc,
        )?;
    }

    Ok(())
}

#[cfg(not(feature = "docs"))]
pub fn generate_documentation() -> Result<(), Box<dyn std::error::Error>> {
    error!("Documentation generation requires the 'docs' feature. Build with: cargo run --features docs -- --generate-docs");
    return Err("docs feature not enabled".into());
}