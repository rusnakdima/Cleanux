use serde::{Deserialize, Serialize};
use crate::models::{ResponseModel, ResponseStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: String,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: message.to_string(),
        }
    }

    pub fn err(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.to_string()),
            message: message.to_string(),
        }
    }
}

impl From<ResponseModel> for ApiResponse<serde_json::Value> {
    fn from(resp: ResponseModel) -> Self {
        match resp.status {
            ResponseStatus::Success | ResponseStatus::Info => ApiResponse {
                success: true,
                data: Some(serde_json::to_value(resp.data).unwrap_or(serde_json::Value::Null)),
                error: None,
                message: resp.message,
            },
            ResponseStatus::Warning | ResponseStatus::Error => ApiResponse {
                success: false,
                data: None,
                error: Some(resp.message.clone()),
                message: resp.message,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheFilesRequest {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClearFilesRequest {
    pub paths: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupRequest {
    pub paths: Vec<String>,
    pub archive_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreRequest {
    pub archive_path: String,
    pub destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleaningReportRequest {
    pub items_cleaned: i64,
    pub space_reclaimed: u64,
    pub duration: f64,
    pub cache_items: i64,
    pub trash_items: i64,
    pub log_items: i64,
    pub large_file_items: i64,
    pub duplicate_items: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub service: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServicesRequest {
    pub services: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeRequest {
    pub recipe_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionRequest {
    pub action_id: String,
}