/* sys lib */
use serde::{Deserialize, Serialize};

/* models */
use crate::models::cleaner_model::{CacheFileModel, LargeFileModel, LogFileModel, TrashFileModel};

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct SystemServiceModel {
  pub name: String,
  pub description: String,
  pub status: String,
  pub isRunning: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case, dead_code)]
pub struct SystemDataModel {
  pub services: Vec<SystemServiceModel>,
  pub cacheFiles: Vec<CacheFileModel>,
  pub trashFiles: Vec<TrashFileModel>,
  pub logFiles: Vec<LogFileModel>,
  pub largeFiles: Vec<LargeFileModel>,
}
