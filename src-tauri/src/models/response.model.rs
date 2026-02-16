/* sys lib */
use serde::{Deserialize, Serialize};

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
pub struct ResponseModel {
  pub status: ResponseStatus,
  pub message: String,
  pub data: DataValue,
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
