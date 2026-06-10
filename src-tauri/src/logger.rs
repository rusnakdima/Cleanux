use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

#[derive(Clone)]
pub struct Logger {
  level: LevelFilter,
}

impl Logger {
  pub fn new() -> Self {
    let level_str = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let level = match level_str.to_lowercase().as_str() {
      "trace" => LevelFilter::Trace,
      "debug" => LevelFilter::Debug,
      "info" => LevelFilter::Info,
      "warn" => LevelFilter::Warn,
      "error" => LevelFilter::Error,
      _ => LevelFilter::Info,
    };
    Self { level }
  }

  pub fn init(&self) {
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

  pub fn debug(&self, msg: &str) {
    log::debug!("{}", msg);
  }

  pub fn info(&self, msg: &str) {
    log::info!("{}", msg);
  }

  pub fn warn(&self, msg: &str) {
    log::warn!("{}", msg);
  }

  pub fn error(&self, msg: &str) {
    log::error!("{}", msg);
  }
}

impl Default for Logger {
  fn default() -> Self {
    Self::new()
  }
}
