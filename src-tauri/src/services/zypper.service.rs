use crate::utils::{calculate_dir_size, run_command, success_response};
use crate::models::{AppError, DataValue, Response};
use std::path::Path;
pub struct ZypperService;
impl ZypperService {
  pub fn get_cache_size_internal() -> u64 {
    let cache_path = Path::new("/var/cache/zypp/");
    if cache_path.exists() {
      calculate_dir_size(cache_path)
        .map(|(size, _)| size)
        .unwrap_or(0)
    } else {
      0
    }
  }
  pub fn clean() -> Result<Response, AppError> {
    let before_size = Self::get_cache_size_internal();
    let (success, stderr, _) = run_command("zypper", &["clean"])?;
    if success {
      let after_size = Self::get_cache_size_internal();
      let freed = before_size.saturating_sub(after_size);
      Ok(success_response(
        format!("Zypper cache cleaned. Freed {} bytes", freed),
        DataValue::Object(serde_json::json!({
            "command": "zypper clean",
            "spaceFreed": freed,
            "message": "Zypper cache cleaned successfully"
        })),
      ))
    } else {
      let err_msg = format!("Failed to clean Zypper cache: {}", stderr);
      Err(AppError::message(err_msg))
    }
  }
}
