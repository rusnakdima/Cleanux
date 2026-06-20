use crate::models::Response;
use crate::services::kernel_cleaner_service::KernelCleanerService;
use crate::services::memory_service::MemoryService;
use crate::services::power_service::PowerService;
use crate::services::process_service::ProcessService;
use crate::services::system_service::SystemService;
use crate::utils::ResponseBuilder;
static KERNEL_SERVICE: std::sync::OnceLock<KernelCleanerService> = std::sync::OnceLock::new();
fn get_kernel_service() -> &'static KernelCleanerService {
  KERNEL_SERVICE.get_or_init(|| KernelCleanerService)
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_memory_info() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  MemoryService::get_memory_info()
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_swap_info() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  MemoryService::get_swap_info()
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_process_memory() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  MemoryService::get_process_memory()
}
#[tauri::command(rename_all = "camelCase")]
pub fn optimize_memory() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  MemoryService::optimize_memory()
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_processes() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  ProcessService::get_processes()
}
#[tauri::command(rename_all = "camelCase")]
pub fn kill_process(pid: u32) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  ProcessService::kill_process(pid)
}
#[tauri::command(rename_all = "camelCase")]
pub fn kill_selected_processes(
  pids: Vec<u32>,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
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
        .data(serde_json::Value::Array(
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
        .data(serde_json::Value::Array(
          failed.into_iter().map(serde_json::Value::from).collect(),
        ))
        .build(),
    )
  }
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_current_kernel() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  Ok(
    ResponseBuilder::new()
      .success("Current kernel retrieved")
      .data(serde_json::Value::String(
        get_kernel_service().get_current_kernel(),
      ))
      .build(),
  )
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_installed_kernels() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  get_kernel_service().get_installed_kernels_response()
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_old_kernels() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  get_kernel_service().get_old_kernels_response()
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_old_kernels_size() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let size = get_kernel_service().get_old_kernels_size();
  Ok(
    ResponseBuilder::new()
      .success("Old kernels size retrieved")
      .data(serde_json::json!({ "size": size }))
      .build(),
  )
}
#[tauri::command(rename_all = "camelCase")]
pub fn remove_kernel(
  version: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  get_kernel_service().remove_kernel(&version)
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_old_initramfs() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  get_kernel_service().get_old_initramfs_response()
}
#[tauri::command(rename_all = "camelCase")]
pub fn remove_initramfs(
  version: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  get_kernel_service().remove_initramfs(&version)
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_boot_space_info() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  get_kernel_service().get_boot_space_info_response()
}
#[tauri::command(rename_all = "camelCase")]
pub fn update_grub() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  get_kernel_service().update_grub()
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_battery_info() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  PowerService::get_battery_info()
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_power_profiles() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  PowerService::get_power_profiles()
}
#[tauri::command(rename_all = "camelCase")]
pub fn set_power_profile(
  profile: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  PowerService::set_power_profile(profile)
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_thermal_info() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  PowerService::get_thermal_info()
}
#[tauri::command(rename_all = "camelCase")]
pub fn stop_service(
  service: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  SystemService.stop_service(&service)
}
#[tauri::command(rename_all = "camelCase")]
pub fn stop_selected_services(
  services: Vec<String>,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  SystemService.stop_selected_services(services)
}
#[tauri::command(rename_all = "camelCase")]
pub fn open_file(
  path: String,
  command: Option<String>,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  SystemService.open_file(&path, command)
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_all_services() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  SystemService.get_all_services()
}
#[tauri::command(rename_all = "camelCase")]
pub fn enable_service(
  service: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  SystemService.enable_service(&service)
}
#[tauri::command(rename_all = "camelCase")]
pub fn start_service(
  service: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  SystemService.start_service(&service)
}
#[tauri::command(rename_all = "camelCase")]
pub fn enable_selected_services(
  services: Vec<String>,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  SystemService.enable_selected_services(services)
}
