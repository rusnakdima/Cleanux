use crate::helpers::{calculate_dir_size, get_command_output, run_command, success_response};
use crate::models::{AppError, DataValue, ResponseModel};
use std::fs;
use std::path::Path;

pub struct PacmanService;

impl PacmanService {
  pub fn get_cache_size_internal() -> u64 {
    let cache_path = Path::new("/var/cache/pacman/pkg/");
    if cache_path.exists() {
      calculate_dir_size(cache_path)
        .map(|(size, _)| size)
        .unwrap_or(0)
    } else {
      0
    }
  }

  pub fn clean(keep_recent: u32) -> Result<ResponseModel, AppError> {
    let before_size = Self::get_cache_size_internal();

    let output = get_command_output(
      "sh",
      &[
        "-c",
        &format!(
          "ls -t /var/cache/pacman/pkg/ | tail -n +{}",
          keep_recent + 1
        ),
      ],
    )?;
    let packages: Vec<String> = output
      .lines()
      .filter(|s| !s.is_empty())
      .map(|s| s.to_string())
      .collect();

    let mut _freed: u64 = 0;
    for pkg in &packages {
      let path = Path::new("/var/cache/pacman/pkg/").join(pkg);
      if let Ok(metadata) = fs::metadata(&path) {
        _freed += metadata.len();
      }
      let _ = run_command("rm", &["-f", &path.to_string_lossy()]);
    }

    let after_size = Self::get_cache_size_internal();
    let actual_freed = before_size.saturating_sub(after_size);

    Ok(success_response(
      format!(
        "Pacman cache cleaned. Removed {} old packages. Freed {} bytes",
        packages.len(),
        actual_freed
      ),
      DataValue::Object(serde_json::json!({
          "command": format!("pacman cache clean (keep {})", keep_recent),
          "spaceFreed": actual_freed,
          "message": format!("Removed {} old packages", packages.len())
      })),
    ))
  }

  pub fn full_clean() -> Result<ResponseModel, AppError> {
    let before_size = Self::get_cache_size_internal();
    let (success, stderr, _) = run_command("pacman", &["-Scc", "--noconfirm"])?;

    if success {
      let after_size = Self::get_cache_size_internal();
      let freed = before_size.saturating_sub(after_size);
      Ok(success_response(
        format!("Pacman full cache clean completed. Freed {} bytes", freed),
        DataValue::Object(serde_json::json!({
            "command": "pacman -Scc --noconfirm",
            "spaceFreed": freed,
            "message": "Pacman full cache clean completed"
        })),
      ))
    } else {
      let err_msg = format!("Failed to run pacman -Scc: {}", stderr);
      Err(AppError::message(err_msg))
    }
  }
}
