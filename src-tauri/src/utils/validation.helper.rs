/* sys lib */
use std::path::{Path, PathBuf};

use crate::security::PathValidator;

pub fn validate_path(path: &str) -> Result<PathBuf, String> {
  PathValidator::validate(path).map_err(|e| e.to_string())
}

pub fn is_allowed_path(path: &Path, home_dir: &Path) -> bool {
  let trash_dir = home_dir.join(".local/share/Trash");

  let allowed_with_trash: Vec<PathBuf> = vec![
    PathBuf::from("/home"),
    PathBuf::from("/var/cache"),
    PathBuf::from("/tmp"),
    PathBuf::from("/snap"),
    trash_dir,
  ];

  for allowed in &allowed_with_trash {
    if let Ok(allowed_canonical) = allowed.canonicalize() {
      if path.starts_with(&allowed_canonical) {
        return true;
      }
    }
  }

  false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_path_nonexistent() {
    let result = validate_path("/nonexistent/path/12345");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
      err.contains("does not exist") || err.contains("No such file"),
      "error: {}",
      err
    );
  }
}
