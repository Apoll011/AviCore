use chrono::Local;
use colored::*;
use log::{Level, LevelFilter, Log, Metadata, Record, info};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU8, Ordering};

static MAX_LEVEL: AtomicU8 = AtomicU8::new(LevelFilter::Trace as u8);

pub struct AviCoreLogger {
    file: Mutex<File>,
}

impl AviCoreLogger {
    pub fn init() {
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("avi.log")
            .expect("Could not open log file");

        let logger = AviCoreLogger {
            file: Mutex::new(log_file),
        };

        logger.write_to_file("=========================== New Init ===========================");

        log::set_boxed_logger(Box::new(logger))
            .map(|()| log::set_max_level(LevelFilter::Trace))
            .expect("Failed to initialize logger");
    }

    pub fn set_level(level_str: &str) {
        let level_filter = match level_str.to_lowercase().as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            "off" => LevelFilter::Off,
            _ => {
                eprintln!("Invalid log level '{}', using Info", level_str);
                LevelFilter::Info
            }
        };

        MAX_LEVEL.store(level_filter as u8, Ordering::Relaxed);

        info!("Log level set to: {:?}", level_filter);
    }

    fn get_current_level() -> LevelFilter {
        match MAX_LEVEL.load(Ordering::Relaxed) {
            0 => LevelFilter::Off,
            1 => LevelFilter::Error,
            2 => LevelFilter::Warn,
            3 => LevelFilter::Info,
            4 => LevelFilter::Debug,
            5 => LevelFilter::Trace,
            _ => LevelFilter::Info,
        }
    }

    fn write_to_file(&self, message: &str) {
        if let Ok(mut file) = self.file.lock() {
            let _ = writeln!(file, "{}", message);
        }
    }

    fn format_console(&self, record: &Record) {
        let timestamp = Local::now().format("%H:%M:%S").to_string().blue();
        let level = match record.level() {
            Level::Error => format!("{:<5}", record.level().to_string())
                .red()
                .to_string(),
            Level::Warn => format!("{:<5}", record.level().to_string())
                .yellow()
                .to_string(),
            Level::Info => format!("{:<5}", record.level().to_string())
                .green()
                .to_string(),
            Level::Debug => format!("{:<5}", record.level().to_string())
                .purple()
                .to_string(),
            Level::Trace => format!("{:<5}", record.level().to_string())
                .blue()
                .to_string(),
        };

        println!("{} [{}] {}", timestamp, level, record.args());
    }

    fn format_file(&self, record: &Record) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");

        let log_line = format!(
            "{} [{:<5}] {}:{} - {}",
            timestamp,
            record.level(),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.args()
        );

        self.write_to_file(&log_line);
    }
}

impl Log for AviCoreLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.target().starts_with("avicore")
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if record.level() <= Self::get_current_level() {
                self.format_console(record);
            }

            self.format_file(record);
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.file.lock() {
            let _ = file.flush();
        }
    }
}
