/* models */
use crate::models::{DataValue, ResponseModel, ResponseStatus};
/* sys lib */
use serde::Serialize;

pub struct ResponseBuilder {
  status: Option<ResponseStatus>,
  message: Option<String>,
  data: Option<DataValue>,
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
    self.status = Some(ResponseStatus::Success);
    self.message = Some(msg.to_string());
    self
  }

  pub fn info(mut self, msg: &str) -> Self {
    self.status = Some(ResponseStatus::Info);
    self.message = Some(msg.to_string());
    self
  }

  pub fn error(mut self, msg: &str) -> Self {
    self.status = Some(ResponseStatus::Error);
    self.message = Some(msg.to_string());
    self
  }

  pub fn data(mut self, value: DataValue) -> Self {
    self.data = Some(value);
    self
  }

  pub fn build(self) -> ResponseModel {
    ResponseModel {
      status: self.status.unwrap_or(ResponseStatus::Info),
      message: self.message.unwrap_or_default(),
      data: self.data.unwrap_or(DataValue::String(String::new())),
    }
  }
}

impl Default for ResponseBuilder {
  fn default() -> Self {
    Self::new()
  }
}

pub fn success_response(message: impl Into<String>, data: DataValue) -> ResponseModel {
  ResponseModel {
    status: ResponseStatus::Success,
    message: message.into(),
    data,
  }
}

pub fn info_response(message: impl Into<String>, data: DataValue) -> ResponseModel {
  ResponseModel {
    status: ResponseStatus::Info,
    message: message.into(),
    data,
  }
}

pub fn error_response(message: impl Into<String>, data: DataValue) -> ResponseModel {
  ResponseModel {
    status: ResponseStatus::Error,
    message: message.into(),
    data,
  }
}

pub fn data_empty_string() -> DataValue {
  DataValue::String(String::new())
}

pub fn data_string(value: impl Into<String>) -> DataValue {
  DataValue::String(value.into())
}

/// Serialize models to JSON values; propagates first serialization failure instead of swallowing it.
pub fn models_into_data_array<T: Serialize>(items: Vec<T>) -> Result<DataValue, serde_json::Error> {
  let values: Vec<serde_json::Value> = items
    .into_iter()
    .map(|item| serde_json::to_value(item))
    .collect::<Result<_, _>>()?;
  Ok(DataValue::Array(values))
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde::Serialize;

  #[derive(Serialize)]
  struct Sample {
    n: i32,
  }

  #[test]
  fn models_into_data_array_serializes() {
    let data = models_into_data_array(vec![Sample { n: 1 }, Sample { n: 2 }]).unwrap();
    match data {
      DataValue::Array(v) => {
        assert_eq!(v.len(), 2);
        assert_eq!(v[0]["n"], 1);
        assert_eq!(v[1]["n"], 2);
      }
      _ => panic!("expected array"),
    }
  }
}
