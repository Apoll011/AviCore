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
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
pub enum AviDeviceType {
    /// Main controller node with full orchestration capabilities
    #[value(name = "core")]
    Core,

    /// Peripheral node for distributed processing
    #[value(name = "node")]
    Node,
}

impl std::fmt::Display for AviDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AviDeviceType::Core => write!(f, "CORE"),
            AviDeviceType::Node => write!(f, "NODE"),
        }
    }
}
