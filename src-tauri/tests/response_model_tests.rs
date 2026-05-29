use cleanux_lib::models::{DataValue, ResponseModel, ResponseStatus};

#[test]
fn test_response_model_success() {
  let response = ResponseModel {
    status: ResponseStatus::Success,
    message: "Test message".to_string(),
    data: DataValue::String("test".to_string()),
  };
  assert_eq!(response.status, ResponseStatus::Success);
  assert_eq!(response.message, "Test message");
}

#[test]
fn test_response_model_error() {
  let response = ResponseModel {
    status: ResponseStatus::Error,
    message: "Error occurred".to_string(),
    data: DataValue::Number(42.0),
  };
  assert_eq!(response.status, ResponseStatus::Error);
}

#[test]
fn test_response_model_display() {
  let response = ResponseModel {
    status: ResponseStatus::Success,
    message: "Hello".to_string(),
    data: DataValue::Bool(true),
  };
  assert_eq!(format!("{}", response), "Hello");
}

#[test]
fn test_data_value_string() {
  let data = DataValue::String("value".to_string());
  assert!(matches!(data, DataValue::String(s) if s == "value"));
}

#[test]
fn test_data_value_array() {
  let arr = vec![serde_json::json!("a"), serde_json::json!(1)];
  let data = DataValue::Array(arr);
  assert!(matches!(data, DataValue::Array(arr) if arr.len() == 2));
}

#[test]
fn test_response_status_variants() {
  assert!(matches!(ResponseStatus::Success, ResponseStatus::Success));
  assert!(matches!(ResponseStatus::Error, ResponseStatus::Error));
  assert!(matches!(ResponseStatus::Warning, ResponseStatus::Warning));
  assert!(matches!(ResponseStatus::Info, ResponseStatus::Info));
}
