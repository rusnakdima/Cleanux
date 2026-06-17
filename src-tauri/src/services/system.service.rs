/* sys lib */
use std::path::PathBuf;

/* models */
use crate::models::{AppError, Response};
use crate::security::allowlist::is_path_allowed;
use crate::utils::service_method_full;
use crate::utils::{pkexec, pkexec_with_args, run_command_raw};
use crate::utils::{stderr_string, stdout_string};

pub struct SystemService;

type ServiceResult<T> = Result<T, AppError>;

impl SystemService {
  pub fn stop_service(
    &self,
    service: &str,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    self
      .stop_service_inner(service)
      .map_err(|e| e.into_response())
  }

  fn stop_service_inner(&self, service: &str) -> ServiceResult<Response<serde_json::Value>> {
    let output = pkexec("systemctl", &["stop", service])?;
    if output.status.success() {
      Ok(Response::success(
        format!("Service {} stopped", service),
        serde_json::Value::String(service.to_string()),
      ))
    } else {
      Err(AppError::ServiceNotFound(format!(
        "Failed to stop service {}: {}",
        service,
        stderr_string(&output)
      )))
    }
  }

  pub fn stop_selected_services(
    &self,
    services: Vec<String>,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    self
      .stop_selected_services_inner(services)
      .map_err(|e| e.into_response())
  }

  fn stop_selected_services_inner(
    &self,
    services: Vec<String>,
  ) -> ServiceResult<Response<serde_json::Value>> {
    if services.is_empty() {
      return Ok(Response::success(
        "No services selected".to_string(),
        serde_json::Value::Array(vec![]),
      ));
    }

    let service_count = services.len();
    let mut args = vec!["stop"];
    args.extend(services.iter().map(|s| s.as_str()));
    let output = pkexec_with_args("systemctl", args)?;

    if output.status.success() {
      Ok(Response::success(
        format!("Stopped {} services successfully", service_count),
        serde_json::Value::Array(
          services
            .into_iter()
            .map(serde_json::Value::String)
            .collect(),
        ),
      ))
    } else {
      Err(AppError::ServiceNotFound(format!(
        "Failed to stop services: {}",
        stderr_string(&output).trim()
      )))
    }
  }

  pub fn open_file(
    &self,
    path: &str,
    command: Option<String>,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    self
      .open_file_inner(path, command)
      .map_err(|e| e.into_response())
  }

  fn open_file_inner(
    &self,
    path: &str,
    _command: Option<String>,
  ) -> ServiceResult<Response<serde_json::Value>> {
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
      Ok(_) => Ok(Response::success(
        format!("Started editor for file: {}", path),
        serde_json::Value::String(path.to_string()),
      )),
      Err(e) => Err(AppError::Unknown(format!("Failed to start editor: {}", e))),
    }
  }

  service_method_full!(get_all_services => get_all_services_inner);

  fn get_all_services_inner(&self) -> ServiceResult<Response<serde_json::Value>> {
    let output = run_command_raw(
      "systemctl",
      &[
        "list-units",
        "--type=service",
        "--all",
        "--no-pager",
        "--no-legend",
      ],
    )?;

    if !output.status.success() {
      return Err(AppError::Unknown(stderr_string(&output)));
    }

    let stdout = stdout_string(&output);
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
              "is_running": active == "active"
            }));
          }
        }
        None
      })
      .collect();

    Ok(Response::success(
      format!("Found {} services", services.len()),
      serde_json::Value::Array(services),
    ))
  }

  pub fn enable_service(
    &self,
    service: &str,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    self
      .enable_service_inner(service)
      .map_err(|e| e.into_response())
  }

  fn enable_service_inner(&self, service: &str) -> ServiceResult<Response<serde_json::Value>> {
    let output = pkexec("systemctl", &["enable", service])?;
    if output.status.success() {
      Ok(Response::success(
        format!("Service {} enabled", service),
        serde_json::Value::String(service.to_string()),
      ))
    } else {
      Err(AppError::ServiceNotFound(stderr_string(&output)))
    }
  }

  pub fn start_service(
    &self,
    service: &str,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    self
      .start_service_inner(service)
      .map_err(|e| e.into_response())
  }

  fn start_service_inner(&self, service: &str) -> ServiceResult<Response<serde_json::Value>> {
    let output = pkexec("systemctl", &["start", service])?;
    if output.status.success() {
      Ok(Response::success(
        format!("Service {} started", service),
        serde_json::Value::String(service.to_string()),
      ))
    } else {
      Err(AppError::ServiceNotFound(stderr_string(&output)))
    }
  }

  pub fn enable_selected_services(
    &self,
    services: Vec<String>,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    self
      .enable_selected_services_inner(services)
      .map_err(|e| e.into_response())
  }

  fn enable_selected_services_inner(
    &self,
    services: Vec<String>,
  ) -> ServiceResult<Response<serde_json::Value>> {
    if services.is_empty() {
      return Ok(Response::success(
        "No services selected".to_string(),
        serde_json::Value::Array(vec![]),
      ));
    }

    let service_count = services.len();
    let mut args = vec!["enable"];
    args.extend(services.iter().map(|s| s.as_str()));
    let output = pkexec_with_args("systemctl", args)?;

    if output.status.success() {
      Ok(Response::success(
        format!("Enabled {} services successfully", service_count),
        serde_json::Value::Array(
          services
            .into_iter()
            .map(serde_json::Value::String)
            .collect(),
        ),
      ))
    } else {
      Err(AppError::ServiceNotFound(format!(
        "Failed to enable services: {}",
        stderr_string(&output).trim()
      )))
    }
  }
}
