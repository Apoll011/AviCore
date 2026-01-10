extern crate core;

mod actions;
mod api;
mod config;
mod context;
mod ctx;
mod dialogue;
mod log;
mod macros;
pub mod skills;
mod start;
mod user;
mod utils;

use crate::log::AviCoreLogger;
use crate::start::start_avi;
use crate::utils::{generate_documentation, generate_dsl_definition};
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "AviCore", about = "AviCore application", long_about = None)]
struct Args {
    /// Generate documentation and exit
    #[arg(long = "generate-docs", short = 'd')]
    generate_docs: bool,
    #[arg(long, short = 's')]
    start: bool,
    #[arg(long)]
    dsl: bool,
    #[arg(long = "type", default_value = "core", short = 't')]
    dev_type: AviDeviceType,
    #[arg(long = "can-gateway")]
    gateway: bool,
}
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum AviDeviceType {
    CORE,
    NODE,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    AviCoreLogger::init();

    let args = Args::parse();

    if args.generate_docs {
        return generate_documentation();
    } else if args.start {
        let is_core = match args.dev_type {
            AviDeviceType::NODE => false,
            AviDeviceType::CORE => true,
        };
        return start_avi(is_core, args.gateway).await;
    } else if args.dsl {
        return generate_dsl_definition();
    }

    Ok(())
}
