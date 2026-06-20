use crate::models::AppError;
use crate::services::junk::types::{JunkCategory, JunkItem};
use crate::utils::{calculate_dir_size, remove_dir_contents};
use std::path::{Path, PathBuf};
pub struct SystemTempScanner;
impl SystemTempScanner {
  pub fn scan() -> Result<Vec<JunkItem>, AppError> {
    let mut items = Vec::new();
    let temp_paths = [
      ("/tmp", "User temporary files"),
      ("/var/tmp", "System temporary files"),
    ];
    for (path_str, description) in temp_paths {
      let path = Path::new(path_str);
      if path.exists() {
        let (size, file_count) = calculate_dir_size(path).unwrap_or((0, 0));
        items.push(JunkItem {
          path: path_str.to_string(),
          size,
          category: JunkCategory::System,
          description: description.to_string(),
          file_count: file_count as u32,
        });
      }
    }
    Ok(items)
  }
  pub fn clean() -> Result<u64, AppError> {
    let temp_paths: Vec<(PathBuf, &str)> = ["/tmp", "/var/tmp"]
      .iter()
      .map(|p| (PathBuf::from(p), *p))
      .collect();
    let mut cleaned_count = 0u64;
    for (path, _name) in temp_paths {
      if path.exists() {
        cleaned_count += remove_dir_contents(&path).unwrap_or(0);
      }
    }
    Ok(cleaned_count)
  }
}
