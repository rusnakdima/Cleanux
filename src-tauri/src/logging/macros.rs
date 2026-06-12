#[macro_export]
macro_rules! log_operation {
  ($service:expr, $source:expr, $operation:expr) => {{
    use std::time::Instant;
    let start = Instant::now();
    
    // Check all toggles via AppLogger
    let logger = crate::logging::AppLogger::new();
    let level_str = match stringify!($operation).contains("Result") {
      true => "info",
      false => "info",
    };
    
    if logger.should_log("info", $source) {
      log::info!("[{}] {} - start (source: {})", $service, stringify!($operation), $source);
    }
    
    let result = (move || -> Result<_, crate::models::ResponseModel> { $operation })();
    let elapsed = start.elapsed();
    
    if logger.should_log("info", $source) {
      match &result {
        Ok(_) => log::info!("[{}] {} - completed in {:?} (source: {})", $service, stringify!($operation), elapsed, $source),
        Err(e) => log::error!("[{}] {} - ERROR: {:?} in {:?} (source: {})", $service, stringify!($operation), e, elapsed, $source),
      }
    }
    result
  }};
}

#[macro_export]
macro_rules! log_operation_simple {
  ($service:expr, $source:expr, $operation:expr) => {{
    use std::time::Instant;
    let start = Instant::now();
    
    let logger = crate::logging::AppLogger::new();
    
    if logger.should_log("info", $source) {
      log::info!("[{}] {} - start (source: {})", $service, stringify!($operation), $source);
    }
    
    let result = (move || { $operation })();
    let elapsed = start.elapsed();
    
    if logger.should_log("info", $source) {
      log::info!("[{}] {} - completed in {:?} (source: {})", $service, stringify!($operation), elapsed, $source);
    }
    result
  }};
}

// Simple logging macro for one-shot logging
#[macro_export]
macro_rules! log_if_enabled {
  ($level:expr, $source:expr, $($arg:tt)*) => {{
    let logger = crate::logging::AppLogger::new();
    if logger.should_log($level, $source) {
      match $level {
        "debug" => log::debug!($($arg)*),
        "info" => log::info!($($arg)*),
        "warn" => log::warn!($($arg)*),
        "error" => log::error!($($arg)*),
        _ => log::info!($($arg)*),
      }
    }
  }};
}
