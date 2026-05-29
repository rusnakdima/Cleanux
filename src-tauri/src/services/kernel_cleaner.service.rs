use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::helpers::get_dir_size;
use crate::models::{DataValue, ResponseModel, ResponseStatus};

#[derive(Debug, Clone, serde::Serialize)]
pub struct KernelInfo {
  pub version: String,
  pub path: String,
  pub size: u64,
  pub is_current: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct InitramfsInfo {
  pub version: String,
  pub path: String,
  pub size: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BootSpaceInfo {
  pub total: u64,
  pub used: u64,
  pub available: u64,
  pub usage_percent: f64,
}

pub struct KernelCleanerService;

fn get_boot_path() -> PathBuf {
  PathBuf::from("/boot")
}

impl KernelCleanerService {
  pub fn get_current_kernel(&self) -> String {
    let output = Command::new("uname").arg("-r").output();

    match output {
      Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
      _ => String::new(),
    }
  }

  pub fn get_installed_kernels(&self) -> Vec<KernelInfo> {
    let current = self.get_current_kernel();
    let mut kernels: Vec<KernelInfo> = Vec::new();

    let modules_path = PathBuf::from("/lib/modules");
    if modules_path.exists() {
      if let Ok(entries) = fs::read_dir(&modules_path) {
        for entry in entries.filter_map(|e| e.ok()) {
          let path = entry.path();
          if path.is_dir() {
            let version = path
              .file_name()
              .map(|n| n.to_string_lossy().to_string())
              .unwrap_or_default();

            let size = Self::calculate_kernel_size(&version);
            let is_current = version == current;

            kernels.push(KernelInfo {
              version,
              path: path.to_string_lossy().to_string(),
              size,
              is_current,
            });
          }
        }
      }
    }

    kernels.sort_by(|a, b| b.version.cmp(&a.version));
    kernels
  }

  fn calculate_kernel_size(version: &str) -> u64 {
    let mut total_size: u64 = 0;

    let kernel_image = get_boot_path().join(format!("vmlinuz-{}", version));
    if let Ok(meta) = fs::metadata(&kernel_image) {
      total_size += meta.len();
    }

    let initrd_image = get_boot_path().join(format!("initrd.img-{}", version));
    if let Ok(meta) = fs::metadata(&initrd_image) {
      total_size += meta.len();
    }

    let dtb_image = get_boot_path().join(format!("dtb-{}", version));
    if let Ok(meta) = fs::metadata(&dtb_image) {
      total_size += meta.len();
    }

    let system_map = get_boot_path().join(format!("System.map-{}", version));
    if let Ok(meta) = fs::metadata(&system_map) {
      total_size += meta.len();
    }

    let modules_path = PathBuf::from("/lib/modules").join(version);
    if modules_path.exists() {
      total_size += get_dir_size(&modules_path);
    }

    total_size
  }

  pub fn get_old_kernels(&self) -> Vec<KernelInfo> {
    let all_kernels = self.get_installed_kernels();
    let _current = self.get_current_kernel();

    if all_kernels.len() <= 1 {
      return Vec::new();
    }

    let latest_version = all_kernels
      .first()
      .map(|k| k.version.clone())
      .unwrap_or_default();

    all_kernels
      .into_iter()
      .filter(|k| !k.is_current && k.version != latest_version)
      .collect()
  }

  pub fn get_old_kernels_size(&self) -> u64 {
    self.get_old_kernels().iter().map(|k| k.size).sum()
  }

  pub fn remove_kernel(&self, version: &str) -> Result<ResponseModel, ResponseModel> {
    let current = self.get_current_kernel();

    if version == current {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: "Cannot remove the currently running kernel!".to_string(),
        data: DataValue::Bool(false),
      });
    }

    let all_kernels = self.get_installed_kernels();
    if all_kernels.len() <= 1 {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: "Cannot remove the last remaining kernel!".to_string(),
        data: DataValue::Bool(false),
      });
    }

    let latest_version = all_kernels
      .first()
      .map(|k| k.version.clone())
      .unwrap_or_default();

    if version == latest_version {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: "Cannot remove the latest kernel (fallback kernel). Remove old kernels first."
          .to_string(),
        data: DataValue::Bool(false),
      });
    }

    let kernel_exists = all_kernels.iter().any(|k| k.version == version);
    if !kernel_exists {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Kernel version {} not found", version),
        data: DataValue::Bool(false),
      });
    }

    let mut removed_items: Vec<String> = Vec::new();
    let mut failed_items: Vec<String> = Vec::new();

    let files_to_remove = vec![
      get_boot_path().join(format!("vmlinuz-{}", version)),
      get_boot_path().join(format!("initrd.img-{}", version)),
      get_boot_path().join(format!("initrd.img-{}.old-dkms", version)),
      get_boot_path().join(format!("dtb-{}", version)),
      get_boot_path().join(format!("System.map-{}", version)),
      get_boot_path().join(format!("config-{}", version)),
    ];

    for file_path in files_to_remove {
      if file_path.exists() {
        match fs::remove_file(&file_path) {
          Ok(_) => removed_items.push(file_path.to_string_lossy().to_string()),
          Err(e) => failed_items.push(format!("{}: {}", file_path.display(), e)),
        }
      }
    }

    let modules_path = PathBuf::from("/lib/modules").join(version);
    if modules_path.exists() {
      match Self::remove_dir_all(&modules_path) {
        Ok(_) => removed_items.push(modules_path.to_string_lossy().to_string()),
        Err(e) => failed_items.push(format!("{}: {}", modules_path.display(), e)),
      }
    }

    let mut removed_initramfs: Vec<String> = Vec::new();
    let boot_files = get_boot_path();
    if let Ok(entries) = fs::read_dir(&boot_files) {
      for entry in entries.filter_map(|e| e.ok()) {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if name.starts_with(&format!("initrd.img-{}-", version))
          || name.starts_with(&format!("initramfs-{}-", version))
        {
          match fs::remove_file(entry.path()) {
            Ok(_) => removed_initramfs.push(entry.path().to_string_lossy().to_string()),
            Err(e) => failed_items.push(format!("{}: {}", entry.path().display(), e)),
          }
        }
      }
    }

    if failed_items.is_empty() {
      let _ = self.update_grub_internal();

      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!(
          "Successfully removed kernel {} ({} items removed)",
          version,
          removed_items.len()
        ),
        data: DataValue::Object(serde_json::json!({
            "removed": removed_items,
            "failed": failed_items
        })),
      })
    } else {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!(
          "Removed kernel {} with {} failures",
          version,
          failed_items.len()
        ),
        data: DataValue::Object(serde_json::json!({
            "removed": removed_items,
            "failed": failed_items
        })),
      })
    }
  }

  fn remove_dir_all(path: &Path) -> Result<(), std::io::Error> {
    if path.is_dir() {
      for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
          Self::remove_dir_all(&entry_path)?;
        } else {
          fs::remove_file(entry_path)?;
        }
      }
      fs::remove_dir(path)?;
    }
    Ok(())
  }

  pub fn get_old_initramfs(&self) -> Vec<InitramfsInfo> {
    let mut initramfs_files: Vec<InitramfsInfo> = Vec::new();
    let current = self.get_current_kernel();

    let boot_path = get_boot_path();
    if let Ok(entries) = fs::read_dir(&boot_path) {
      for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let file_name = path
          .file_name()
          .map(|n| n.to_string_lossy().to_string())
          .unwrap_or_default();

        if (file_name.starts_with("initrd.img-") || file_name.starts_with("initramfs-"))
          && !file_name.contains(&current)
          && !file_name.ends_with(".old-dkms")
        {
          let version = file_name
            .replace("initrd.img-", "")
            .replace("initramfs-", "")
            .replace(".pkg", "");

          let size = path.metadata().map(|m| m.len()).unwrap_or(0);

          initramfs_files.push(InitramfsInfo {
            version,
            path: path.to_string_lossy().to_string(),
            size,
          });
        }
      }
    }

    initramfs_files.sort_by(|a, b| b.size.cmp(&a.size));
    initramfs_files
  }

  pub fn remove_initramfs(&self, version: &str) -> Result<ResponseModel, ResponseModel> {
    let boot_path = get_boot_path();

    let patterns = vec![
      format!("initrd.img-{}", version),
      format!("initramfs-{}", version),
      format!("initrd.img-{}.pkg", version),
    ];

    let mut removed: Vec<String> = Vec::new();
    let mut failed: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(&boot_path) {
      for entry in entries.filter_map(|e| e.ok()) {
        let file_name = entry.file_name().to_string_lossy().to_string();
        for pattern in &patterns {
          if file_name.starts_with(pattern) || file_name == *pattern {
            match fs::remove_file(entry.path()) {
              Ok(_) => removed.push(entry.path().to_string_lossy().to_string()),
              Err(e) => failed.push(format!("{}: {}", entry.path().display(), e)),
            }
            break;
          }
        }
      }
    }

    if failed.is_empty() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Removed {} initramfs files", removed.len()),
        data: DataValue::Object(serde_json::json!({
            "removed": removed.len(),
            "files": removed
        })),
      })
    } else {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Removed {} files, {} failed", removed.len(), failed.len()),
        data: DataValue::Object(serde_json::json!({
            "removed": removed.len(),
            "failed": failed.len(),
            "files": removed,
            "errors": failed
        })),
      })
    }
  }

  pub fn get_boot_space_info(&self) -> BootSpaceInfo {
    let boot_path = get_boot_path();

    if let Ok(_meta) = fs::metadata(&boot_path) {
      let output = Command::new("df").args(["-B1", "/boot"]).output();

      if let Ok(out) = output {
        if out.status.success() {
          let stdout = String::from_utf8_lossy(&out.stdout);
          let lines: Vec<&str> = stdout.lines().collect();

          if lines.len() >= 2 {
            let parts: Vec<&str> = lines[1].split_whitespace().collect();
            if parts.len() >= 4 {
              let total = parts[1].parse::<u64>().unwrap_or(0);
              let used = parts[2].parse::<u64>().unwrap_or(0);
              let available = parts[3].parse::<u64>().unwrap_or(0);
              let usage_percent = if total > 0 {
                (used as f64 / total as f64) * 100.0
              } else {
                0.0
              };

              return BootSpaceInfo {
                total,
                used,
                available,
                usage_percent,
              };
            }
          }
        }
      }
    }

    BootSpaceInfo {
      total: 0,
      used: 0,
      available: 0,
      usage_percent: 0.0,
    }
  }

  pub fn update_grub(&self) -> Result<ResponseModel, ResponseModel> {
    self.update_grub_internal()
  }

  fn update_grub_internal(&self) -> Result<ResponseModel, ResponseModel> {
    let update_cmd = if Path::new("/usr/sbin/update-grub").exists() {
      ("update-grub", vec![])
    } else if Path::new("/usr/sbin/grub-mkconfig").exists() {
      ("grub-mkconfig", vec!["-o", "/boot/grub/grub.cfg"])
    } else {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: "No GRUB update tool found".to_string(),
        data: DataValue::Bool(false),
      });
    };

    let output = if update_cmd.1.is_empty() {
      Command::new("pkexec").arg(update_cmd.0).output()
    } else {
      Command::new("pkexec")
        .arg(update_cmd.0)
        .args(&update_cmd.1)
        .output()
    };

    match output {
      Ok(result) => {
        if result.status.success() {
          Ok(ResponseModel {
            status: ResponseStatus::Success,
            message: "GRUB configuration updated successfully".to_string(),
            data: DataValue::Bool(true),
          })
        } else {
          let stderr = String::from_utf8_lossy(&result.stderr);
          Err(ResponseModel {
            status: ResponseStatus::Error,
            message: format!("GRUB update failed: {}", stderr),
            data: DataValue::Bool(false),
          })
        }
      }
      Err(e) => Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Failed to execute GRUB update: {}", e),
        data: DataValue::Bool(false),
      }),
    }
  }
}
