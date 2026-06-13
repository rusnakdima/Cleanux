/* sys lib */
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;

fn is_cache_file(name: &OsStr) -> bool {
  let name_str = name.to_string_lossy();
  name_str.contains("cache")
    || name_str.ends_with(".tmp")
    || name_str.ends_with(".bak")
    || name_str.ends_with(".log")
}

/* models */
use crate::models::{DataValue, ResponseModel, ScanSummaryModel, SystemServiceModel};

/* helpers */
use crate::helpers::{home_dir, stderr_string, stdout_string, ResponseBuilder};
use rayon::prelude::*;
use walkdir::WalkDir;

pub struct DashboardService;

#[allow(non_snake_case)]
impl DashboardService {
  pub fn getRunningServices(&self) -> Result<ResponseModel, ResponseModel> {
    #[cfg(target_os = "linux")]
    {
      let output = Command::new("systemctl")
        .args([
          "list-units",
          "--type=service",
          "--state=running",
          "--no-pager",
          "--plain",
        ])
        .output()
        .map_err(|e| format!("Failed to run systemctl: {}", e))?;

      if !output.status.success() {
        return Err(
          ResponseBuilder::new()
            .error(&stderr_string(&output))
            .build(),
        );
      }

      let stdout = stdout_string(&output);
      let mut services = Vec::new();

      for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
          let name = parts[0].to_string();
          let status = parts[3].to_string();
          let description = if parts.len() > 4 {
            parts[4..].join(" ")
          } else {
            "No description".to_string()
          };
          services.push(SystemServiceModel {
            name,
            description,
            status: status.clone(),
            isRunning: status == "running",
          });
        }
      }

      Ok(
        ResponseBuilder::new()
          .success("Running services retrieved successfully")
          .data(DataValue::Array(
            services
              .into_iter()
              .map(|s| serde_json::to_value(s).map_err(|e| format!("Serialization error: {}", e)))
              .collect::<Result<Vec<_>, _>>()?,
          ))
          .build(),
      )
    }
  }

  pub fn getCacheSummary(&self) -> Result<ResponseModel, ResponseModel> {
    let cacheDir = dirs::cache_dir().ok_or("Cache directory not found")?;

    let entries: Vec<_> = WalkDir::new(cacheDir)
      .max_depth(4)
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.file_type().is_file())
      .take(2000)
      .collect();

    let (totalSize, fileCount) = entries
      .into_par_iter()
      .filter(|e| is_cache_file(e.file_name()))
      .map(|e| {
        let metadata = fs::metadata(e.path()).ok();
        (metadata.as_ref().map(|m| m.len()).unwrap_or(0), 1)
      })
      .reduce(|| (0u64, 0usize), |(a, b), (c, d)| (a + c, b + d));

    let summary = ScanSummaryModel {
      fileCount,
      totalSize,
    };

    Ok(
      ResponseBuilder::new()
        .success("Large files summary retrieved successfully")
        .data(DataValue::Object(
          serde_json::to_value(summary).map_err(|e| format!("Serialization error: {}", e))?,
        ))
        .build(),
    )
  }

  pub fn getTrashSummary(&self) -> Result<ResponseModel, ResponseModel> {
    let home = home_dir().map_err(|_| "Home directory not found")?;
    let trashDir = home.join(".local/share/Trash/files");
    let mut totalSize = 0;
    let mut fileCount = 0;

    if let Ok(entries) = fs::read_dir(&trashDir) {
      for entry in entries.flatten() {
        if let Ok(meta) = fs::metadata(entry.path()) {
          totalSize += meta.len();
          fileCount += 1;
        }
      }
    }

    let summary = ScanSummaryModel {
      totalSize,
      fileCount,
    };

    Ok(
      ResponseBuilder::new()
        .success("Trash summary retrieved successfully")
        .data(DataValue::Object(
          serde_json::to_value(summary).map_err(|e| format!("Serialization error: {}", e))?,
        ))
        .build(),
    )
  }

  pub fn getLogSummary(&self) -> Result<ResponseModel, ResponseModel> {
    let logDir = Path::new("/var/log");

    let entries: Vec<_> = WalkDir::new(logDir)
      .max_depth(2)
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.file_type().is_file())
      .take(500)
      .collect();

    let (totalSize, fileCount) = entries
      .into_par_iter()
      .filter_map(|entry| fs::metadata(entry.path()).ok())
      .fold(
        || (0u64, 0usize),
        |mut acc, meta| {
          acc.0 += meta.len();
          acc.1 += 1;
          acc
        },
      )
      .reduce(|| (0u64, 0usize), |a, b| (a.0 + b.0, a.1 + b.1));

    let summary = ScanSummaryModel {
      totalSize,
      fileCount,
    };

    Ok(
      ResponseBuilder::new()
        .success("Log summary retrieved successfully")
        .data(DataValue::Object(
          serde_json::to_value(summary).map_err(|e| format!("Serialization error: {}", e))?,
        ))
        .build(),
    )
  }

  pub fn getLargeFilesSummary(&self) -> Result<ResponseModel, ResponseModel> {
    let home = home_dir().map_err(|_| "Home directory not found")?;
    let threshold = 100 * 1024 * 1024;

    let dirsToScan = vec![
      home.join("Downloads"),
      home.join("Documents"),
      home.join("Videos"),
      home.join("Pictures"),
      home.join("Desktop"),
    ];

    let (totalSize, fileCount) = dirsToScan
      .into_par_iter()
      .map(|dir| {
        if !dir.exists() {
          return (0u64, 0usize);
        }
        WalkDir::new(dir)
          .max_depth(3)
          .into_iter()
          .filter_map(|e| e.ok())
          .filter(|e| e.file_type().is_file())
          .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            if metadata.len() > threshold {
              Some(metadata.len())
            } else {
              None
            }
          })
          .fold((0u64, 0usize), |acc, size| (acc.0 + size, acc.1 + 1))
      })
      .reduce(|| (0u64, 0usize), |a, b| (a.0 + b.0, a.1 + b.1));

    let summary = ScanSummaryModel {
      totalSize,
      fileCount,
    };

    Ok(
      ResponseBuilder::new()
        .success("Large files summary retrieved successfully")
        .data(DataValue::Object(
          serde_json::to_value(summary).map_err(|e| format!("Serialization error: {}", e))?,
        ))
        .build(),
    )
  }
}
