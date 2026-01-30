use clap::{Parser, Subcommand};

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
        /// Configuration file path
        #[arg(long = "config", short = 'c', help = "Path to configuration path")]
        config: Option<String>,

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
