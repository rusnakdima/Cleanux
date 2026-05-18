/* sys lib */
use std::process::Command;

use crate::models::{AppError, DataValue, ResponseModel, ResponseStatus};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DockerInfo {
  pub installed: bool,
  pub images_size: u64,
  pub containers_count: usize,
  pub volumes_size: u64,
  pub version: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PodmanInfo {
  pub installed: bool,
  pub images_size: u64,
  pub containers_count: usize,
  pub version: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContainerSummary {
  pub docker: DockerInfo,
  pub podman: PodmanInfo,
}

pub struct ContainerService;

impl ContainerService {
  fn run_command(cmd: &str, args: &[&str]) -> Result<std::process::Output, AppError> {
    let output = Command::new(cmd).args(args).output()?;
    Ok(output)
  }

  fn check_docker_installed() -> bool {
    Command::new("docker")
      .arg("--version")
      .output()
      .map(|o| o.status.success())
      .unwrap_or(false)
  }

  fn check_podman_installed() -> bool {
    Command::new("podman")
      .arg("--version")
      .output()
      .map(|o| o.status.success())
      .unwrap_or(false)
  }

  pub fn get_docker_info(&self) -> DockerInfo {
    let installed = Self::check_docker_installed();
    if !installed {
      return DockerInfo {
        installed: false,
        images_size: 0,
        containers_count: 0,
        volumes_size: 0,
        version: None,
      };
    }

    let version = self.get_docker_version();
    let images_size = self.get_docker_images_size();
    let containers_count = self.get_docker_containers_count();
    let volumes_size = self.get_docker_volumes_size();

    DockerInfo {
      installed,
      images_size,
      containers_count,
      volumes_size,
      version,
    }
  }

  fn get_docker_version(&self) -> Option<String> {
    Self::run_command("docker", &["--version"])
      .ok()
      .and_then(|o| {
        let stdout = String::from_utf8_lossy(&o.stdout);
        stdout
          .trim()
          .split_whitespace()
          .nth(2)
          .map(|s| s.to_string())
      })
  }

  pub fn get_docker_images_size(&self) -> u64 {
    Self::run_command("docker", &["system", "df", "--format", "{{.Size}}"])
      .ok()
      .and_then(|o| {
        let output = String::from_utf8_lossy(&o.stdout);
        let size_str = output.trim().split_whitespace().next()?;
        Self::parse_size_to_bytes(size_str)
      })
      .unwrap_or(0)
  }

  pub fn get_docker_containers_count(&self) -> usize {
    Self::run_command("docker", &["ps", "-aq"])
      .ok()
      .and_then(|o| {
        let stdout = String::from_utf8_lossy(&o.stdout);
        let count = stdout.lines().filter(|l| !l.is_empty()).count();
        Some(count)
      })
      .unwrap_or(0)
  }

  pub fn get_docker_volumes_size(&self) -> u64 {
    Self::run_command("docker", &["system", "df", "-v", "--format", "{{.Size}}"])
      .ok()
      .and_then(|o| {
        let output = String::from_utf8_lossy(&o.stdout);
        output
          .lines()
          .filter(|l| !l.is_empty() && !l.contains("Total"))
          .last()
          .and_then(|l| Self::parse_size_to_bytes(l.trim()))
      })
      .unwrap_or(0)
  }

  fn parse_size_to_bytes(size_str: &str) -> Option<u64> {
    let size_str = size_str.trim().to_uppercase();
    let multiplier: u64 = if size_str.ends_with("GB") {
      1024 * 1024 * 1024
    } else if size_str.ends_with("MB") {
      1024 * 1024
    } else if size_str.ends_with("KB") {
      1024
    } else if size_str.ends_with("B") {
      1
    } else {
      return size_str.parse().ok();
    };

    size_str
      .trim_end_matches(|c: char| !c.is_ascii_digit())
      .parse::<f64>()
      .ok()
      .map(|n| (n * multiplier as f64) as u64)
  }

  pub fn docker_system_prune(&self, all: bool) -> Result<ResponseModel, AppError> {
    let mut args = vec!["system", "prune", "-f"];
    if all {
      args.push("-a");
    }

    let output = Self::run_command("docker", &args)?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let message = if stdout.is_empty() {
      stderr.to_string()
    } else {
      stdout.to_string()
    };

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Docker system prune completed: {}", message),
        data: DataValue::String(message.to_string()),
      })
    } else {
      Err(AppError::Unknown(format!(
        "Docker prune failed: {}",
        stderr
      )))
    }
  }

  pub fn docker_image_prune(&self, all: bool) -> Result<ResponseModel, AppError> {
    let mut args = vec!["image", "prune", "-f"];
    if all {
      args.push("-a");
    }

    let output = Self::run_command("docker", &args)?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let message = if stdout.is_empty() {
      stderr.to_string()
    } else {
      stdout.to_string()
    };

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Docker image prune completed: {}", message),
        data: DataValue::String(message.to_string()),
      })
    } else {
      Err(AppError::Unknown(format!("Image prune failed: {}", stderr)))
    }
  }

  pub fn docker_container_prune(&self) -> Result<ResponseModel, AppError> {
    let output = Self::run_command("docker", &["container", "prune", "-f"])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let message = if stdout.is_empty() {
      stderr.to_string()
    } else {
      stdout.to_string()
    };

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Docker container prune completed: {}", message),
        data: DataValue::String(message.to_string()),
      })
    } else {
      Err(AppError::Unknown(format!(
        "Container prune failed: {}",
        stderr
      )))
    }
  }

  pub fn docker_volume_prune(&self) -> Result<ResponseModel, AppError> {
    let output = Self::run_command("docker", &["volume", "prune", "-f"])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let message = if stdout.is_empty() {
      stderr.to_string()
    } else {
      stdout.to_string()
    };

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Docker volume prune completed: {}", message),
        data: DataValue::String(message.to_string()),
      })
    } else {
      Err(AppError::Unknown(format!(
        "Volume prune failed: {}",
        stderr
      )))
    }
  }

  pub fn get_podman_info(&self) -> PodmanInfo {
    let installed = Self::check_podman_installed();
    if !installed {
      return PodmanInfo {
        installed: false,
        images_size: 0,
        containers_count: 0,
        version: None,
      };
    }

    let version = self.get_podman_version();
    let images_size = self.get_podman_images_size();
    let containers_count = self.get_podman_containers_count();

    PodmanInfo {
      installed,
      images_size,
      containers_count,
      version,
    }
  }

  fn get_podman_version(&self) -> Option<String> {
    Self::run_command("podman", &["--version"])
      .ok()
      .and_then(|o| {
        let stdout = String::from_utf8_lossy(&o.stdout);
        stdout
          .trim()
          .split_whitespace()
          .nth(2)
          .map(|s| s.to_string())
      })
  }

  fn get_podman_images_size(&self) -> u64 {
    Self::run_command("podman", &["images", "--format", "{{.Size}}"])
      .ok()
      .and_then(|o| {
        let output = String::from_utf8_lossy(&o.stdout);
        let sizes: u64 = output
          .lines()
          .filter_map(|l| Self::parse_size_to_bytes(l.trim()))
          .sum();
        Some(sizes)
      })
      .unwrap_or(0)
  }

  fn get_podman_containers_count(&self) -> usize {
    Self::run_command("podman", &["ps", "-aq"])
      .ok()
      .and_then(|o| {
        let stdout = String::from_utf8_lossy(&o.stdout);
        let count = stdout.lines().filter(|l| !l.is_empty()).count();
        Some(count)
      })
      .unwrap_or(0)
  }

  pub fn podman_system_prune(&self, all: bool) -> Result<ResponseModel, AppError> {
    let mut args = vec!["system", "prune", "-f"];
    if all {
      args.push("-a");
    }

    let output = Self::run_command("podman", &args)?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let message = if stdout.is_empty() {
      stderr.to_string()
    } else {
      stdout.to_string()
    };

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Podman system prune completed: {}", message),
        data: DataValue::String(message.to_string()),
      })
    } else {
      Err(AppError::Unknown(format!(
        "Podman prune failed: {}",
        stderr
      )))
    }
  }

  pub fn podman_image_prune(&self, all: bool) -> Result<ResponseModel, AppError> {
    let mut args = vec!["image", "prune", "-f"];
    if all {
      args.push("-a");
    }

    let output = Self::run_command("podman", &args)?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let message = if stdout.is_empty() {
      stderr.to_string()
    } else {
      stdout.to_string()
    };

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Podman image prune completed: {}", message),
        data: DataValue::String(message.to_string()),
      })
    } else {
      Err(AppError::Unknown(format!(
        "Podman image prune failed: {}",
        stderr
      )))
    }
  }

  pub fn get_container_summary(&self) -> ContainerSummary {
    ContainerSummary {
      docker: self.get_docker_info(),
      podman: self.get_podman_info(),
    }
  }

  pub fn docker_preview_prune(&self, all: bool) -> Result<ResponseModel, AppError> {
    let mut args = vec!["system", "prune", "-f", "--dry-run"];
    if all {
      args.push("-a");
    }

    let output = Self::run_command("docker", &args)?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let message = if stdout.is_empty() {
      stderr.to_string()
    } else {
      stdout.to_string()
    };

    if output.status.success() || stderr.contains("Would prune") {
      Ok(ResponseModel {
        status: ResponseStatus::Info,
        message: format!("Docker dry-run preview: {}", message),
        data: DataValue::String(message.to_string()),
      })
    } else {
      Err(AppError::Unknown(format!(
        "Docker preview failed: {}",
        stderr
      )))
    }
  }
}
