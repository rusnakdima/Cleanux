/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::power_service::PowerService;

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
