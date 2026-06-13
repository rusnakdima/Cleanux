/* sys lib */
use crate::helpers::{run_command_ignore_error, run_command_raw, stderr_string, stdout_string};
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
    run_command_raw(cmd, args)
  }

  fn check_docker_installed() -> bool {
    run_command_ignore_error("docker", &["--version"])
      .map(|o| o.status.success())
      .unwrap_or(false)
  }

  fn check_podman_installed() -> bool {
    run_command_ignore_error("podman", &["--version"])
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
        let stdout = stdout_string(&o);
        stdout.split_whitespace().nth(2).map(|s| s.to_string())
      })
  }

  pub fn get_docker_images_size(&self) -> u64 {
    Self::run_command("docker", &["system", "df", "--format", "{{.Size}}"])
      .ok()
      .and_then(|o| {
        let output = stdout_string(&o);
        let size_str = output.split_whitespace().next()?;
        Self::parse_size_to_bytes(size_str)
      })
      .unwrap_or(0)
  }

  pub fn get_docker_containers_count(&self) -> usize {
    Self::run_command("docker", &["ps", "-aq"])
      .ok()
      .map(|o| {
        let stdout = stdout_string(&o);
        stdout.lines().filter(|l| !l.is_empty()).count()
      })
      .unwrap_or(0)
  }

  pub fn get_docker_volumes_size(&self) -> u64 {
    Self::run_command("docker", &["system", "df", "-v", "--format", "{{.Size}}"])
      .ok()
      .map(|o| {
        let output = stdout_string(&o);
        output
          .lines()
          .rfind(|l| !l.is_empty() && !l.contains("Total"))
          .map(|l| Self::parse_size_to_bytes(l.trim()).unwrap_or(0))
          .unwrap_or(0)
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

  fn container_system_prune(container_type: &str, all: bool) -> Result<ResponseModel, AppError> {
    let mut args = vec!["system", "prune", "-f"];
    if all {
      args.push("-a");
    }

    let output = Self::run_command(container_type, &args)?;
    let stderr = stderr_string(&output);
    let stdout = stdout_string(&output);
    let message = if stdout.is_empty() {
      stderr.to_string()
    } else {
      stdout.to_string()
    };

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("{} system prune completed: {}", container_type, message),
        data: DataValue::String(message.to_string()),
      })
    } else {
      Err(AppError::Unknown(format!(
        "{} prune failed: {}",
        container_type, stderr
      )))
    }
  }

  fn container_image_prune(container_type: &str, all: bool) -> Result<ResponseModel, AppError> {
    let mut args = vec!["image", "prune", "-f"];
    if all {
      args.push("-a");
    }

    let output = Self::run_command(container_type, &args)?;
    let stderr = stderr_string(&output);
    let stdout = stdout_string(&output);
    let message = if stdout.is_empty() {
      stderr.to_string()
    } else {
      stdout.to_string()
    };

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("{} image prune completed: {}", container_type, message),
        data: DataValue::String(message.to_string()),
      })
    } else {
      Err(AppError::Unknown(format!(
        "{} image prune failed: {}",
        container_type, stderr
      )))
    }
  }

  fn container_container_prune(container_type: &str) -> Result<ResponseModel, AppError> {
    let output = Self::run_command(container_type, &["container", "prune", "-f"])?;
    let stderr = stderr_string(&output);
    let stdout = stdout_string(&output);
    let message = if stdout.is_empty() {
      stderr.to_string()
    } else {
      stdout.to_string()
    };

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("{} container prune completed: {}", container_type, message),
        data: DataValue::String(message.to_string()),
      })
    } else {
      Err(AppError::Unknown(format!(
        "{} container prune failed: {}",
        container_type, stderr
      )))
    }
  }

  fn container_volume_prune(container_type: &str) -> Result<ResponseModel, AppError> {
    let output = Self::run_command(container_type, &["volume", "prune", "-f"])?;
    let stderr = stderr_string(&output);
    let stdout = stdout_string(&output);
    let message = if stdout.is_empty() {
      stderr.to_string()
    } else {
      stdout.to_string()
    };

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("{} volume prune completed: {}", container_type, message),
        data: DataValue::String(message.to_string()),
      })
    } else {
      Err(AppError::Unknown(format!(
        "{} volume prune failed: {}",
        container_type, stderr
      )))
    }
  }

  fn container_preview_prune(container_type: &str, all: bool) -> Result<ResponseModel, AppError> {
    let mut args = vec!["system", "prune", "-f", "--dry-run"];
    if all {
      args.push("-a");
    }

    let output = Self::run_command(container_type, &args)?;
    let stderr = stderr_string(&output);
    let stdout = stdout_string(&output);
    let message = if stdout.is_empty() {
      stderr.to_string()
    } else {
      stdout.to_string()
    };

    if output.status.success() || stderr.contains("Would prune") {
      Ok(ResponseModel {
        status: ResponseStatus::Info,
        message: format!("{} dry-run preview: {}", container_type, message),
        data: DataValue::String(message.to_string()),
      })
    } else {
      Err(AppError::Unknown(format!(
        "{} preview failed: {}",
        container_type, stderr
      )))
    }
  }

  pub fn docker_system_prune(&self, all: bool) -> Result<ResponseModel, AppError> {
    Self::container_system_prune("docker", all)
  }

  pub fn docker_image_prune(&self, all: bool) -> Result<ResponseModel, AppError> {
    Self::container_image_prune("docker", all)
  }

  pub fn docker_container_prune(&self) -> Result<ResponseModel, AppError> {
    Self::container_container_prune("docker")
  }

  pub fn docker_volume_prune(&self) -> Result<ResponseModel, AppError> {
    Self::container_volume_prune("docker")
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
        let stdout = stdout_string(&o);
        stdout.split_whitespace().nth(2).map(|s| s.to_string())
      })
  }

  fn get_podman_images_size(&self) -> u64 {
    Self::run_command("podman", &["images", "--format", "{{.Size}}"])
      .ok()
      .map(|o| {
        let output = stdout_string(&o);
        output
          .lines()
          .filter_map(|l| Self::parse_size_to_bytes(l.trim()))
          .sum()
      })
      .unwrap_or(0)
  }

  fn get_podman_containers_count(&self) -> usize {
    Self::run_command("podman", &["ps", "-aq"])
      .ok()
      .map(|o| {
        let stdout = stdout_string(&o);
        stdout.lines().filter(|l| !l.is_empty()).count()
      })
      .unwrap_or(0)
  }

  pub fn podman_system_prune(&self, all: bool) -> Result<ResponseModel, AppError> {
    Self::container_system_prune("podman", all)
  }

  pub fn podman_image_prune(&self, all: bool) -> Result<ResponseModel, AppError> {
    Self::container_image_prune("podman", all)
  }

  pub fn get_container_summary(&self) -> ContainerSummary {
    ContainerSummary {
      docker: self.get_docker_info(),
      podman: self.get_podman_info(),
    }
  }

  pub fn docker_preview_prune(&self, all: bool) -> Result<ResponseModel, AppError> {
    Self::container_preview_prune("docker", all)
  }
}
