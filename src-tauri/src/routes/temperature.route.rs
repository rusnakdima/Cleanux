/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::temperature_service::TemperatureService;

#[tauri::command]
pub fn get_temperatures() -> Result<ResponseModel, ResponseModel> {
  TemperatureService::get_temperatures()
}

#[tauri::command]
pub fn get_cpu_temperature() -> Result<ResponseModel, ResponseModel> {
  TemperatureService::get_cpu_temperature()
}

#[tauri::command]
pub fn get_gpu_temperature() -> Result<ResponseModel, ResponseModel> {
  TemperatureService::get_gpu_temperature()
}

#[tauri::command]
pub fn clear_temperature_cache() {
  TemperatureService::clear_cache();
}
