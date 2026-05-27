use crate::helpers::ResponseBuilder;
use crate::models::{DataValue, ResponseModel};
use crate::services::kernel_cleaner_service::KernelCleanerService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_current_kernel() -> Result<ResponseModel, ResponseModel> {
  Ok(
    ResponseBuilder::new()
      .success("Current kernel retrieved")
      .data(DataValue::String(
        KernelCleanerService {}.get_current_kernel(),
      ))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_installed_kernels() -> Result<ResponseModel, ResponseModel> {
  let kernels = KernelCleanerService {}.get_installed_kernels();
  let values: Vec<serde_json::Value> = kernels
    .into_iter()
    .map(serde_json::to_value)
    .collect::<Result<_, _>>()
    .map_err(|e| format!("Serialization error: {}", e))?;
  Ok(
    ResponseBuilder::new()
      .success("Installed kernels retrieved")
      .data(DataValue::Array(values))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_old_kernels() -> Result<ResponseModel, ResponseModel> {
  let kernels = KernelCleanerService {}.get_old_kernels();
  let values: Vec<serde_json::Value> = kernels
    .into_iter()
    .map(serde_json::to_value)
    .collect::<Result<_, _>>()
    .map_err(|e| format!("Serialization error: {}", e))?;
  Ok(
    ResponseBuilder::new()
      .success("Old kernels retrieved")
      .data(DataValue::Array(values))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_old_kernels_size() -> Result<ResponseModel, ResponseModel> {
  Ok(
    ResponseBuilder::new()
      .success("Old kernels size retrieved")
      .data(DataValue::Number(
        KernelCleanerService {}.get_old_kernels_size() as f64,
      ))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn remove_kernel(version: String) -> Result<ResponseModel, ResponseModel> {
  KernelCleanerService {}.remove_kernel(&version)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_old_initramfs() -> Result<ResponseModel, ResponseModel> {
  let initramfs = KernelCleanerService {}.get_old_initramfs();
  let values: Vec<serde_json::Value> = initramfs
    .into_iter()
    .map(serde_json::to_value)
    .collect::<Result<_, _>>()
    .map_err(|e| format!("Serialization error: {}", e))?;
  Ok(
    ResponseBuilder::new()
      .success("Old initramfs retrieved")
      .data(DataValue::Array(values))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn remove_initramfs(version: String) -> Result<ResponseModel, ResponseModel> {
  KernelCleanerService {}.remove_initramfs(&version)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_boot_space_info() -> Result<ResponseModel, ResponseModel> {
  let info = KernelCleanerService {}.get_boot_space_info();
  Ok(
    ResponseBuilder::new()
      .success("Boot space info retrieved")
      .data(DataValue::Object(
        serde_json::to_value(info).map_err(|e| format!("Serialization error: {}", e))?,
      ))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn update_grub() -> Result<ResponseModel, ResponseModel> {
  KernelCleanerService {}.update_grub()
}
