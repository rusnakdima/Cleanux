use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
  Success,
  Created,
  Updated,
  Deleted,
  Error,
  ValidationError,
  NotFound,
  Unauthorized,
  Forbidden,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Response<T = serde_json::Value> {
  pub status: Status,
  pub message: String,
  pub data: T,
}

impl<T: Default> Response<T> {
  pub fn success(data: T) -> Self {
    Self {
      status: Status::Success,
      message: String::new(),
      data,
    }
  }

  pub fn created(data: T) -> Self {
    Self {
      status: Status::Created,
      message: "Created successfully".into(),
      data,
    }
  }

  pub fn updated(data: T) -> Self {
    Self {
      status: Status::Updated,
      message: "Updated successfully".into(),
      data,
    }
  }

  pub fn deleted() -> Self {
    Self {
      status: Status::Deleted,
      message: "Deleted successfully".into(),
      data: T::default(),
    }
  }

  pub fn error(message: impl Into<String>) -> Self {
    Self {
      status: Status::Error,
      message: message.into(),
      data: T::default(),
    }
  }

  pub fn validation_error(message: impl Into<String>) -> Self {
    Self {
      status: Status::ValidationError,
      message: message.into(),
      data: T::default(),
    }
  }

  pub fn not_found(entity: &str) -> Self {
    Self {
      status: Status::NotFound,
      message: format!("{} not found", entity),
      data: T::default(),
    }
  }

  pub fn unauthorized() -> Self {
    Self {
      status: Status::Unauthorized,
      message: "Unauthorized".into(),
      data: T::default(),
    }
  }

  pub fn forbidden() -> Self {
    Self {
      status: Status::Forbidden,
      message: "Forbidden".into(),
      data: T::default(),
    }
  }
}

impl Response<serde_json::Value> {
  pub fn from_result<E: std::fmt::Display>(result: Result<(), E>) -> Self {
    match result {
      Ok(()) => Self::success(serde_json::Value::Null),
      Err(e) => Self::error(e.to_string()),
    }
  }
}
