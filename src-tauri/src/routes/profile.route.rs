/* services */
use crate::models::CleaningProfile;
use crate::services::profile_service::ProfileService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn save_profile(
  profile: CleaningProfile,
) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
  ProfileService::save_profile(profile)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn load_profile(
  name: String,
) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
  ProfileService::load_profile(&name)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn list_profiles() -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
  ProfileService::list_profiles()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn delete_profile(
  name: String,
) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
  ProfileService::delete_profile(&name)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn apply_profile(
  name: String,
) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
  ProfileService.apply_profile(&name)
}
