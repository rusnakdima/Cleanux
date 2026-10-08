//! TrashCleanerService — scans and empties the system trash
//!
//! Scans ~/.local/share/Trash/files and ~/.local/share/Trash/expunged.

use crate::infrastructure::sys_utils;
use tokio::fs;
use walkdir::WalkDir;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrashItem {
  pub path: String,
  pub size: u64,
  pub deleted_date: String,
}

pub struct TrashCleanerService;

impl TrashCleanerService {
  pub fn new() -> Self {
    Self
  }
}

impl Default for TrashCleanerService {
  fn default() -> Self {
    Self::new()
  }
}

impl TrashCleanerService {
  /// Scan the trash directories: ~/.local/share/Trash/files and expunged.
  pub async fn scan_trash(&self) -> Result<Vec<TrashItem>, String> {
    let mut items = Vec::new();
    let home = sys_utils::home_dir();

    let trash_files = home
      .join(".local")
      .join("share")
      .join("Trash")
      .join("files");
    let trash_expunged = home
      .join(".local")
      .join("share")
      .join("Trash")
      .join("expunged");

    // Scan Trash/files
    if trash_files.exists() {
      let file_items = self.scan_trash_dir(trash_files.as_path()).await?;
      items.extend(file_items);
    }

    // Scan Trash/expunged
    if trash_expunged.exists() {
      let expunged_items = self.scan_trash_dir(trash_expunged.as_path()).await?;
      items.extend(expunged_items);
    }

    Ok(items)
  }

  /// Empty the trash, returning bytes freed.
  pub async fn empty_trash(&self) -> Result<u64, String> {
    let mut bytes_freed: u64 = 0;
    let home = sys_utils::home_dir();

    let trash_files = home
      .join(".local")
      .join("share")
      .join("Trash")
      .join("files");
    let trash_expunged = home
      .join(".local")
      .join("share")
      .join("Trash")
      .join("expunged");

    // Get size before deletion
    if trash_files.exists() {
      bytes_freed += sys_utils::get_dir_size(&trash_files).await.unwrap_or(0);
      let _ = fs::remove_dir_all(&trash_files).await;
      // Recreate empty directory
      let _ = fs::create_dir_all(&trash_files).await;
    }

    if trash_expunged.exists() {
      bytes_freed += sys_utils::get_dir_size(&trash_expunged).await.unwrap_or(0);
      let _ = fs::remove_dir_all(&trash_expunged).await;
      // Recreate empty directory
      let _ = fs::create_dir_all(&trash_expunged).await;
    }

    Ok(bytes_freed)
  }

  async fn scan_trash_dir(&self, dir: &std::path::Path) -> Result<Vec<TrashItem>, String> {
    let mut items = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
      let path = entry.path();
      // Skip directories themselves
      if path == dir {
        continue;
      }

      if let Ok(metadata) = entry.metadata() {
        let size = if metadata.is_file() {
          metadata.len()
        } else {
          sys_utils::get_dir_size(path).await.unwrap_or(0)
        };

        let modified = metadata
          .modified()
          .map(|t| {
            let datetime: chrono::DateTime<chrono::Utc> = t.into();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
          })
          .unwrap_or_else(|_| "unknown".to_string());

        items.push(TrashItem {
          path: path.to_string_lossy().to_string(),
          size,
          deleted_date: modified,
        });
      }
    }

    Ok(items)
  }
}
