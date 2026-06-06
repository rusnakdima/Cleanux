use crate::models::{AppError, DataValue, ResponseModel, ResponseStatus};
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

#[allow(non_snake_case)]
impl ProcessService {
  pub fn getProcesses() -> Result<ResponseModel, ResponseModel> {
    Self::get_processes_inner().map_err(|e| e.into_response())
  }

  fn get_processes_inner() -> ProcessResult<ResponseModel> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let processes: Vec<ProcessItem> = sys
      .processes()
      .iter()
      .map(|(pid, process)| ProcessItem {
        pid: pid.as_u32(),
        name: process.name().to_string_lossy().to_string(),
        cpu_usage: process.cpu_usage(),
        memory_usage: process.memory(),
      })
      .collect();

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Found {} processes", processes.len()),
      data: DataValue::Array(
        processes
          .into_iter()
          .map(serde_json::to_value)
          .filter_map(Result::ok)
          .collect(),
      ),
    })
  }

  pub fn killProcess(pid: u32) -> Result<ResponseModel, ResponseModel> {
    Self::kill_process_inner(pid).map_err(|e| e.into_response())
  }

  fn kill_process_inner(pid: u32) -> ProcessResult<ResponseModel> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let sysinfo_pid = sysinfo::Pid::from_u32(pid);
    if let Some(process) = sys.process(sysinfo_pid) {
      if process.kill() {
        Ok(ResponseModel {
          status: ResponseStatus::Success,
          message: format!("Process {} killed", pid),
          data: DataValue::String(pid.to_string()),
        })
      } else {
        Err(AppError::ProcessNotFound(pid))
      }
    } else {
      Err(AppError::ProcessNotFound(pid))
    }
  }
}
