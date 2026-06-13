use crate::models::{DataValue, ResponseModel, ResponseStatus};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, serde::Serialize)]
pub struct EmptyDirectory {
  pub path: String,
  pub depth: u32,
  pub parent: Option<String>,
}

pub struct DirectoryService;

#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectoryNode {
  pub name: String,
  pub path: String,
  pub size: u64,
  #[serde(default)]
  pub children: Vec<DirectoryNode>,
}

#[allow(non_snake_case)]
impl DirectoryService {
  pub fn scan_directory(path: &str, max_depth: u32) -> Result<ResponseModel, ResponseModel> {
    let start_path = Path::new(path);
    if !start_path.exists() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Path does not exist: {}", path),
        data: DataValue::Array(vec![]),
      });
    }

    let tree = Self::build_directory_tree(start_path, max_depth)?;
    let size = tree.size;

    let result = serde_json::json!({
        "tree": tree,
        "totalSize": size
    });

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Directory scanned successfully".to_string(),
      data: DataValue::Object(result),
    })
  }

  pub fn get_directory_size(path: &str) -> Result<ResponseModel, ResponseModel> {
    let dir_path = Path::new(path);
    if !dir_path.exists() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Path does not exist: {}", path),
        data: DataValue::Object(serde_json::Value::Null),
      });
    }

    let mut total_size: u64 = 0;

    for entry in WalkDir::new(dir_path)
      .follow_links(false)
      .into_iter()
      .filter_map(|e| e.ok())
    {
      if entry.path().is_file() {
        if let Ok(metadata) = fs::metadata(entry.path()) {
          total_size += metadata.len();
        }
      }
    }

    let result = serde_json::json!({
        "path": path,
        "size": total_size
    });

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Size: {} bytes", total_size),
      data: DataValue::Object(result),
    })
  }

  fn build_directory_tree(path: &Path, max_depth: u32) -> Result<DirectoryNode, ResponseModel> {
    let name = path
      .file_name()
      .map(|n| n.to_string_lossy().into_owned())
      .unwrap_or_else(|| path.to_string_lossy().into_owned());

    let mut node = DirectoryNode {
      name,
      path: path.to_string_lossy().into_owned(),
      size: 0,
      children: vec![],
    };

    if let Ok(entries) = fs::read_dir(path) {
      let mut children: Vec<DirectoryNode> = vec![];
      let mut dir_size: u64 = 0;

      for entry in entries.filter_map(|e| e.ok()) {
        let entry_path = entry.path();

        if entry_path.is_dir() {
          if let Ok(subtree) = Self::build_directory_tree(&entry_path, max_depth.saturating_sub(1))
          {
            dir_size += subtree.size;
            children.push(subtree);
          }
        } else if entry_path.is_file() {
          if let Ok(metadata) = fs::metadata(&entry_path) {
            let file_size = metadata.len();
            dir_size += file_size;

            let file_name = entry_path
              .file_name()
              .map(|n| n.to_string_lossy().into_owned())
              .unwrap_or_default();

            children.push(DirectoryNode {
              name: file_name,
              path: entry_path.to_string_lossy().into_owned(),
              size: file_size,
              children: vec![],
            });
          }
        }
      }

      children.sort_by_key(|b| std::cmp::Reverse(b.size));
      node.children = children;
      node.size = dir_size;
    }

    Ok(node)
  }

  pub fn find_empty_directories(path: &str) -> Result<ResponseModel, ResponseModel> {
    let start_path = Path::new(path);
    if !start_path.exists() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Path does not exist: {}", path),
        data: DataValue::Array(vec![]),
      });
    }

    let mut empty_dirs: Vec<EmptyDirectory> = Vec::new();
    Self::collect_empty_directories(start_path, 0, &mut empty_dirs);

    let result: Vec<serde_json::Value> = empty_dirs
      .iter()
      .map(|dir| {
        serde_json::json!({
            "path": dir.path,
            "depth": dir.depth,
            "parent": dir.parent
        })
      })
      .collect();

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Found {} empty directories", result.len()),
      data: DataValue::Array(result),
    })
  }

  fn collect_empty_directories(path: &Path, depth: u32, empty_dirs: &mut Vec<EmptyDirectory>) {
    let parent = path.parent().map(|p| p.to_string_lossy().into_owned());

    if let Ok(entries) = fs::read_dir(path) {
      let mut has_content = false;

      for entry in entries.filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        if entry_path.is_dir() {
          Self::collect_empty_directories(&entry_path, depth + 1, empty_dirs);
        } else {
          has_content = true;
        }
      }

      if !has_content {
        empty_dirs.push(EmptyDirectory {
          path: path.to_string_lossy().into_owned(),
          depth,
          parent,
        });
      }
    }
  }

  pub fn find_nested_empty_directories(path: &str) -> Result<ResponseModel, ResponseModel> {
    let start_path = Path::new(path);
    if !start_path.exists() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Path does not exist: {}", path),
        data: DataValue::Array(vec![]),
      });
    }

    let mut nested_empty_dirs: Vec<EmptyDirectory> = Vec::new();
    Self::collect_nested_empty_directories(start_path, 0, &mut nested_empty_dirs);

    let result: Vec<serde_json::Value> = nested_empty_dirs
      .iter()
      .map(|dir| {
        serde_json::json!({
            "path": dir.path,
            "depth": dir.depth,
            "parent": dir.parent
        })
      })
      .collect();

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Found {} nested empty directories", result.len()),
      data: DataValue::Array(result),
    })
  }

  fn collect_nested_empty_directories(
    path: &Path,
    depth: u32,
    nested_empty_dirs: &mut Vec<EmptyDirectory>,
  ) {
    let parent = path.parent().map(|p| p.to_string_lossy().into_owned());

    if let Ok(entries) = fs::read_dir(path) {
      let entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
      let mut all_subdirs_empty = true;

      for entry in &entries {
        let entry_path = entry.path();
        if entry_path.is_dir() {
          Self::collect_nested_empty_directories(&entry_path, depth + 1, nested_empty_dirs);
          if !Self::is_directory_empty(&entry_path) {
            all_subdirs_empty = false;
          }
        } else {
          all_subdirs_empty = false;
        }
      }

      if all_subdirs_empty && !entries.is_empty() {
        nested_empty_dirs.push(EmptyDirectory {
          path: path.to_string_lossy().into_owned(),
          depth,
          parent,
        });
      }
    }
  }

  fn is_directory_empty(path: &Path) -> bool {
    if let Ok(entries) = fs::read_dir(path) {
      let count = entries.count();
      count == 0
    } else {
      true
    }
  }

  pub fn remove_empty_directory(path: &str) -> Result<ResponseModel, ResponseModel> {
    let dir_path = Path::new(path);
    if !dir_path.exists() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Path does not exist: {}", path),
        data: DataValue::Object(serde_json::Value::Null),
      });
    }

    if !dir_path.is_dir() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Path is not a directory: {}", path),
        data: DataValue::Object(serde_json::Value::Null),
      });
    }

    if let Ok(entries) = fs::read_dir(dir_path) {
      if entries.count() > 0 {
        return Err(ResponseModel {
          status: ResponseStatus::Error,
          message: format!("Directory is not empty: {}", path),
          data: DataValue::Object(serde_json::Value::Null),
        });
      }
    }

    match fs::remove_dir(path) {
      Ok(_) => Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Removed directory: {}", path),
        data: DataValue::Object(serde_json::Value::Null),
      }),
      Err(e) => Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Failed to remove directory: {} - {}", path, e),
        data: DataValue::Object(serde_json::Value::Null),
      }),
    }
  }

  pub fn remove_empty_directories(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
    let mut removed_count = 0;
    let mut failed: Vec<String> = Vec::new();

    for path in &paths {
      let dir_path = Path::new(path);
      if dir_path.exists() && dir_path.is_dir() {
        if let Ok(entries) = fs::read_dir(dir_path) {
          if entries.count() == 0 {
            match fs::remove_dir(path) {
              Ok(_) => removed_count += 1,
              Err(e) => failed.push(format!("{}: {}", path, e)),
            }
          } else {
            failed.push(format!("{}: not empty", path));
          }
        }
      } else {
        failed.push(format!("{}: does not exist", path));
      }
    }

    let result = serde_json::json!({
        "removed": removed_count,
        "failed": failed,
        "total": paths.len()
    });

    if failed.is_empty() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Successfully removed {} directories", removed_count),
        data: DataValue::Object(result),
      })
    } else {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!(
          "Removed {} directories, {} failed",
          removed_count,
          failed.len()
        ),
        data: DataValue::Object(result),
      })
    }
  }
}
