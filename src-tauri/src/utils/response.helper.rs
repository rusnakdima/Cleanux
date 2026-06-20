/* models */
use crate::models::{Response, Status};
/* sys lib */
use serde::Serialize;
pub struct ResponseBuilder {
  status: Option<Status>,
  message: Option<String>,
  data: Option<serde_json::Value>,
}
impl ResponseBuilder {
  pub fn new() -> Self {
    ResponseBuilder {
      status: None,
      message: None,
      data: None,
    }
  }
  pub fn success(mut self, msg: &str) -> Self {
    self.status = Some(Status::Success);
    self.message = Some(msg.to_string());
    self
  }
  pub fn info(mut self, msg: &str) -> Self {
    self.status = Some(Status::Info);
    self.message = Some(msg.to_string());
    self
  }
  pub fn error(mut self, msg: &str) -> Self {
    self.status = Some(Status::Error);
    self.message = Some(msg.to_string());
    self
  }
  pub fn data(mut self, value: serde_json::Value) -> Self {
    self.data = Some(value);
    self
  }
  pub fn build(self) -> Response<serde_json::Value> {
    let status = self.status.unwrap_or(Status::Info);
    let message = self.message.unwrap_or_default();
    let data = self
      .data
      .unwrap_or(serde_json::Value::String(String::new()));
    if status == Status::Success {
      Response::success(message, data)
    } else {
      Response::error(status, message)
    }
  }
}
impl Default for ResponseBuilder {
  fn default() -> Self {
    Self::new()
  }
}
pub fn success_response(
  message: impl Into<String>,
  data: serde_json::Value,
) -> Response<serde_json::Value> {
  Response::success(message, data)
}
pub fn info_response(
  message: impl Into<String>,
  data: serde_json::Value,
) -> Response<serde_json::Value> {
  Response::success(message, data)
}
pub fn error_response(
  message: impl Into<String>,
  _data: serde_json::Value,
) -> Response<serde_json::Value> {
  Response::error(Status::Error, message)
}
pub fn data_empty_string() -> serde_json::Value {
  serde_json::Value::String(String::new())
}
pub fn data_string(value: impl Into<String>) -> serde_json::Value {
  serde_json::Value::String(value.into())
}
pub fn array_response<T: Serialize>(
  message: impl Into<String>,
  items: Vec<T>,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let data: Vec<serde_json::Value> = items
    .into_iter()
    .map(|item| serde_json::to_value(item))
    .collect::<Result<_, _>>()
    .map_err(|e| Response::error(Status::Error, format!("Serialization error: {}", e)))?;
  Ok(Response::success(message, serde_json::Value::Array(data)))
}
/// Serialize models to JSON values; propagates first serialization failure instead of swallowing it.
pub fn models_into_data_array<T: Serialize>(
  items: Vec<T>,
) -> Result<serde_json::Value, serde_json::Error> {
  let values: Vec<serde_json::Value> = items
    .into_iter()
    .map(|item| serde_json::to_value(item))
    .collect::<Result<_, _>>()?;
  Ok(serde_json::Value::Array(values))
}
#[cfg(test)]
mod tests {
  use super::*;
  #[derive(Serialize)]
  struct Sample {
    n: i32,
  }
  #[test]
  fn models_into_data_array_serializes() {
    let data = models_into_data_array(vec![Sample { n: 1 }, Sample { n: 2 }]).unwrap();
    match data {
      serde_json::Value::Array(v) => {
        assert_eq!(v.len(), 2);
        assert_eq!(v[0]["n"], 1);
        assert_eq!(v[1]["n"], 2);
      }
      _ => panic!("expected array"),
    }
  }
}
