//! Docker/Podman container management service
//!
//! Provides container info and cleanup functionality via CLI.

use crate::error::AppError;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

/// Docker container information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainerInfo {
  pub installed: bool,
  pub version: Option<String>,
  pub images_size: u64,
  pub containers_count: usize,
  pub volumes_size: u64,
}

/// Podman container information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodmanContainerInfo {
  pub installed: bool,
  pub version: Option<String>,
  pub images_size: u64,
  pub containers_count: usize,
}

/// Container summary combining Docker and Podman info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerSummary {
  pub docker: Option<DockerContainerInfo>,
  pub podman: Option<PodmanContainerInfo>,
}

/// Result of a prune operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PruneResult {
  pub bytes_freed: u64,
  pub message: String,
}

pub struct DockerService;

impl DockerService {
  pub fn new() -> Self {
    Self
  }

  /// Parse a size string (e.g. "1.5GB", "500MB", "1.2kB") to bytes.
  fn parse_size_to_bytes(s: &str) -> u64 {
    let s = s.trim();
    let Some((num_str, unit)) = s.split_once(|c: char| c.is_alphabetic()) else {
      return s.parse().unwrap_or(0);
    };
    let num: f64 = num_str.trim().parse().unwrap_or(0.0);
    let unit = unit.trim().to_uppercase();
    let factor: u64 = match unit.as_str() {
      "B" | "" => 1,
      "KB" => 1024,
      "MB" => 1024 * 1024,
      "GB" => 1024 * 1024 * 1024,
      "TB" => 1024 * 1024 * 1024 * 1024,
      _ => 1,
    };
    (num * factor as f64) as u64
  }

