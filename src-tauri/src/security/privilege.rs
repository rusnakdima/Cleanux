/* security/privilege.rs - Privilege operation auditing */

use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command, Output};

use crate::models::AppError;

const PRIVILEGE_LOG_PATH: &str = "/var/log/cleanux_privilege.log";

#[derive(Debug, Clone)]
pub struct PrivilegeOperation {
  pub timestamp: String,
  pub operation: String,
  pub target: String,
  pub result: String,
  pub user: String,
}

impl PrivilegeOperation {
  pub fn new(operation: &str, target: &str) -> Self {
    Self {
      timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
      operation: operation.to_string(),
      target: target.to_string(),
      result: String::new(),
      user: std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()),
    }
  }

  pub fn with_result(mut self, result: &str) -> Self {
    self.result = result.to_string();
    self
  }
}

fn log_privilege_operation(op: &PrivilegeOperation) {
  let log_entry = format!(
    "[{}] USER={} OPERATION={} TARGET={} RESULT={}\n",
    op.timestamp, op.user, op.operation, op.target, op.result
  );

  if let Ok(mut file) = OpenOptions::new()
    .create(true)
    .append(true)
    .open(PRIVILEGE_LOG_PATH)
  {
    let _ = file.write_all(log_entry.as_bytes());
  }
}

pub fn pkexec_with_timeout(program: &str, args: &[&str]) -> Result<Output, AppError> {
  let allowed_programs = ["rm", "chmod", "systemctl", "journalctl", "apt", "dpkg"];
  if !allowed_programs.contains(&program) {
    return Err(AppError::InvalidPath(format!(
      "Program '{}' is not allowed to be executed via pkexec",
      program
    )));
  }

  let mut cmd = Command::new("pkexec");
  cmd.arg(program);
  for arg in args {
    cmd.arg(arg);
  }
  cmd.arg("--disable-internal-agent");

  let output = cmd
    .output()
    .map_err(|e| AppError::Unknown(format!("Failed to execute pkexec: {}", e)))?;

  Ok(output)
}

pub fn privileged_delete(paths: &[String]) -> Result<Output, AppError> {
  use crate::security::allowlist::is_path_allowed;
  use std::path::PathBuf;

  for path in paths {
    let path_buf = PathBuf::from(path);
    if !is_path_allowed(&path_buf) {
      return Err(AppError::PathOutsideAllowed(format!(
        "Path '{}' is not in allowed directories for deletion",
        path
      )));
    }

    let op = PrivilegeOperation::new("DELETE", path);
    log_privilege_operation(&op);
  }

  let mut cmd = Command::new("pkexec");
  cmd.arg("rm").arg("-f");
  for path in paths {
    cmd.arg(path);
  }

  let output = cmd
    .output()
    .map_err(|e| AppError::Unknown(format!("Failed to run pkexec rm: {}", e)))?;

  let result = if output.status.success() {
    "SUCCESS"
  } else {
    "FAILED"
  };

  for path in paths {
    let op = PrivilegeOperation::new("DELETE", path).with_result(result);
    log_privilege_operation(&op);
  }

  Ok(output)
}

pub fn privileged_systemctl(action: &str, service: &str) -> Result<Output, AppError> {
  let op = PrivilegeOperation::new(&format!("systemctl {}", action), service);
  log_privilege_operation(&op);

  let output = Command::new("pkexec")
    .args(["systemctl", action, service])
    .output()
    .map_err(|e| AppError::Unknown(format!("Failed to run pkexec: {}", e)))?;

  let result = if output.status.success() {
    "SUCCESS"
  } else {
    "FAILED"
  };

  let op = PrivilegeOperation::new(&format!("systemctl {}", action), service).with_result(result);
  log_privilege_operation(&op);

  Ok(output)
}

pub fn privileged_chmod(mode: &str, path: &str) -> Result<Output, AppError> {
  let op = PrivilegeOperation::new(&format!("chmod {}", mode), path);
  log_privilege_operation(&op);

  let output = Command::new("pkexec")
    .args(["chmod", mode, path])
    .output()
    .map_err(|e| AppError::Unknown(format!("Failed to run pkexec chmod: {}", e)))?;

  let result = if output.status.success() {
    "SUCCESS"
  } else {
    "FAILED"
  };

  let op = PrivilegeOperation::new(&format!("chmod {}", mode), path).with_result(result);
  log_privilege_operation(&op);

  Ok(output)
}

pub fn privileged_pkexec(program: &str, args: &[String]) -> Result<Output, AppError> {
  let target = if args.is_empty() {
    program.to_string()
  } else {
    format!("{} {}", program, args.join(" "))
  };

  let op = PrivilegeOperation::new("PKEXEC", &target);
  log_privilege_operation(&op);

  let mut cmd = Command::new("pkexec");
  cmd.arg(program);
  for arg in args {
    cmd.arg(arg);
  }

  let output = cmd
    .output()
    .map_err(|e| AppError::Unknown(format!("Failed to execute pkexec: {}", e)))?;

  let result = if output.status.success() {
    "SUCCESS"
  } else {
    "FAILED"
  };

  let op = PrivilegeOperation::new("PKEXEC", &target).with_result(result);
  log_privilege_operation(&op);

  Ok(output)
}

pub fn requires_confirmation(operation: &str) -> bool {
  matches!(
    operation,
    "rm -rf /" | "rm -rf /home/*" | "dd" | "mkfs" | "fdisk" | "sfdisk" | "parted"
  )
}

pub fn log_confirmation_request(operation: &str, reason: &str) {
  let op = PrivilegeOperation::new("CONFIRMATION_REQUIRED", operation).with_result(reason);
  log_privilege_operation(&op);
}
