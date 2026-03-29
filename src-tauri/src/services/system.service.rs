/* sys lib */
use std::process::Command;

/* models */
use crate::models::{DataValue, ResponseModel, ResponseStatus};

pub struct SystemService;

#[allow(non_snake_case)]
impl SystemService {
  pub fn stopService(&self, service: &str) -> Result<ResponseModel, ResponseModel> {
    let output = Command::new("pkexec")
      .args(["systemctl", "stop", service])
      .output()
      .map_err(|e| format!("Failed to run command: {}", e))?;
    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Service {} stopped", service),
        data: DataValue::String(service.to_string()),
      })
    } else {
      Err(ResponseModel {
        status: ResponseStatus::Error,
        message: String::from_utf8_lossy(&output.stderr).to_string(),
        data: DataValue::String("".to_string()),
      })
    }
  }

  pub fn stopSelectedServices(
    &self,
    services: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
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

    let output = cmd.output().map_err(|e| ResponseModel {
      status: ResponseStatus::Error,
      message: format!("Failed to run pkexec: {}", e),
      data: DataValue::Array(vec![]),
    })?;

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
      Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!(
          "Failed to stop services: {}",
          String::from_utf8_lossy(&output.stderr).trim()
        ),
        data: DataValue::Array(vec![]),
      })
    }
  }

  pub fn openFile(
    &self,
    path: &str,
    command: Option<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    let mut cmd = if let Some(custom_cmd) = command {
      let mut c = std::process::Command::new(custom_cmd);
      c.arg(path);
      c
    } else {
      let mut c = std::process::Command::new("xdg-open");
      c.arg(path);
      c
    };

    match cmd.spawn() {
      Ok(_) => Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Started editor for file: {}", path),
        data: DataValue::String(path.to_string()),
      }),
      Err(e) => Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Failed to start editor: {}", e),
        data: DataValue::String("".to_string()),
      }),
    }
  }

  #[allow(dead_code)]
  pub fn getDisabledServices(&self) -> Result<ResponseModel, ResponseModel> {
    let output = Command::new("systemctl")
      .args([
        "list-unit-files",
        "--state=enabled",
        "--no-pager",
        "--no-legend",
      ])
      .output()
      .map_err(|e| ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Failed to list services: {}", e),
        data: DataValue::Array(vec![]),
      })?;

    if !output.status.success() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: String::from_utf8_lossy(&output.stderr).to_string(),
        data: DataValue::Array(vec![]),
      });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let services: Vec<serde_json::Value> = stdout
      .lines()
      .filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
          return None;
        }
        let name = parts[0].to_string();
        if name.ends_with(".service") {
          Some(serde_json::json!({
            "name": name,
            "status": "enabled"
          }))
        } else {
          None
        }
      })
      .collect();

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Found {} enabled services", services.len()),
      data: DataValue::Array(services),
    })
  }

  pub fn getAllServices(&self) -> Result<ResponseModel, ResponseModel> {
    let output = Command::new("systemctl")
      .args([
        "list-units",
        "--type=service",
        "--all",
        "--no-pager",
        "--no-legend",
      ])
      .output()
      .map_err(|e| ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Failed to list services: {}", e),
        data: DataValue::Array(vec![]),
      })?;

    if !output.status.success() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: String::from_utf8_lossy(&output.stderr).to_string(),
        data: DataValue::Array(vec![]),
      });
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
    let output = Command::new("pkexec")
      .args(["systemctl", "enable", service])
      .output()
      .map_err(|e| ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Failed to run command: {}", e),
        data: DataValue::String("".to_string()),
      })?;

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Service {} enabled", service),
        data: DataValue::String(service.to_string()),
      })
    } else {
      Err(ResponseModel {
        status: ResponseStatus::Error,
        message: String::from_utf8_lossy(&output.stderr).to_string(),
        data: DataValue::String("".to_string()),
      })
    }
  }

  pub fn startService(&self, service: &str) -> Result<ResponseModel, ResponseModel> {
    let output = Command::new("pkexec")
      .args(["systemctl", "start", service])
      .output()
      .map_err(|e| ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Failed to run command: {}", e),
        data: DataValue::String("".to_string()),
      })?;

    if output.status.success() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Service {} started", service),
        data: DataValue::String(service.to_string()),
      })
    } else {
      Err(ResponseModel {
        status: ResponseStatus::Error,
        message: String::from_utf8_lossy(&output.stderr).to_string(),
        data: DataValue::String("".to_string()),
      })
    }
  }

  pub fn enableSelectedServices(
    &self,
    services: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
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

    let output = cmd.output().map_err(|e| ResponseModel {
      status: ResponseStatus::Error,
      message: format!("Failed to run pkexec: {}", e),
      data: DataValue::Array(vec![]),
    })?;

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
      Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!(
          "Failed to enable services: {}",
          String::from_utf8_lossy(&output.stderr).trim()
        ),
        data: DataValue::Array(vec![]),
      })
    }
  }
}
