/* helpers */
use crate::helpers::{data_empty_string, error_response};
/* models */
use crate::models::ResponseModel;
/* sys lib */
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("{0}")]
  Message(String),
  #[error(transparent)]
  SerdeJson(#[from] serde_json::Error),
}

impl AppError {
  pub fn message(text: impl Into<String>) -> Self {
    AppError::Message(text.into())
  }

  pub fn into_response(self) -> ResponseModel {
    match self {
      AppError::Message(m) => error_response(m, data_empty_string()),
      AppError::SerdeJson(e) => error_response(
        format!("Serialization error: {}", e),
        data_empty_string(),
      ),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::models::ResponseStatus;

  #[test]
  fn message_maps_to_error_response() {
    let r = AppError::message("missing").into_response();
    assert_eq!(r.status, ResponseStatus::Error);
    assert_eq!(r.message, "missing");
  }
}
