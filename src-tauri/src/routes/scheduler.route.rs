/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::scheduler_service::{CleaningType, ScheduleConfig, SchedulerService};

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_schedule_config() -> Result<ResponseModel, ResponseModel> {
  SchedulerService::get_schedule()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn save_schedule_config(config: ScheduleConfig) -> Result<ResponseModel, ResponseModel> {
  SchedulerService::save_schedule(config)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn delete_schedule_config() -> Result<ResponseModel, ResponseModel> {
  SchedulerService::delete_schedule()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn run_cleaning_now(cleaning_type: String) -> Result<ResponseModel, ResponseModel> {
  let ct = match cleaning_type.as_str() {
    "cache" => CleaningType::Cache,
    "trash" => CleaningType::Trash,
    "logs" => CleaningType::Logs,
    "largefiles" => CleaningType::LargeFiles,
    "all" => CleaningType::All,
    _ => {
      return Err(crate::helpers::error_response(
        "Invalid cleaning type",
        crate::helpers::data_empty_string(),
      ))
    }
  };
  SchedulerService::run_cleaning(ct)
}
