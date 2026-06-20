use crate::models::{AppError, Response, Status};
use serde_json::Value;
use sysinfo::System;
pub struct ProcessService;
type ProcessResult<T> = Result<T, AppError>;
#[derive(serde::Serialize)]
pub struct ProcessItem {
  pub pid: u32,
  pub name: String,
  pub cpu_usage: f32,
  pub memory_usage: u64,
}
impl ProcessService {
  pub fn get_processes() -> Result<Response<Value>, Response<Value>> {
    Self::get_processes_inner().map_err(|e| e.into_response())
  }
  fn get_processes_inner() -> ProcessResult<Response<Value>> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let processes: Vec<ProcessItem> = sys
      .processes()
      .iter()
      .map(|(pid, process)| ProcessItem {
        pid: pid.as_u32(),
        name: process.name().to_string_lossy().into_owned(),
        cpu_usage: process.cpu_usage(),
        memory_usage: process.memory(),
      })
      .collect();
    Ok(Response {
      status: Status::Success,
      message: format!("Found {} processes", processes.len()),
      data: Value::Array(
        processes
          .into_iter()
          .map(serde_json::to_value)
          .filter_map(Result::ok)
          .collect(),
      ),
    })
  }
  pub fn kill_process(pid: u32) -> Result<Response<Value>, Response<Value>> {
    Self::kill_process_inner(pid).map_err(|e| e.into_response())
  }
  fn kill_process_inner(pid: u32) -> ProcessResult<Response<Value>> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let sysinfo_pid = sysinfo::Pid::from_u32(pid);
    if let Some(process) = sys.process(sysinfo_pid) {
      if process.kill() {
        Ok(Response {
          status: Status::Success,
          message: format!("Process {} killed", pid),
          data: Value::String(pid.to_string()),
        })
      } else {
        Err(AppError::ProcessNotFound(pid))
      }
    } else {
      Err(AppError::ProcessNotFound(pid))
    }
  }
}
