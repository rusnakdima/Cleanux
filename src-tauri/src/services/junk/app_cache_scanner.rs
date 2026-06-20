use crate::models::AppError;
use crate::services::junk::types::{JunkCategory, JunkItem};
use crate::utils::common_paths::CommonPath;
use crate::utils::{calculate_dir_size, remove_dir_contents};
pub struct ApplicationCacheScanner;
impl ApplicationCacheScanner {
  pub fn scan() -> Result<Vec<JunkItem>, AppError> {
    let mut items = Vec::new();
    let app_cache_paths = [
      (CommonPath::FlatpakCache, "Flatpak"),
      (CommonPath::FlatpakAlt, "Flatpak (alt)"),
      (CommonPath::SnapCache, "Snap"),
      (CommonPath::SnapAlt, "Snap (alt)"),
    ];
    for (common_path, name) in app_cache_paths {
      let path = match common_path.path() {
        Some(p) => p,
        None => continue,
      };
      if path.exists() {
        let (size, file_count) = calculate_dir_size(&path).unwrap_or((0, 0));
        items.push(JunkItem {
          path: path.to_string_lossy().into_owned(),
          size,
          category: JunkCategory::Applications,
          description: format!("{} application cache", name),
          file_count: file_count as u32,
        });
      }
    }
    if let Some(app_image_cache) = CommonPath::AppImageCache.path() {
      if app_image_cache.exists() {
        let (size, file_count) = calculate_dir_size(&app_image_cache).unwrap_or((0, 0));
        items.push(JunkItem {
          path: app_image_cache.to_string_lossy().into_owned(),
          size,
          category: JunkCategory::Applications,
          description: "AppImage cache".to_string(),
          file_count: file_count as u32,
        });
      }
    }
    Ok(items)
  }
  pub fn clean() -> Result<u64, AppError> {
    let paths = [
      (CommonPath::FlatpakCache, "Flatpak"),
      (CommonPath::FlatpakAlt, "Flatpak alt"),
      (CommonPath::SnapCache, "Snap"),
      (CommonPath::SnapAlt, "Snap alt"),
      (CommonPath::AppImageCache, "AppImage"),
    ];
    let mut cleaned_count = 0u64;
    for (common_path, _name) in paths {
      if let Some(path) = common_path.path() {
        if path.exists() {
          cleaned_count += remove_dir_contents(&path).unwrap_or(0);
        }
      }
    }
    Ok(cleaned_count)
  }
}
