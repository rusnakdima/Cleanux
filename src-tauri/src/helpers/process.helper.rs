/* sys lib */
use std::process::{Command, Output};

use crate::models::AppError;

pub fn stderr_message(output: &Output) -> String {
  String::from_utf8_lossy(&output.stderr).trim().to_string()
}

pub fn stderr_string(output: &Output) -> String {
  String::from_utf8_lossy(&output.stderr).into_owned()
}

pub fn stdout_string(output: &Output) -> String {
  String::from_utf8_lossy(&output.stdout).into_owned()
}

pub fn run_command(cmd: &str, args: &[&str]) -> Result<(bool, String, u64), AppError> {
  let output = Command::new(cmd)
    .args(args)
    .output()
    .map_err(|e| AppError::message(format!("Failed to run {} {}: {}", cmd, args.join(" "), e)))?;

  let stderr = stderr_string(&output);
  Ok((
    output.status.success(),
    stderr,
    output.status.code().unwrap_or(-1) as u64,
  ))
}

pub fn get_command_output(cmd: &str, args: &[&str]) -> Result<String, AppError> {
  let output = Command::new(cmd)
    .args(args)
    .output()
    .map_err(|e| AppError::message(format!("Failed to run {} {}: {}", cmd, args.join(" "), e)))?;

  if output.status.success() {
    Ok(stdout_string(&output))
  } else {
    Err(AppError::message(stderr_string(&output)))
  }
}

pub fn pkexec_rm_paths(paths: &[String]) -> Result<Output, std::io::Error> {
  let mut cmd = Command::new("pkexec");
  cmd.arg("rm").arg("-f");
  for path in paths {
    cmd.arg(path);
  }
  cmd.output()
}

pub fn run_command_raw(cmd: &str, args: &[&str]) -> Result<Output, AppError> {
  Command::new(cmd)
    .args(args)
    .output()
    .map_err(|e| AppError::message(format!("Failed to run {} {}: {}", cmd, args.join(" "), e)))
}

pub fn run_command_checked(cmd: &str, args: &[&str]) -> Result<Output, AppError> {
  let output = run_command_raw(cmd, args)?;
  if output.status.success() {
    Ok(output)
  } else {
    Err(AppError::message(stderr_string(&output)))
  }
}

pub fn run_command_ignore_error(cmd: &str, args: &[&str]) -> Option<Output> {
  Command::new(cmd).args(args).output().ok()
}

pub fn pkexec(command: &str, args: &[&str]) -> Result<Output, AppError> {
  let mut cmd = Command::new("pkexec");
  cmd.arg(command);
  for arg in args {
    cmd.arg(arg);
  }
  cmd.output().map_err(|e| {
    AppError::message(format!(
      "Failed to run pkexec {} {}: {}",
      command,
      args.join(" "),
      e
    ))
  })
}

pub fn pkexec_with_args(program: &str, args: Vec<&str>) -> Result<Output, AppError> {
  let mut cmd = Command::new("pkexec");
  cmd.arg(program);
  for arg in args {
    cmd.arg(arg);
  }
  cmd
    .output()
    .map_err(|e| AppError::message(format!("Failed to run pkexec {}: {}", program, e)))
}
