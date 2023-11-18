use log::LevelFilter;
use simplelog::{CombinedLogger, TermLogger, Config, TerminalMode, ColorChoice, WriteLogger};

pub(crate) fn get_log_level() -> LevelFilter {
    match std::env::var("LOG_LEVEL") {
        Ok(level) => match level.as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Info,
        },
        Err(_) => LevelFilter::Info,
    }
}

pub(crate) fn init_logging() {
    // Set up logging
    let level = get_log_level();
    println!("Log level: {}", level);
    CombinedLogger::init(vec![
        TermLogger::new(
            level,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            std::fs::File::create("server.log").unwrap(),
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            std::fs::File::create("debug.log").unwrap(),
        ),
    ])
    .unwrap();
}