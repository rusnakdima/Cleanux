/* security/privilege.rs - Privilege operation auditing */
use chrono::Local;
#[derive(Debug, Clone)]
pub struct PrivilegeOperation {
  pub timestamp: String,
  pub operation: String,
  pub target: String,
  pub result: String,
  pub user: String,
}
impl PrivilegeOperation {
  pub fn new(operation: &str, target: &str) -> Self {
    Self {
      timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
      operation: operation.to_string(),
      target: target.to_string(),
      result: String::new(),
      user: std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()),
    }
  }
  pub fn with_result(mut self, result: &str) -> Self {
    self.result = result.to_string();
    self
  }
}
