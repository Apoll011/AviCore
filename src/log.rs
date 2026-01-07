use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;

pub fn create_log() {
    match CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("avi.log").unwrap(),
        ),
    ]) {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to initialize logger: {}", e),
    }
}
