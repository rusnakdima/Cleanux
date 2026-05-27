/* sys lib */
use std::path::{Path, PathBuf};

const ALLOWED_DIRS: &[&str] = &["/home", "/var/cache", "/tmp", "/snap"];

pub fn validate_path(path: &str) -> Result<PathBuf, String> {
  let path_buf = PathBuf::from(path);

  let canonical = path_buf
    .canonicalize()
    .map_err(|e| format!("Failed to canonicalize path '{}': {}", path, e))?;

  if !canonical.exists() {
    return Err(format!("Path does not exist: {}", path));
  }

  if !canonical.is_file() && !canonical.is_dir() {
    return Err(format!("Path is neither a file nor directory: {}", path));
  }

  Ok(canonical)
}

pub fn is_allowed_path(path: &PathBuf, home_dir: &Path) -> bool {
  let trash_dir = home_dir.join(".local/share/Trash");

  let allowed_with_trash: Vec<PathBuf> = ALLOWED_DIRS
    .iter()
    .map(|s| PathBuf::from(s))
    .chain(std::iter::once(trash_dir.clone()))
    .collect();

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
