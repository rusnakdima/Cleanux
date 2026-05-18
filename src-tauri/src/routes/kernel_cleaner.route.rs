use crate::models::ResponseModel;
use crate::services::kernel_cleaner_service::KernelCleanerService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_current_kernel() -> String {
  KernelCleanerService {}.get_current_kernel()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_installed_kernels() -> Vec<crate::services::kernel_cleaner_service::KernelInfo> {
  KernelCleanerService {}.get_installed_kernels()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_old_kernels() -> Vec<crate::services::kernel_cleaner_service::KernelInfo> {
  KernelCleanerService {}.get_old_kernels()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_old_kernels_size() -> u64 {
  KernelCleanerService {}.get_old_kernels_size()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn remove_kernel(version: String) -> Result<ResponseModel, ResponseModel> {
  KernelCleanerService {}.remove_kernel(&version)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_old_initramfs() -> Vec<crate::services::kernel_cleaner_service::InitramfsInfo> {
  KernelCleanerService {}.get_old_initramfs()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn remove_initramfs(version: String) -> Result<ResponseModel, ResponseModel> {
  KernelCleanerService {}.remove_initramfs(&version)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_boot_space_info() -> crate::services::kernel_cleaner_service::BootSpaceInfo {
  KernelCleanerService {}.get_boot_space_info()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn update_grub() -> Result<ResponseModel, ResponseModel> {
  KernelCleanerService {}.update_grub()
}
