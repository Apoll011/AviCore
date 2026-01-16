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
mod user;
mod utils;

use crate::log::AviCoreLogger;
use crate::skills::avi_script::avi_librarymanager::get_lib_path;
use crate::start::start_avi;
use crate::utils::{generate_documentation, generate_dsl_definition};
use ::log::info;
use clap::{Parser, Subcommand, ValueEnum};

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
            info!("AviCore v{}", VERSION);

            if let Some(level) = log_level {
                AviCoreLogger::set_level(&level);
            }

            info!("Loading configuration from: {}", config);

            let is_core = matches!(dev_type, AviDeviceType::CORE);

            info!(
                "Starting AviCore as {} device{}",
                dev_type,
                if gateway {
                    " with CAN gateway enabled"
                } else {
                    ""
                }
            );

            start_avi(is_core, gateway, config).await
        }

        Commands::GenerateDocs {
            output,
            include_internal,
        } => {
            info!("Generating documentation to: {}", output);
            if include_internal {
                info!("Including internal API documentation");
            }
            generate_documentation(include_internal)
        }

        Commands::GenerateDsl { output } => {
            info!("Generating DSL definition");
            info!("Output will be written to: {}", output);
            generate_dsl_definition(output)
        }

        Commands::Version { verbose } => {
            println!("AviCore v{}", VERSION);

            if verbose {
                println!("\nBuild Information:");
                println!("  Package: {}", env!("CARGO_PKG_NAME"));
                println!("  Version: {}", VERSION);
                println!("\nRuntime Information:");
                println!("  Library path: {}", get_lib_path().display());
                println!("  Platform: {}", std::env::consts::OS);
                println!("  Architecture: {}", std::env::consts::ARCH);

                println!("\nFor more information, visit: https://github.com/apoll011/AviCore");
            }

            Ok(())
        }
    }
}
