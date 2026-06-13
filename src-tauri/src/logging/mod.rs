use log::{error, warn, info, debug};

pub fn init() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();
}

pub fn log_error(source: &str, message: &str) {
    error!("[{}] {}", source, message);
}

pub fn log_warn(source: &str, message: &str) {
    warn!("[{}] {}", source, message);
}

pub fn log_info(source: &str, message: &str) {
    info!("[{}] {}", source, message);
}

pub fn log_debug(source: &str, message: &str) {
    debug!("[{}] {}", source, message);
}

#[macro_export]
macro_rules! log_error {
    ($source:expr, $($arg:tt)*) => {
        error!(concat!("[{}] ", $($arg)*), $source);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($source:expr, $($arg:tt)*) => {
        warn!(concat!("[{}] ", $($arg)*), $source);
    };
}

#[macro_export]
macro_rules! log_info {
    ($source:expr, $($arg:tt)*) => {
        info!(concat!("[{}] ", $($arg)*), $source);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($source:expr, $($arg:tt)*) => {
        debug!(concat!("[{}] ", $($arg)*), $source);
    };
}
