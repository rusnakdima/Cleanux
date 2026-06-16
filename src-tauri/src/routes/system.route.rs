use crate::helpers::ResponseBuilder;
use crate::models::{DataValue, ResponseModel};
use crate::services::kernel_cleaner_service::KernelCleanerService;
use crate::services::memory_service::MemoryService;
use crate::services::power_service::PowerService;
use crate::services::process_service::ProcessService;
use crate::services::system_service::SystemService;

static KERNEL_SERVICE: std::sync::OnceLock<KernelCleanerService> = std::sync::OnceLock::new();

fn get_kernel_service() -> &'static KernelCleanerService {
  KERNEL_SERVICE.get_or_init(|| KernelCleanerService)
}

#[tauri::command]
pub fn get_memory_info() -> Result<ResponseModel, ResponseModel> {
  MemoryService::get_memory_info()
}

#[tauri::command]
pub fn get_swap_info() -> Result<ResponseModel, ResponseModel> {
  MemoryService::get_swap_info()
}

#[tauri::command]
pub fn get_process_memory() -> Result<ResponseModel, ResponseModel> {
  MemoryService::get_process_memory()
}

#[tauri::command]
pub fn optimize_memory() -> Result<ResponseModel, ResponseModel> {
  MemoryService::optimize_memory()
}

#[tauri::command]
pub fn get_processes() -> Result<ResponseModel, ResponseModel> {
  ProcessService::get_processes()
}

#[tauri::command]
pub fn kill_process(pid: u32) -> Result<ResponseModel, ResponseModel> {
  ProcessService::kill_process(pid)
}

#[tauri::command]
pub fn kill_selected_processes(pids: Vec<u32>) -> Result<ResponseModel, ResponseModel> {
  let mut killed = vec![];
  let mut failed = vec![];

  for pid in pids {
    match ProcessService::kill_process(pid) {
      Ok(_) => killed.push(pid),
      Err(_) => failed.push(pid),
    }
  }

  if failed.is_empty() {
    Ok(
      ResponseBuilder::new()
        .success(&format!("Killed {} processes", killed.len()))
        .data(DataValue::Array(
          killed.into_iter().map(serde_json::Value::from).collect(),
        ))
        .build(),
    )
  } else {
    Err(
      ResponseBuilder::new()
        .error(&format!(
          "Killed {} processes, failed to kill {}",
          killed.len(),
          failed.len()
        ))
        .data(DataValue::Array(
          failed.into_iter().map(serde_json::Value::from).collect(),
        ))
        .build(),
    )
  }
}

#[tauri::command]
pub fn get_current_kernel() -> Result<ResponseModel, ResponseModel> {
  Ok(
    ResponseBuilder::new()
      .success("Current kernel retrieved")
      .data(DataValue::String(get_kernel_service().get_current_kernel()))
      .build(),
  )
}

#[tauri::command]
pub fn get_installed_kernels() -> Result<ResponseModel, ResponseModel> {
  get_kernel_service().get_installed_kernels_response()
}

#[tauri::command]
pub fn get_old_kernels() -> Result<ResponseModel, ResponseModel> {
  get_kernel_service().get_old_kernels_response()
}

#[tauri::command]
pub fn get_old_kernels_size() -> Result<ResponseModel, ResponseModel> {
  Ok(
    ResponseBuilder::new()
      .success("Old kernels size retrieved")
      .data(DataValue::Number(
        get_kernel_service().get_old_kernels_size() as f64,
      ))
      .build(),
  )
}

#[tauri::command]
pub fn remove_kernel(version: String) -> Result<ResponseModel, ResponseModel> {
  get_kernel_service().remove_kernel(&version)
}

#[tauri::command]
pub fn get_old_initramfs() -> Result<ResponseModel, ResponseModel> {
  get_kernel_service().get_old_initramfs_response()
}

#[tauri::command]
pub fn remove_initramfs(version: String) -> Result<ResponseModel, ResponseModel> {
  get_kernel_service().remove_initramfs(&version)
}

#[tauri::command]
pub fn get_boot_space_info() -> Result<ResponseModel, ResponseModel> {
  get_kernel_service().get_boot_space_info_response()
}

#[tauri::command]
pub fn update_grub() -> Result<ResponseModel, ResponseModel> {
  get_kernel_service().update_grub()
}

#[tauri::command]
pub fn get_battery_info() -> Result<ResponseModel, ResponseModel> {
  PowerService::get_battery_info()
}

#[tauri::command]
pub fn get_power_profiles() -> Result<ResponseModel, ResponseModel> {
  PowerService::get_power_profiles()
}

#[tauri::command]
pub fn set_power_profile(profile: String) -> Result<ResponseModel, ResponseModel> {
  PowerService::set_power_profile(profile)
}

#[tauri::command]
pub fn get_thermal_info() -> Result<ResponseModel, ResponseModel> {
  PowerService::get_thermal_info()
}

#[tauri::command]
pub fn stop_service(service: String) -> Result<ResponseModel, ResponseModel> {
  SystemService.stop_service(&service)
}

#[tauri::command]
pub fn stop_selected_services(services: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  SystemService.stop_selected_services(services)
}

#[tauri::command]
pub fn open_file(path: String, command: Option<String>) -> Result<ResponseModel, ResponseModel> {
  SystemService.open_file(&path, command)
}

#[tauri::command]
pub fn get_all_services() -> Result<ResponseModel, ResponseModel> {
  SystemService.get_all_services()
}

#[tauri::command]
pub fn enable_service(service: String) -> Result<ResponseModel, ResponseModel> {
  SystemService.enable_service(&service)
}

#[tauri::command]
pub fn start_service(service: String) -> Result<ResponseModel, ResponseModel> {
  SystemService.start_service(&service)
}

#[tauri::command]
pub fn enable_selected_services(services: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  SystemService.enable_selected_services(services)
}
