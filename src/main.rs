extern crate core;

mod actions;
mod api;
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

use crate::log::AviCoreLogger;
use crate::skills::avi_script::avi_librarymanager::get_lib_path;
use crate::start::start_avi;
use crate::utils::{generate_documentation, generate_dsl_definition};
use ::log::{error, info};
use clap::{Parser, Subcommand, ValueEnum};
use std::time::Duration;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(
    name = "AviCore",
    version = VERSION,
    author = "AviCore Team",
    about = "AviCore - Advanced Voice Interface Core System",
    long_about = "AviCore is a distributed voice interface system that enables seamless \
                  interaction across multiple nodes. It supports both CORE and NODE device \
                  types with optional CAN gateway integration for automotive applications."
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start the AviCore system
    #[command(about = "Launch AviCore in the specified mode")]
    Start {
        /// Device type to run as
        #[arg(
            long = "type",
            short = 't',
            default_value = "core",
            help = "Specify device type: CORE (main controller) or NODE (peripheral device)"
        )]
        dev_type: AviDeviceType,

        /// Enable CAN gateway functionality
        #[arg(
            long = "can-gateway",
            short = 'g',
            help = "Enable CAN bus gateway for automotive integration"
        )]
        gateway: bool,

        /// Configuration file path
        #[arg(
            long = "config",
            short = 'c',
            default_value = "./config",
            help = "Path to configuration path"
        )]
        config: String,

        /// Log level
        #[arg(
            long = "log-level",
            short = 'l',
            default_value = "info",
            help = "Set logging level: trace, debug, info, warn, error"
        )]
        log_level: Option<String>,
    },

    /// Generate comprehensive system documentation
    #[command(about = "Generate markdown documentation for all skills and APIs")]
    GenerateDocs {
        /// Output directory for documentation
        #[arg(
            long = "output",
            short = 'o',
            default_value = "./docs",
            help = "Directory where documentation will be generated"
        )]
        output: String,

        /// Include internal APIs
        #[arg(
            long = "include-internal",
            short = 'i',
            help = "Include internal/private API documentation"
        )]
        include_internal: bool,
    },

    /// Generate DSL (Domain Specific Language) definition
    #[command(about = "Generate DSL schema and grammar definitions")]
    GenerateDsl {
        /// Output path
        #[arg(
            long = "output",
            short = 'o',
            default_value = "./definitions",
            help = "Output path (defaults to stdout)"
        )]
        output: String,
    },

    /// Display version and build information
    #[command(about = "Show detailed version and build information")]
    Version {
        /// Show verbose build information
        #[arg(
            long = "verbose",
            short = 'v',
            help = "Display extended version info including enabled features"
        )]
        verbose: bool,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum AviDeviceType {
    /// Main controller node with full orchestration capabilities
    #[value(name = "core")]
    CORE,

    /// Peripheral node for distributed processing
    #[value(name = "node")]
    NODE,
}

impl std::fmt::Display for AviDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AviDeviceType::CORE => write!(f, "CORE"),
            AviDeviceType::NODE => write!(f, "NODE"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    AviCoreLogger::init();
    let args = Args::parse();

    match args.command {
        Commands::Start {
            dev_type,
            gateway,
            config,
            log_level,
        } => {
            ui::print_logo();

            ui::step(1, 7, "Initializing Environment");
            if let Some(level) = log_level {
                AviCoreLogger::set_level(&level);
            }
            //let pb = indicatif::ProgressBar::new_spinner();
            //pb.set_style(ui::spinner_style());
            //pb.set_message(format!("Loading configuration from: {}", config));
            //pb.enable_steady_tick(Duration::from_millis(120));
            //tokio::time::sleep(Duration::from_millis(800)).await;
            //pb.finish_with_message("Configuration Loaded");

            let is_core = matches!(dev_type, AviDeviceType::CORE);
            let mode_str = if is_core {
                "CORE CONTROLLER"
            } else {
                "PERIPHERAL NODE"
            };

            ui::step(2, 7, &format!("Booting sequence initiated: {}", mode_str));

            if gateway {
                info!("Enabled [Gateway Mode]");
            }

            info!("System ownership transferred to AviCore Reactor...");

            start_avi(is_core, gateway, config).await?;
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
