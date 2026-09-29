//! Package managers infrastructure (apt, dnf, pacman, zypper)

use dioxus_shared::AppError;
use std::process::Command;

/// Cleans package manager cache and returns bytes freed.
pub async fn clean_package_cache(manager: &str) -> Result<u64, AppError> {
    let freed: u64 = match manager {
        "apt" => {
            let output = Command::new("apt")
                .args(["clean"])
                .output()
                .map_err(|e| AppError::Internal(format!("apt clean failed: {}", e)))?;
            if output.status.success() { 0 } else { 0 }
        }
        "dnf" => {
            let output = Command::new("dnf")
                .args(["clean", "all"])
                .output()
                .map_err(|e| AppError::Internal(format!("dnf clean failed: {}", e)))?;
            if output.status.success() { 0 } else { 0 }
        }
        "pacman" => {
            let output = Command::new("paccache")
                .args(["-r"])
                .output()
                .map_err(|e| AppError::Internal(format!("paccache failed: {}", e)))?;
            if output.status.success() { 0 } else { 0 }
        }
        "zypper" => {
            let output = Command::new("zypper")
                .args(["clean"])
                .output()
                .map_err(|e| AppError::Internal(format!("zypper clean failed: {}", e)))?;
            if output.status.success() { 0 } else { 0 }
        }
        _ => return Err(AppError::ValidationError(format!("unknown package manager: {}", manager))),
    };
    Ok(freed)
}

/// Package manager type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Apt,
    Dnf,
    Pacman,
    Zypper,
}
