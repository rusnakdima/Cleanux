use serde::{Deserialize, Serialize};
use serde_json::Value;

// Local Response type — mirrors tauri_shared::Response field layout so conversions work,
// but exposes Cleanux's preferred API (success(data,msg), error(Status,msg)).
// tauri_shared::Response has data: Option<T>; this has data: T (unwrapped).
// Conversion via TryFrom/From at the bottom of this file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
  Success,
  Info,
  Warning,
  Error,
  Created,
  Updated,
  Deleted,
  ValidationError,
  NotFound,
  Unauthorized,
  Forbidden,
  Duplicate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T = Value> {
  pub status: Status,
  pub message: String,
  pub data: T,
}

impl<T> Response<T> {
  pub fn success(data: T, message: impl Into<String>) -> Self {
    Self {
      status: Status::Success,
      message: message.into(),
      data,
    }
  }
}

impl<T: Default> Response<T> {
  pub fn error(status: Status, message: impl Into<String>) -> Self {
    Self {
      status,
      message: message.into(),
      data: T::default(),
    }
  }
}

impl<T: Serialize> Response<T> {
  pub fn to_json_value(self) -> Value {
    serde_json::to_value(self).unwrap_or_else(|_| {
      serde_json::json!({
          "status": "error",
          "message": "Serialization failed",
          "data": null
      })
    })
  }
}

impl Default for Status {
  fn default() -> Self {
    Status::Success
  }
}

pub type ResponseModel = Response<Value>;

impl ResponseModel {
  pub fn created(data: Value) -> Self {
    ResponseModel {
      status: Status::Created,
      message: "Created successfully".into(),
      data,
    }
  }

  pub fn updated(data: Value) -> Self {
    ResponseModel {
      status: Status::Updated,
      message: "Updated successfully".into(),
      data,
    }
  }

  pub fn deleted(data: Value) -> Self {
    ResponseModel {
      status: Status::Deleted,
      message: "Deleted successfully".into(),
      data,
    }
  }

  pub fn validation_error(message: impl Into<String>) -> Self {
    ResponseModel {
      status: Status::ValidationError,
      message: message.into(),
      data: Value::Null,
    }
  }

  pub fn not_found(entity: &str) -> Self {
    ResponseModel {
      status: Status::NotFound,
      message: format!("{} not found", entity),
      data: Value::Null,
    }
  }

  pub fn unauthorized() -> Self {
    ResponseModel {
      status: Status::Unauthorized,
      message: "Unauthorized".into(),
      data: Value::Null,
    }
  }

  pub fn forbidden() -> Self {
    ResponseModel {
      status: Status::Forbidden,
      message: "Forbidden".into(),
      data: Value::Null,
    }
  }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ResponseModel {
  fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
    ResponseModel {
      status: Status::Error,
      message: error.to_string(),
      data: Value::String("".to_string()),
    }
  }
}

impl From<serde_json::Error> for ResponseModel {
  fn from(error: serde_json::Error) -> Self {
    ResponseModel {
      status: Status::Error,
      message: error.to_string(),
      data: Value::String("".to_string()),
    }
  }
}

impl From<String> for ResponseModel {
  fn from(error: String) -> Self {
    ResponseModel {
      status: Status::Error,
      message: error,
      data: Value::String("".to_string()),
    }
  }
}

use crate::errors::AppError;
impl From<AppError> for Response<serde_json::Value> {
  fn from(error: AppError) -> Self {
    error.into_response()
  }
}

impl From<&str> for Response<serde_json::Value> {
  fn from(error: &str) -> Self {
    Response::error(Status::Error, error.to_string())
  }
}

// Conversion to/from tauri_shared::Response
fn status_from_tauri(ts: tauri_shared::Status) -> Status {
  match ts {
    tauri_shared::Status::Success => Status::Success,
    tauri_shared::Status::Created => Status::Created,
    tauri_shared::Status::Updated => Status::Updated,
    tauri_shared::Status::Deleted => Status::Deleted,
    tauri_shared::Status::Error => Status::Error,
    tauri_shared::Status::ValidationError => Status::ValidationError,
    tauri_shared::Status::NotFound => Status::NotFound,
    tauri_shared::Status::Unauthorized => Status::Unauthorized,
    tauri_shared::Status::Forbidden => Status::Forbidden,
    tauri_shared::Status::Info => Status::Info,
    tauri_shared::Status::Warning => Status::Warning,
    tauri_shared::Status::Duplicate => Status::Duplicate,
  }
}

fn status_to_tauri(s: Status) -> tauri_shared::Status {
  match s {
    Status::Success => tauri_shared::Status::Success,
    Status::Created => tauri_shared::Status::Created,
    Status::Updated => tauri_shared::Status::Updated,
    Status::Deleted => tauri_shared::Status::Deleted,
    Status::Error => tauri_shared::Status::Error,
    Status::ValidationError => tauri_shared::Status::ValidationError,
    Status::NotFound => tauri_shared::Status::NotFound,
    Status::Unauthorized => tauri_shared::Status::Unauthorized,
    Status::Forbidden => tauri_shared::Status::Forbidden,
    Status::Info => tauri_shared::Status::Info,
    Status::Warning => tauri_shared::Status::Warning,
    Status::Duplicate => tauri_shared::Status::Duplicate,
  }
}

impl TryFrom<crate::TauriResponse<serde_json::Value>> for Response<serde_json::Value> {
  type Error = &'static str;

  fn try_from(ts: crate::TauriResponse<serde_json::Value>) -> Result<Self, Self::Error> {
    let data = ts.data.ok_or("tauri_shared Response data is None")?;
    Ok(Response {
      status: status_from_tauri(ts.status),
      message: ts.message,
      data,
    })
  }
}

impl From<Response<serde_json::Value>> for crate::TauriResponse<serde_json::Value> {
  fn from(r: Response<serde_json::Value>) -> Self {
    crate::TauriResponse {
      status: status_to_tauri(r.status),
      message: r.message,
      data: Some(r.data),
    }
  }
}
