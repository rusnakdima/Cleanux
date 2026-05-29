/* sys lib */
use std::process::Command;

/* models */
use crate::models::{AppError, DataValue, ResponseModel, ResponseStatus};

pub struct SystemService;

type ServiceResult<T> = Result<T, AppError>;

#[allow(non_snake_case)]
impl SystemService {
  pub fn stopService(&self, service: &str) -> Result<ResponseModel, ResponseModel> {
    self
      .stop_service_inner(service)
      .map_err(|e| e.into_response())
  }

  fn stop_service_inner(&self, service: &str) -> ServiceResult<ResponseModel> {
    let output = Command::new("pkexec")
      .args(["systemctl", "stop", service])
      .output()?;
    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Service {} stopped", service),
        data: DataValue::String(service.to_string()),
      })
    } else {
      Err(AppError::ServiceNotFound(format!(
        "Failed to stop service {}: {}",
        service,
        String::from_utf8_lossy(&output.stderr)
      )))
    }
  }

  pub fn stopSelectedServices(
    &self,
    services: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    self
      .stop_selected_services_inner(services)
      .map_err(|e| e.into_response())
  }

  fn stop_selected_services_inner(&self, services: Vec<String>) -> ServiceResult<ResponseModel> {
    if services.is_empty() {
      return Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: "No services selected".to_string(),
        data: DataValue::Array(vec![]),
      });
    }

    let mut cmd = std::process::Command::new("pkexec");
    cmd.arg("systemctl").arg("stop");
    for service in &services {
      cmd.arg(service);
    }

    let output = cmd.output()?;

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Stopped {} services successfully", services.len()),
        data: DataValue::Array(
          services
            .into_iter()
            .map(serde_json::Value::String)
            .collect(),
        ),
      })
    } else {
      Err(AppError::ServiceNotFound(format!(
        "Failed to stop services: {}",
        String::from_utf8_lossy(&output.stderr).trim()
      )))
    }
  }

  pub fn openFile(
    &self,
    path: &str,
    command: Option<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    self
      .open_file_inner(path, command)
      .map_err(|e| e.into_response())
  }

  fn open_file_inner(&self, path: &str, _command: Option<String>) -> ServiceResult<ResponseModel> {
    use crate::security::allowlist::is_path_allowed;
    use std::path::PathBuf;

    let path_buf = PathBuf::from(path);
    if !is_path_allowed(&path_buf) {
      return Err(AppError::PathOutsideAllowed(format!(
        "Path '{}' is not in allowed directories",
        path
      )));
    }

    let mut cmd = std::process::Command::new("xdg-open");
    cmd.arg(path);

    match cmd.spawn() {
      Ok(_) => Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Started editor for file: {}", path),
        data: DataValue::String(path.to_string()),
      }),
      Err(e) => Err(AppError::Unknown(format!("Failed to start editor: {}", e))),
    }
  }

  pub fn getAllServices(&self) -> Result<ResponseModel, ResponseModel> {
    self.get_all_services_inner().map_err(|e| e.into_response())
  }

  fn get_all_services_inner(&self) -> ServiceResult<ResponseModel> {
    let output = Command::new("systemctl")
      .args([
        "list-units",
        "--type=service",
        "--all",
        "--no-pager",
        "--no-legend",
      ])
      .output()?;

    if !output.status.success() {
      return Err(AppError::Unknown(
        String::from_utf8_lossy(&output.stderr).to_string(),
      ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let services: Vec<serde_json::Value> = stdout
      .lines()
      .filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
          let name = parts[0].to_string();
          let load = parts[1].to_string();
          let active = parts[2].to_string();
          let status = parts[3].to_string();
          if name.ends_with(".service") {
            return Some(serde_json::json!({
              "name": name,
              "load": load,
              "active": active,
              "status": status,
              "isRunning": active == "active"
            }));
          }
        }
        None
      })
      .collect();

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Found {} services", services.len()),
      data: DataValue::Array(services),
    })
  }

  pub fn enableService(&self, service: &str) -> Result<ResponseModel, ResponseModel> {
    self
      .enable_service_inner(service)
      .map_err(|e| e.into_response())
  }

  fn enable_service_inner(&self, service: &str) -> ServiceResult<ResponseModel> {
    let output = Command::new("pkexec")
      .args(["systemctl", "enable", service])
      .output()?;
    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Service {} enabled", service),
        data: DataValue::String(service.to_string()),
      })
    } else {
      Err(AppError::ServiceNotFound(
        String::from_utf8_lossy(&output.stderr).to_string(),
      ))
    }
  }

  pub fn startService(&self, service: &str) -> Result<ResponseModel, ResponseModel> {
    self
      .start_service_inner(service)
      .map_err(|e| e.into_response())
  }

  fn start_service_inner(&self, service: &str) -> ServiceResult<ResponseModel> {
    let output = Command::new("pkexec")
      .args(["systemctl", "start", service])
      .output()?;
    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Service {} started", service),
        data: DataValue::String(service.to_string()),
      })
    } else {
      Err(AppError::ServiceNotFound(
        String::from_utf8_lossy(&output.stderr).to_string(),
      ))
    }
  }

  pub fn enableSelectedServices(
    &self,
    services: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    self
      .enable_selected_services_inner(services)
      .map_err(|e| e.into_response())
  }

  fn enable_selected_services_inner(&self, services: Vec<String>) -> ServiceResult<ResponseModel> {
    if services.is_empty() {
      return Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: "No services selected".to_string(),
        data: DataValue::Array(vec![]),
      });
    }

    let mut cmd = std::process::Command::new("pkexec");
    cmd.arg("systemctl").arg("enable");
    for service in &services {
      cmd.arg(service);
    }

    let output = cmd.output()?;

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Enabled {} services successfully", services.len()),
        data: DataValue::Array(
          services
            .into_iter()
            .map(serde_json::Value::String)
            .collect(),
        ),
      })
    } else {
      Err(AppError::ServiceNotFound(format!(
        "Failed to enable services: {}",
        String::from_utf8_lossy(&output.stderr).trim()
      )))
    }
  }
}
