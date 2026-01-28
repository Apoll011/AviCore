extern crate core;

mod actions;
mod api;
mod cli_args;
mod config;
mod context;
mod ctx;
mod dialogue;
mod log;
mod macros;
mod skills;
mod start;
mod ui;
mod user;
mod utils;

use crate::cli_args::{Args, Commands};
use crate::log::AviCoreLogger;
use crate::skills::avi_script::avi_librarymanager::get_lib_path;
use crate::start::start_avi;
use crate::utils::{Setup, config_dir, generate_documentation, generate_dsl_definition};
use ::log::{error, info};
use clap::Parser;
use std::time::Duration;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    AviCoreLogger::init();
    let args = Args::parse();

    match args.command {
        Commands::Start { config, log_level } => {
            ui::print_logo();

            ui::step(1, 8, "Initializing Environment");
            if let Some(level) = log_level {
                AviCoreLogger::set_level(&level);
            }

            let config_w;

            if let Some(c) = config {
                config_w = c.as_str().into();
            } else {
                config_w = config_dir();
            }

            let s = Setup::new(&config_w);

            s.check().await;

            ui::step(2, 8, &format!("Booting sequence initiated"));

            info!("System ownership transferred to AviCore Reactor...");

            start_avi(config_w.display().to_string()).await?;
        }

        Commands::GenerateDocs {
            output,
            include_internal,
        } => {
            ui::print_logo();
            ui::step(1, 2, "Scanning Modules");

            let pb = indicatif::ProgressBar::new(100);
            pb.set_style(ui::main_progress_style());
            pb.set_message("Parsing Skills...");

            for _ in 0..40 {
                pb.inc(1);
                tokio::time::sleep(Duration::from_millis(10)).await;
            }

            ui::step(2, 2, "Generating Artifacts");
            if include_internal {
                info!("Scope Public + Internal API");
            } else {
                info!("Scope Public API Only");
            }

            match generate_documentation(include_internal) {
                Ok(_) => {
                    pb.finish_with_message("Done");
                    info!("Documentation generated at: {}", output);
                }
                Err(e) => {
                    pb.abandon();
                    error!("Failed to generate docs: {}", e);
                }
            }
        }

        Commands::GenerateDsl { output } => {
            ui::print_logo();
            ui::step(1, 1, "Compiling DSL Definitions");

            let pb = indicatif::ProgressBar::new_spinner();
            pb.set_style(ui::spinner_style());
            pb.enable_steady_tick(Duration::from_millis(80));
            pb.set_message("Generating Grammar...");

            match generate_dsl_definition(output.clone()) {
                Ok(_) => {
                    pb.finish_and_clear();
                    info!("DSL Definition Generated");
                    info!("Output: {}", &output);
                }
                Err(e) => {
                    pb.abandon();
                    error!("DSL Generation failed: {}", e);
                }
            }
        }

        Commands::Version { verbose } => {
            ui::print_logo();

            if verbose {
                println!(
                    "{}",
                    console::style("Build Information:").bold().underlined()
                );
                ui::info_line("Package", env!("CARGO_PKG_NAME"));
                ui::info_line("Version", VERSION);

                println!(
                    "\n{}",
                    console::style("Runtime Information:").bold().underlined()
                );
                ui::info_line("Library Path", &get_lib_path().display().to_string());
                ui::info_line("Platform", std::env::consts::OS);
                ui::info_line("Architecture", std::env::consts::ARCH);

                println!("\nFor more information, visit: https://github.com/apoll011/AviCore");
            }
        }
    }

    Ok(())
}
