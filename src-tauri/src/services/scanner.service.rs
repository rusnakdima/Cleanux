/* models */
use crate::helpers::{
  home_dir,
  validation_helper::{is_allowed_path, validate_path},
};
use crate::models::{DataValue, ResponseModel, ResponseStatus};
/* sys lib */
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use walkdir::WalkDir;

pub struct ScannerService;

#[derive(Debug, Clone, serde::Serialize)]
pub struct DuplicateGroup {
  pub hash: String,
  pub file_size: u64,
  pub wasted_space: u64,
  pub files: Vec<DuplicateFile>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DuplicateFile {
  pub name: String,
  pub path: String,
  pub size: u64,
}

#[allow(non_snake_case)]
impl ScannerService {
  pub fn scan_for_duplicates(
    path: &str,
    extension_filter: Option<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    let home = match home_dir() {
      Ok(h) => h,
      Err(_) => {
        return Err(ResponseModel {
          status: ResponseStatus::Error,
          message: "Home directory not found".to_string(),
          data: DataValue::Array(vec![]),
        });
      }
    };

    let validated_path = match validate_path(path) {
      Ok(p) => p,
      Err(e) => {
        return Err(ResponseModel {
          status: ResponseStatus::Error,
          message: e,
          data: DataValue::Array(vec![]),
        });
      }
    };

    if !is_allowed_path(&validated_path, &home) {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Path is not within allowed directories: {}", path),
        data: DataValue::Array(vec![]),
      });
    }

    let start_path = Path::new(path);

    let mut hash_map: HashMap<String, Vec<(String, u64)>> = HashMap::new();

    let extension_filter_lower = extension_filter.as_ref().map(|e| e.to_lowercase());

    for entry in WalkDir::new(start_path)
      .follow_links(false)
      .into_iter()
      .filter_map(|e| e.ok())
    {
      let file_path = entry.path();
      if !file_path.is_file() {
        continue;
      }

      if let Some(ref ext_lower) = extension_filter_lower {
        if let Some(file_ext) = file_path.extension() {
          if file_ext.to_string_lossy().to_lowercase() != *ext_lower {
            continue;
          }
        } else {
          continue;
        }
      }

      let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => continue,
      };
      let metadata = match file.metadata() {
        Ok(m) => m,
        Err(_) => continue,
      };

      let file_size = metadata.len();
      if file_size == 0 {
        continue;
      }

      let mut reader = BufReader::new(file);
      let mut hasher = Sha256::new();
      let mut buffer = [0u8; 8192];

      while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
          break;
        }
        hasher.update(&buffer[..bytes_read]);
      }

      let hash = format!("{:x}", hasher.finalize());

      hash_map
        .entry(hash)
        .or_default()
        .push((file_path.to_string_lossy().into_owned(), file_size));
    }

    let mut duplicate_groups: Vec<DuplicateGroup> = Vec::new();
    let mut total_wasted: u64 = 0;

    for (hash, files) in hash_map {
      if files.len() > 1 {
        let file_size = files[0].1;
        let wasted = file_size * (files.len() as u64 - 1);
        total_wasted += wasted;

        let duplicate_files: Vec<DuplicateFile> = files
          .into_iter()
          .map(|(path, size)| {
            let name = Path::new(&path)
              .file_name()
              .map(|n| n.to_string_lossy().into_owned())
              .unwrap_or_default();
            DuplicateFile { name, path, size }
          })
          .collect();

        duplicate_groups.push(DuplicateGroup {
          hash,
          file_size,
          wasted_space: wasted,
          files: duplicate_files,
        });
      }
    }

    duplicate_groups.sort_by(|a, b| b.wasted_space.cmp(&a.wasted_space));

    let total_duplicates: u64 = duplicate_groups.iter().map(|g| g.files.len() as u64).sum();

    let result = serde_json::json!({
        "groups": duplicate_groups,
        "totalGroups": duplicate_groups.len(),
        "totalDuplicates": total_duplicates,
        "totalWastedSpace": total_wasted
    });

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Found {} duplicate groups", duplicate_groups.len()),
      data: DataValue::Object(result),
    })
  }
}
