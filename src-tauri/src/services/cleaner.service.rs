/* helpers */
use crate::helpers::validation_helper::{is_allowed_path, validate_path};
/* errors */
use crate::models::AppError;
/* services */
use crate::services::cache_cleaning_service::CacheCleaningService;
use crate::services::file_preview_service::FilePreviewService;
use crate::services::large_file_cleaning_service::LargeFileCleaningService;
use crate::services::log_cleaning_service::LogCleaningService;
use crate::services::trash_cleaning_service::TrashCleaningService;

pub struct CleanerService;

impl CleanerService {
  pub fn getCacheFiles(
    &self,
  ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    CacheCleaningService.getCacheFiles()
  }

  pub fn getTrashFiles(
    &self,
  ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    TrashCleaningService.getTrashFiles()
  }

  pub fn getSystemLogs(
    &self,
  ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    LogCleaningService.getSystemLogs()
  }

  pub fn getLargeFiles(
    &self,
  ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    LargeFileCleaningService.getLargeFiles()
  }

  pub fn clearSelectedCacheFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    CacheCleaningService.clearSelectedCacheFiles(paths)
  }

  pub fn clearSelectedTrashFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    TrashCleaningService.clearSelectedTrashFiles(paths)
  }

  pub fn clearSelectedLargeFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    LargeFileCleaningService.clearSelectedLargeFiles(paths)
  }

  pub fn clearSelectedLogFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    LogCleaningService.clearSelectedLogFiles(paths)
  }

  pub fn clearTrash(&self) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    TrashCleaningService.clearTrash()
  }

  pub fn clearCache(&self) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    CacheCleaningService.clearCache()
  }

  pub fn clearAllLogs(&self) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    LogCleaningService.clearAllLogs()
  }

  pub fn clearAllLargeFiles(
    &self,
  ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    LargeFileCleaningService.clearAllLargeFiles()
  }

  pub fn previewFile(
    &self,
    path: String,
  ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
    let validated = validate_path(&path).map_err(|e| AppError::InvalidPath(e))?;
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    if !is_allowed_path(&validated, &home) {
      return Err(AppError::PathOutsideAllowed(path).into_response());
    }
    FilePreviewService::preview_file(path)
  }
}
