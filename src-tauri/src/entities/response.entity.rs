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

impl ResponseModel {
  pub fn new_false(message: &str) -> Self {
    ResponseModel {
      status: ResponseStatus::Error,
      message: message.to_string(),
      data: DataValue::String(String::new()),
    }
  }

  pub fn new_success(message: &str) -> Self {
    ResponseModel {
      status: ResponseStatus::Success,
      message: message.to_string(),
      data: DataValue::String(String::new()),
    }
  }

  pub fn new_success_with_data(message: &str, data: serde_json::Value) -> Self {
    ResponseModel {
      status: ResponseStatus::Success,
      message: message.to_string(),
      data: DataValue::Object(data),
    }
  }

  pub fn new_success_with_array(message: &str, data: Vec<serde_json::Value>) -> Self {
    ResponseModel {
      status: ResponseStatus::Success,
      message: message.to_string(),
      data: DataValue::Array(data),
    }
  }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ResponseModel {
  fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
    ResponseModel {
      status: ResponseStatus::Error,
      message: error.to_string(),
      data: DataValue::String(String::new()),
    }
  }
}

impl From<serde_json::Error> for ResponseModel {
  fn from(error: serde_json::Error) -> Self {
    ResponseModel {
      status: ResponseStatus::Error,
      message: error.to_string(),
      data: DataValue::String(String::new()),
    }
  }
}

impl From<String> for ResponseModel {
  fn from(error: String) -> Self {
    ResponseModel {
      status: ResponseStatus::Error,
      message: error,
      data: DataValue::String(String::new()),
    }
  }
}

impl From<&str> for ResponseModel {
  fn from(error: &str) -> Self {
    ResponseModel {
      status: ResponseStatus::Error,
      message: error.to_string(),
      data: DataValue::String(String::new()),
    }
  }
}

impl From<nosql_orm::error::OrmError> for ResponseModel {
  fn from(error: nosql_orm::error::OrmError) -> Self {
    ResponseModel {
      status: ResponseStatus::Error,
      message: error.to_string(),
      data: DataValue::String(String::new()),
    }
  }
}
