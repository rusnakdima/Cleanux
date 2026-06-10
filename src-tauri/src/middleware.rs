use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use crate::models::ResponseModel;
use crate::models::ResponseStatus;

static COMMAND_COUNT: AtomicU64 = AtomicU64::new(0);
static ERROR_COUNT: AtomicU64 = AtomicU64::new(0);

pub struct LoggingMiddleware;

impl LoggingMiddleware {
    pub fn log_command(name: &str, start: Instant) {
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(100) {
            log::warn!("[SLOW] Command '{}' took {:?}", name, elapsed);
        } else {
            log::info!("[CMD] {} took {:?}", name, elapsed);
        }
    }
}

pub struct ErrorHandler;

impl ErrorHandler {
    pub fn handle<E: std::fmt::Display>(error: E) -> ResponseModel {
        ResponseModel {
            status: ResponseStatus::Error,
            message: error.to_string(),
            data: crate::models::DataValue::String(String::new()),
        }
    }
}

pub struct MetricsCollector;

impl MetricsCollector {
    pub fn record_command() {
        COMMAND_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_error() {
        ERROR_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_stats() -> (u64, u64) {
        (
            COMMAND_COUNT.load(Ordering::SeqCst),
            ERROR_COUNT.load(Ordering::SeqCst),
        )
    }
}