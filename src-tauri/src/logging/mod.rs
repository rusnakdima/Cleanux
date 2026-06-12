pub mod macros;
pub mod logger;

pub use logger::AppLogger;
pub use crate::{log_operation, log_operation_simple, log_if_enabled};
