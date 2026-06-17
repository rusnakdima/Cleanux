use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use log;

use crate::models::{AppError, Response, Status};
use crate::utils::{
  get_dir_size, models_into_data_array, stderr_string, stdout_string, success_response,
};

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
      Ok(out) if out.status.success() => {
        let kernel = stdout_string(&out).trim().to_string();
        log::info!("Current kernel: {}", kernel);
        kernel
      }
      _ => {
        log::warn!("Failed to get current kernel version");
        String::new()
      }
    }
  }

  pub fn get_installed_kernels(&self) -> Vec<KernelInfo> {
    log::info!("Scanning for installed kernels");
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
              .map(|n| n.to_string_lossy().into_owned())
              .unwrap_or_default();

            let size = Self::calculate_kernel_size(&version);
            let is_current = version == current;

            kernels.push(KernelInfo {
              version,
              path: path.to_string_lossy().into_owned(),
              size,
              is_current,
            });
          }
        }
      }
    }

    kernels.sort_by(|a, b| b.version.cmp(&a.version));
    log::info!("Found {} installed kernels", kernels.len());
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

  pub fn remove_kernel(
    &self,
    version: &str,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    log::info!("Attempting to remove kernel version: {}", version);
    let current = self.get_current_kernel();

    if version == current {
      log::warn!("Cannot remove currently running kernel: {}", version);
      return Err(Response::error(
        Status::Error,
        "Cannot remove the currently running kernel!".to_string(),
      ));
    }

    let all_kernels = self.get_installed_kernels();
    if all_kernels.len() <= 1 {
      log::warn!("Cannot remove last remaining kernel");
      return Err(Response::error(
        Status::Error,
        "Cannot remove the last remaining kernel!".to_string(),
      ));
    }

    let latest_version = all_kernels
      .first()
      .map(|k| k.version.clone())
      .unwrap_or_default();

    if version == latest_version {
      log::warn!("Cannot remove latest kernel (fallback kernel): {}", version);
      return Err(Response::error(
        Status::Error,
        "Cannot remove the latest kernel (fallback kernel). Remove old kernels first.".to_string(),
      ));
    }

    let kernel_exists = all_kernels.iter().any(|k| k.version == version);
    if !kernel_exists {
      log::error!("Kernel version {} not found", version);
      return Err(Response::error(
        Status::Error,
        format!("Kernel version {} not found", version),
      ));
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
          Ok(_) => removed_items.push(file_path.to_string_lossy().into_owned()),
          Err(e) => failed_items.push(format!("{}: {}", file_path.display(), e)),
        }
      }
    }

    let modules_path = PathBuf::from("/lib/modules").join(version);
    if modules_path.exists() {
      match Self::remove_dir_all(&modules_path) {
        Ok(_) => removed_items.push(modules_path.to_string_lossy().into_owned()),
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
            Ok(_) => removed_initramfs.push(entry.path().to_string_lossy().into_owned()),
            Err(e) => failed_items.push(format!("{}: {}", entry.path().display(), e)),
          }
        }
      }
    }

    if failed_items.is_empty() {
      let _ = self.update_grub_internal();
      log::info!(
        "Successfully removed kernel {} ({} items removed)",
        version,
        removed_items.len()
      );

      Ok(Response::success(
        format!(
          "Successfully removed kernel {} ({} items removed)",
          version,
          removed_items.len()
        ),
        serde_json::json!({
            "removed": removed_items,
            "failed": failed_items
        }),
      ))
    } else {
      Ok(Response::success(
        format!(
          "Removed kernel {} with {} failures",
          version,
          failed_items.len()
        ),
        serde_json::json!({
            "removed": removed_items,
            "failed": failed_items
        }),
      ))
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
          .map(|n| n.to_string_lossy().into_owned())
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
            path: path.to_string_lossy().into_owned(),
            size,
          });
        }
      }
    }

    initramfs_files.sort_by_key(|b| std::cmp::Reverse(b.size));
    initramfs_files
  }

  pub fn remove_initramfs(
    &self,
    version: &str,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
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
        let file_name = entry.file_name().to_string_lossy().into_owned();
        for pattern in &patterns {
          if file_name.starts_with(pattern) || file_name == *pattern {
            match fs::remove_file(entry.path()) {
              Ok(_) => removed.push(entry.path().to_string_lossy().into_owned()),
              Err(e) => failed.push(format!("{}: {}", entry.path().display(), e)),
            }
            break;
          }
        }
      }
    }

    if failed.is_empty() {
      Ok(Response::success(
        format!("Removed {} initramfs files", removed.len()),
        serde_json::json!({
            "removed": removed.len(),
            "files": removed
        }),
      ))
    } else {
      Ok(Response::success(
        format!("Removed {} files, {} failed", removed.len(), failed.len()),
        serde_json::json!({
            "removed": removed.len(),
            "failed": failed.len(),
            "files": removed,
            "errors": failed
        }),
      ))
    }
  }

  pub fn get_boot_space_info(&self) -> BootSpaceInfo {
    let boot_path = get_boot_path();

    if let Ok(_meta) = fs::metadata(&boot_path) {
      let output = Command::new("df").args(["-B1", "/boot"]).output();

      if let Ok(out) = output {
        if out.status.success() {
          let stdout = stdout_string(&out);
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

  pub fn update_grub(&self) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    self.update_grub_internal()
  }

  fn update_grub_internal(
    &self,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    log::info!("Updating GRUB configuration");
    let update_cmd = if Path::new("/usr/sbin/update-grub").exists() {
      ("update-grub", vec![])
    } else if Path::new("/usr/sbin/grub-mkconfig").exists() {
      ("grub-mkconfig", vec!["-o", "/boot/grub/grub.cfg"])
    } else {
      log::error!("No GRUB update tool found");
      return Err(Response::error(
        Status::Error,
        "No GRUB update tool found".to_string(),
      ));
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
          log::info!("GRUB configuration updated successfully");
          Ok(Response::success(
            "GRUB configuration updated successfully".to_string(),
            serde_json::Value::Bool(true),
          ))
        } else {
          let stderr = stderr_string(&result);
          log::error!("GRUB update failed: {}", stderr);
          Err(Response::error(
            Status::Error,
            format!("GRUB update failed: {}", stderr),
          ))
        }
      }
      Err(e) => {
        log::error!("Failed to execute GRUB update: {}", e);
        Err(Response::error(
          Status::Error,
          format!("Failed to execute GRUB update: {}", e),
        ))
      }
    }
  }

  pub fn get_installed_kernels_response(
    &self,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    let kernels = self.get_installed_kernels();
    let data = models_into_data_array(kernels).map_err(|e| AppError::from(e).into_response())?;
    Ok(success_response("Installed kernels retrieved", data))
  }

  pub fn get_old_kernels_response(
    &self,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    let kernels = self.get_old_kernels();
    let data = models_into_data_array(kernels).map_err(|e| AppError::from(e).into_response())?;
    Ok(success_response("Old kernels retrieved", data))
  }

  pub fn get_old_initramfs_response(
    &self,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    let initramfs = self.get_old_initramfs();
    let data = models_into_data_array(initramfs).map_err(|e| AppError::from(e).into_response())?;
    Ok(success_response("Old initramfs retrieved", data))
  }

  pub fn get_boot_space_info_response(
    &self,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    let info = self.get_boot_space_info();
    Ok(success_response(
      "Boot space info retrieved",
      serde_json::to_value(info).unwrap_or(serde_json::Value::Null),
    ))
  }
}
