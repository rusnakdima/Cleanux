use crate::models::AppError;
use crate::services::junk::types::{JunkCategory, JunkItem};
use crate::utils::common_paths::CommonPath;
use crate::utils::{calculate_dir_size, remove_dir_contents};

pub struct BrowserCacheScanner;

impl BrowserCacheScanner {
  pub fn scan() -> Result<Vec<JunkItem>, AppError> {
    let mut items = Vec::new();

    let browser_paths = [
      (CommonPath::MozillaCache, "Firefox"),
      (CommonPath::ChromeCache, "Chrome"),
      (CommonPath::BraveCache, "Brave"),
      (CommonPath::EdgeCache, "Edge"),
    ];

    for (common_path, name) in browser_paths {
      let path = match common_path.path() {
        Some(p) => p,
        None => continue,
      };
      if path.exists() {
        let (size, file_count) = calculate_dir_size(&path).unwrap_or((0, 0));
        items.push(JunkItem {
          path: path.to_string_lossy().into_owned(),
          size,
          category: JunkCategory::Browser,
          description: format!("{} browser cache", name),
          file_count: file_count as u32,
        });
      }
    }

    Ok(items)
  }

  pub fn clean() -> Result<u64, AppError> {
    let browser_paths = [
      CommonPath::MozillaCache,
      CommonPath::ChromeCache,
      CommonPath::BraveCache,
      CommonPath::EdgeCache,
    ];

    let mut cleaned_count = 0u64;

    for common_path in browser_paths {
      if let Some(path) = common_path.path() {
        if path.exists() {
          cleaned_count += remove_dir_contents(&path).unwrap_or(0);
        }
      }
    }

    Ok(cleaned_count)
  }
}
