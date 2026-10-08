//! Settings handlers — delegate to the infrastructure `SettingsStorage`.

use crate::domain::settings::CleanuxSettings;
use crate::error::AppError;
use crate::infrastructure::settings as settings_storage;
use crate::Response as DSResponse;

pub async fn get_settings() -> Result<DSResponse<CleanuxSettings>, AppError> {
  let settings = settings_storage::load_settings();
  Ok(DSResponse::success(settings, None))
}

/// Bridge alias for get_settings
pub async fn settings_get() -> Result<DSResponse<CleanuxSettings>, AppError> {
  get_settings().await
}

pub async fn set_settings(
  settings: CleanuxSettings,
) -> Result<DSResponse<CleanuxSettings>, AppError> {
  settings_storage::save_settings(&settings).map_err(|e| AppError::Internal(e.to_string()))?;
  Ok(DSResponse::updated(settings))
}

/// Bridge alias for set_settings
pub async fn settings_save(
  settings: CleanuxSettings,
) -> Result<DSResponse<CleanuxSettings>, AppError> {
  set_settings(settings).await
}

pub async fn patch_setting(
  key: String,
  value: serde_json::Value,
) -> Result<DSResponse<CleanuxSettings>, AppError> {
  settings_storage::apply_setting_patch(&key, value);
  let settings = settings_storage::load_settings();
  Ok(DSResponse::updated(settings))
}

/// Bridge alias for patch_setting
pub async fn settings_patch(
  key: String,
  value: serde_json::Value,
) -> Result<DSResponse<CleanuxSettings>, AppError> {
  patch_setting(key, value).await
}

pub async fn add_excluded_folder(path: String) -> Result<DSResponse<CleanuxSettings>, AppError> {
  settings_storage::add_excluded(path);
  let settings = settings_storage::load_settings();
  Ok(DSResponse::updated(settings))
}

pub async fn remove_excluded_folder(index: usize) -> Result<DSResponse<CleanuxSettings>, AppError> {
  settings_storage::remove_excluded(index);
  let settings = settings_storage::load_settings();
  Ok(DSResponse::updated(settings))
}
