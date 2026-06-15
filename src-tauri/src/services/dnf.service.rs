use crate::helpers::{calculate_dir_size, run_command, success_response};
use crate::models::{AppError, DataValue, ResponseModel};
use std::path::Path;

pub struct DnfService;

impl DnfService {
  pub fn get_cache_size_internal() -> u64 {
    let cache_path = Path::new("/var/cache/dnf/");
    if cache_path.exists() {
      calculate_dir_size(cache_path)
        .map(|(size, _)| size)
        .unwrap_or(0)
    } else {
      0
    }
  }

  pub fn clean_all() -> Result<ResponseModel, AppError> {
    let before_size = Self::get_cache_size_internal();
    let (success, stderr, _) = run_command("dnf", &["clean", "all"])?;

    if success {
      let after_size = Self::get_cache_size_internal();
      let freed = before_size.saturating_sub(after_size);
      Ok(success_response(
        format!("DNF cache cleaned. Freed {} bytes", freed),
        DataValue::Object(serde_json::json!({
            "command": "dnf clean all",
            "spaceFreed": freed,
            "message": "DNF cache cleaned successfully"
        })),
      ))
    } else {
      let err_msg = format!("Failed to clean DNF cache: {}", stderr);
      Err(AppError::message(err_msg))
    }
  }
}
