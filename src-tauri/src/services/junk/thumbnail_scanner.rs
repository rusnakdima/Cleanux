use crate::models::AppError;
use crate::services::junk::types::{JunkCategory, JunkItem};
use crate::utils::common_paths::CommonPath;
use crate::utils::{calculate_dir_size, remove_dir_contents};

pub struct ThumbnailCacheScanner;

impl ThumbnailCacheScanner {
  pub fn scan() -> Result<Vec<JunkItem>, AppError> {
    let path = match CommonPath::Thumbnails.path() {
      Some(p) => p,
      None => return Ok(Vec::new()),
    };

    if !path.exists() {
      return Ok(Vec::new());
    }

    let (size, file_count) = calculate_dir_size(&path).unwrap_or((0, 0));
    Ok(vec![JunkItem {
      path: path.to_string_lossy().into_owned(),
      size,
      category: JunkCategory::Thumbnails,
      description: "Image thumbnail cache".to_string(),
      file_count: file_count as u32,
    }])
  }

  pub fn clean() -> Result<u64, AppError> {
    let path = match CommonPath::Thumbnails.path() {
      Some(p) => p,
      None => {
        return Err(AppError::InvalidPath(
          "Home directory not found".to_string(),
        ))
      }
    };

    if !path.exists() {
      return Ok(0);
    }

    remove_dir_contents(&path)
  }
}
