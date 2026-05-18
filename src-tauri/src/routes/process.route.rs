use crate::models::{DataValue, ResponseModel, ResponseStatus};
use crate::services::process_service::ProcessService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn getProcesses() -> Result<ResponseModel, ResponseModel> {
  ProcessService::getProcesses()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn killProcess(pid: u32) -> Result<ResponseModel, ResponseModel> {
  ProcessService::killProcess(pid)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn killSelectedProcesses(pids: Vec<u32>) -> Result<ResponseModel, ResponseModel> {
  let mut killed = vec![];
  let mut failed = vec![];

  for pid in pids {
    match ProcessService::killProcess(pid) {
      Ok(_) => killed.push(pid),
      Err(_) => failed.push(pid),
    }
  }

  if failed.is_empty() {
    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Killed {} processes", killed.len()),
      data: DataValue::Array(killed.into_iter().map(serde_json::Value::from).collect()),
    })
  } else {
    Err(ResponseModel {
      status: ResponseStatus::Error,
      message: format!(
        "Killed {} processes, failed to kill {}",
        killed.len(),
        failed.len()
      ),
      data: DataValue::Array(failed.into_iter().map(serde_json::Value::from).collect()),
    })
  }
}
