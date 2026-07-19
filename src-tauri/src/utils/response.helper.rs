/* helpers */
use serde::Serialize;
use tauri_shared::response::{Response, Status};

/// A fluent builder for constructing Response objects.
pub struct ResponseBuilder {
  status: Status,
  message: String,
  data: Option<serde_json::Value>,
}

impl ResponseBuilder {
  /// Creates a new ResponseBuilder with default Info status and empty message.
  pub fn new() -> Self {
    Self {
      status: Status::Info,
      message: String::new(),
      data: None,
    }
  }

  /// Sets the response status to Success with a message.
  pub fn success(mut self, message: impl Into<String>) -> Self {
    self.status = Status::Success;
    self.message = message.into();
    self
  }

  /// Sets the response status to Info with a message.
  pub fn info(mut self, message: impl Into<String>) -> Self {
    self.status = Status::Info;
    self.message = message.into();
    self
  }

  /// Sets the response status to Error with a message.
  pub fn error(mut self, message: impl Into<String>) -> Self {
    self.status = Status::Error;
    self.message = message.into();
    self
  }

  /// Sets the response data.
  pub fn data(mut self, data: serde_json::Value) -> Self {
    self.data = Some(data);
    self
  }

  /// Builds the final Response object.
  pub fn build(self) -> Response<serde_json::Value> {
    Response {
      status: self.status,
      message: self.message,
      data: self.data,
    }
  }
}

impl Default for ResponseBuilder {
  fn default() -> Self {
    Self::new()
  }
}

/// Creates a success response with data and message.
pub fn success_response(
  data: serde_json::Value,
  message: impl Into<String>,
) -> Response<serde_json::Value> {
  Response::success(data, Some(&message.into()))
}

/// Creates an info response with data and message.
pub fn info_response(
  data: serde_json::Value,
  message: impl Into<String>,
) -> Response<serde_json::Value> {
  Response::success(data, Some(&message.into()))
}

/// Creates an error response with a message and optional data.
pub fn error_response(
  message: impl Into<String>,
  _data: serde_json::Value,
) -> Response<serde_json::Value> {
  Response::error_with_status(Status::Error, message)
}

/// Returns an empty string JSON value.
pub fn data_empty_string() -> serde_json::Value {
  serde_json::Value::String(String::new())
}

/// Creates a string JSON value.
pub fn data_string(value: impl Into<String>) -> serde_json::Value {
  serde_json::Value::String(value.into())
}

/// Creates a success response from a vector of items, converting each to JSON.
///
/// Returns an error response if any item fails to serialize.
pub fn array_response<T: Serialize>(
  items: Vec<T>,
  message: impl Into<String>,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let data: Vec<serde_json::Value> = items
    .into_iter()
    .map(|item| serde_json::to_value(item))
    .collect::<Result<_, _>>()
    .map_err(|e| {
      Response::error_with_status(Status::Error, format!("Serialization error: {}", e))
    })?;
  Ok(Response::success(
    serde_json::Value::Array(data),
    Some(&message.into()),
  ))
}

/// Converts a vector of serializable items into a JSON array value.
///
/// Returns an error if any item fails to serialize.
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
