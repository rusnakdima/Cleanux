use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::collections::HashMap;

#[derive(Clone)]
pub struct AppLogger {
  level: LevelFilter,
  enabled: bool,
  level_toggles: HashMap<String, bool>,
  source_toggles: HashMap<String, bool>,
}

impl AppLogger {
  pub fn new() -> Self {
    let enabled = std::env::var("LOG_ENABLED")
      .unwrap_or_else(|_| "true".to_string())
      .parse::<bool>()
      .unwrap_or(true);
    
    let level_str = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let level = match level_str.to_lowercase().as_str() {
      "trace" => LevelFilter::Trace,
      "debug" => LevelFilter::Debug,
      "info" => LevelFilter::Info,
      "warn" => LevelFilter::Warn,
      "error" => LevelFilter::Error,
      _ => LevelFilter::Info,
    };
    
    // Level toggles
    let mut level_toggles = HashMap::new();
    level_toggles.insert("debug".to_string(), std::env::var("LOG_LEVEL_DEBUG").unwrap_or_else(|_| "true".to_string()).parse::<bool>().unwrap_or(true));
    level_toggles.insert("info".to_string(), std::env::var("LOG_LEVEL_INFO").unwrap_or_else(|_| "true".to_string()).parse::<bool>().unwrap_or(true));
    level_toggles.insert("warn".to_string(), std::env::var("LOG_LEVEL_WARN").unwrap_or_else(|_| "true".to_string()).parse::<bool>().unwrap_or(true));
    level_toggles.insert("error".to_string(), std::env::var("LOG_LEVEL_ERROR").unwrap_or_else(|_| "true".to_string()).parse::<bool>().unwrap_or(true));
    
    // Source toggles
    let mut source_toggles = HashMap::new();
    source_toggles.insert("route".to_string(), std::env::var("LOG_SOURCE_ROUTE").unwrap_or_else(|_| "true".to_string()).parse::<bool>().unwrap_or(true));
    source_toggles.insert("service".to_string(), std::env::var("LOG_SOURCE_SERVICE").unwrap_or_else(|_| "true".to_string()).parse::<bool>().unwrap_or(true));
    source_toggles.insert("helper".to_string(), std::env::var("LOG_SOURCE_HELPER").unwrap_or_else(|_| "true".to_string()).parse::<bool>().unwrap_or(true));
    source_toggles.insert("security".to_string(), std::env::var("LOG_SOURCE_SECURITY").unwrap_or_else(|_| "true".to_string()).parse::<bool>().unwrap_or(true));
    source_toggles.insert("user".to_string(), std::env::var("LOG_SOURCE_USER").unwrap_or_else(|_| "true".to_string()).parse::<bool>().unwrap_or(true));
    
    Self { level, enabled, level_toggles, source_toggles }
  }

  pub fn init(&self) {
    if !self.enabled {
      return;
    }
    
    Builder::new()
      .format(|buf, record| {
        writeln!(
          buf,
          "[{}] {} - {}",
          chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
          record.level(),
          record.args()
        )
      })
      .filter(None, self.level)
      .init();
  }

  pub fn is_enabled(&self) -> bool {
    self.enabled
  }
  
  pub fn is_level_enabled(&self, level: &str) -> bool {
    self.level_toggles.get(level).copied().unwrap_or(true)
  }
  
  pub fn is_source_enabled(&self, source: &str) -> bool {
    self.source_toggles.get(source).copied().unwrap_or(true)
  }

  pub fn should_log(&self, level: &str, source: &str) -> bool {
    if !self.enabled {
      return false;
    }
    if !self.is_level_enabled(level) {
      return false;
    }
    if !self.is_source_enabled(source) {
      return false;
    }
    true
  }
}

impl Default for AppLogger {
  fn default() -> Self {
    Self::new()
  }
}
