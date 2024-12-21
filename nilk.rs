// Import the log crate for logging
use log::{info, LevelFilter};

// Import the env_logger crate for environment-based logging
use env_logger::Env;

fn main() {
    // Initialize the env_logger with a filter level of Debug
    env_logger::init_from_env(Env::default().filter_or("LOG_LEVEL", LevelFilter::Debug));

    // Log an info message
    info!("This is an info message");

    // You can also log debug, warn, and error messages
    log::debug!("This is a debug message");
    log::warn!("This is a warn message");
    log::error!("This is an error message");
}
