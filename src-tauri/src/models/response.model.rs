/* sys lib */
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
  Success,
  Info,
  Warning,
  Error,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum DataValue {
  String(String),
  Number(f64),
  Bool(bool),
  Array(Vec<serde_json::Value>),
  Object(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginatedData<T> {
  pub data: Vec<T>,
  pub has_more: bool,
  pub total: usize,
}

impl<T: Serialize> PaginatedData<T> {
  pub fn new(data: Vec<T>, has_more: bool, total: usize) -> Self {
    Self {
      data,
      has_more,
      total,
    }
  }
}

impl From<serde_json::Value> for DataValue {
  fn from(v: serde_json::Value) -> Self {
    DataValue::Object(v)
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseModel {
  pub status: ResponseStatus,
  pub message: String,
  pub data: DataValue,
}

impl std::fmt::Display for ResponseModel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ResponseModel {
  fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
    ResponseModel {
      status: ResponseStatus::Error,
      message: error.to_string(),
      data: DataValue::String("".to_string()),
    }
  }
}

impl From<String> for ResponseModel {
  fn from(error: String) -> Self {
    ResponseModel {
      status: ResponseStatus::Error,
      message: error,
      data: DataValue::String("".to_string()),
    }
  }
}

impl From<&str> for ResponseModel {
  fn from(error: &str) -> Self {
    ResponseModel {
      status: ResponseStatus::Error,
      message: error.to_string(),
      data: DataValue::String("".to_string()),
    }
  }
}

impl From<AppError> for ResponseModel {
  fn from(error: AppError) -> Self {
    error.into_response()
  }
}
