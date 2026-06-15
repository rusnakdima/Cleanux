/* sys lib */
#[path = "response.model.rs"]
pub mod response_model;

#[path = "response/response.rs"]
pub mod response;

#[path = "system.model.rs"]
pub mod system_model;

#[path = "cleaner.model.rs"]
pub mod cleaner_model;

pub use crate::errors::AppError;
pub use cleaner_model::{
  CacheFileModel, CleaningProfile, LargeFileModel, LogFileModel, ScanSummaryModel, TrashFileModel,
};
pub use response::{Response, Status};
pub use response_model::{DataValue, PaginatedData, ResponseModel, ResponseStatus};
pub use system_model::SystemServiceModel;