  /// Format bytes to human-readable string.
  fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
      format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
      format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
      format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
      format!("{} B", bytes)
    }
  }

  /// Get Docker info via CLI. Returns None if Docker is not installed.
  pub async fn get_docker_info(&self) -> Option<DockerContainerInfo> {
    // Check Docker availability via --version
    let version_check = Command::new("docker")
      .arg("--version")
      .output()
      .await
      .ok()?;

    if !version_check.status.success() {
      return None;
    }

    let version = String::from_utf8(version_check.stdout)
      .ok()?
      .split_whitespace()
      .nth(2)
      .map(|s| s.to_string());

    // docker system df --format {{.Size}} → first token is total images size
    let images_size = Command::new("docker")
      .arg("system")
      .arg("df")
      .arg("--format")
      .arg("{{.Size}}")
      .output()
      .await
      .ok()
      .and_then(|o| String::from_utf8(o.stdout).ok())
      .map(|s| Self::parse_size_to_bytes(s.split_whitespace().next().unwrap_or("0")))
      .unwrap_or(0);

    // docker ps -aq → count non-empty lines = running containers
    let containers_count = Command::new("docker")
      .arg("ps")
      .arg("-aq")
      .output()
      .await
      .ok()
      .map(|o| {
        String::from_utf8(o.stdout)
          .unwrap_or_default()
          .lines()
          .filter(|l| !l.trim().is_empty())
          .count()
      })
      .unwrap_or(0);

    // docker system df -v --format {{.Size}} → last non-empty, non-"Total" line = volumes size
    let volumes_size = Command::new("docker")
      .arg("system")
      .arg("df")
      .arg("-v")
      .arg("--format")
      .arg("{{.Size}}")
      .output()
      .await
      .ok()
      .and_then(|o| String::from_utf8(o.stdout).ok())
      .and_then(|s| {
        s.lines()
          .filter(|l| !l.trim().is_empty() && !l.contains("Total"))
          .rfind(|_| true)
          .map(|l| Self::parse_size_to_bytes(l.trim()))
      })
      .unwrap_or(0);

    Some(DockerContainerInfo {
      installed: true,
      version,
      images_size,
      containers_count,
      volumes_size,
    })
  }

  /// Get Podman info via CLI. Returns None if Podman is not installed.
  pub async fn get_podman_info(&self) -> Option<PodmanContainerInfo> {
    let version_check = Command::new("podman")
      .arg("--version")
      .output()
      .await
      .ok()?;

    if !version_check.status.success() {
      return None;
    }

    let version = String::from_utf8(version_check.stdout)
      .ok()?
      .split_whitespace()
      .nth(2)
      .map(|s| s.to_string());

    // Sum sizes from podman images --format {{.Size}}
    let images_size = Command::new("podman")
      .arg("images")
      .arg("--format")
      .arg("{{.Size}}")
      .output()
      .await
      .ok()
      .and_then(|o| String::from_utf8(o.stdout).ok())
      .map(|s| {
        s.lines()
          .filter(|l| !l.trim().is_empty())
          .map(|l| Self::parse_size_to_bytes(l.trim()))
          .sum()
      })
      .unwrap_or(0);

    let containers_count = Command::new("podman")
      .arg("ps")
      .arg("-aq")
      .output()
      .await
      .ok()
      .map(|o| {
        String::from_utf8(o.stdout)
          .unwrap_or_default()
          .lines()
          .filter(|l| !l.trim().is_empty())
          .count()
      })
      .unwrap_or(0);

    Some(PodmanContainerInfo {
      installed: true,
      version,
      images_size,
      containers_count,
    })
  }

  /// Get container summary for both Docker and Podman.
  pub async fn get_container_summary(&self) -> Result<ContainerSummary, AppError> {
    let docker_info = self.get_docker_info().await;
    let podman_info = self.get_podman_info().await;

    Ok(ContainerSummary {
      docker: docker_info,
      podman: podman_info,
    })
  }

  /// Docker system prune — runs `docker system prune --all` via CLI.
  pub async fn docker_system_prune(&self) -> Result<PruneResult, AppError> {
    // Verify docker is available
    let check = Command::new("docker")
      .arg("--version")
      .output()
      .await
      .map_err(|e| AppError::System(format!("docker not available: {}", e)))?;

    if !check.status.success() {
      return Err(AppError::System("docker command failed".to_string()));
    }

    let output = Command::new("docker")
      .arg("system")
      .arg("prune")
      .arg("-a")
      .arg("-f")
      .output()
      .await
      .map_err(|e| AppError::System(format!("docker system prune failed: {}", e)))?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
      return Err(AppError::System(format!(
        "docker system prune failed: {}",
        stderr
      )));
    }

    // Parse "Total reclaimed space: X.Y GB/MB/KB/B" from stdout
    let freed = stdout
      .lines()
      .filter(|l| l.contains("reclaimed"))
      .rfind(|_| true)
      .and_then(|l| {
        l.split(':')
          .nth(1)
          .map(|s| Self::parse_size_to_bytes(s.trim()))
      })
      .unwrap_or(0);

    tracing::info!("Docker system prune completed: {} bytes freed", freed);

    Ok(PruneResult {
      bytes_freed: freed,
      message: format!("Docker system pruned, {} freed", Self::format_bytes(freed)),
    })
  }

  /// Podman system prune — runs `podman system prune` via CLI.
  pub async fn podman_system_prune(&self) -> Result<PruneResult, AppError> {
    // Verify podman is available
    let check = Command::new("podman")
      .arg("--version")
      .output()
      .await
      .map_err(|e| AppError::System(format!("podman not available: {}", e)))?;

    if !check.status.success() {
      return Err(AppError::System("podman command failed".to_string()));
    }

    let output = Command::new("podman")
      .arg("system")
      .arg("prune")
      .arg("-a")
      .arg("-f")
      .output()
      .await
      .map_err(|e| AppError::System(format!("podman system prune failed: {}", e)))?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
      return Err(AppError::System(format!(
        "podman system prune failed: {}",
        stderr
      )));
    }

    // Parse bytes freed from output
    let freed = stdout
      .lines()
      .filter(|l| l.contains("reclaimed") || l.contains("freed"))
      .rfind(|_| true)
      .and_then(|l| {
        l.split_whitespace()
          .rev()
          .find(|w| w.parse::<f64>().is_ok())
          .map(|s| Self::parse_size_to_bytes(s))
      })
      .unwrap_or(0);

    tracing::info!("Podman system prune completed: {} bytes freed", freed);

    Ok(PruneResult {
      bytes_freed: freed,
      message: format!("Podman system pruned, {} freed", Self::format_bytes(freed)),
    })
  }
}

impl Default for DockerService {
  fn default() -> Self {
    Self::new()
  }
}
