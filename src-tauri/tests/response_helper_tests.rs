use cleanux_lib::helpers::response_helper::{
  data_empty_string, data_string, error_response, info_response, models_into_data_array,
  success_response, ResponseBuilder,
};
use cleanux_lib::models::{DataValue, ResponseStatus};
#[test]
fn test_response_builder_new() {
  let builder = ResponseBuilder::new();
  let response = builder.build();
  assert_eq!(response.status, ResponseStatus::Info);
}
#[test]
fn test_response_builder_success() {
  let response = ResponseBuilder::new()
    .success("Operation successful")
    .build();
  assert_eq!(response.status, ResponseStatus::Success);
  assert_eq!(response.message, "Operation successful");
}
#[test]
fn test_response_builder_info() {
  let response = ResponseBuilder::new().info("Informational message").build();
  assert_eq!(response.status, ResponseStatus::Info);
}
#[test]
fn test_response_builder_error() {
  let response = ResponseBuilder::new().error("Something went wrong").build();
  assert_eq!(response.status, ResponseStatus::Error);
  assert_eq!(response.message, "Something went wrong");
}
#[test]
fn test_response_builder_with_data() {
  let response = ResponseBuilder::new()
    .success("OK")
    .data(DataValue::String("test data".to_string()))
    .build();
  assert_eq!(response.status, ResponseStatus::Success);
  assert_eq!(response.message, "OK");
  match response.data {
    DataValue::String(s) => assert_eq!(s, "test data"),
    _ => panic!("Expected String data"),
  }
}
#[test]
fn test_response_builder_chaining() {
  let response = ResponseBuilder::new()
    .success("Done")
    .data(DataValue::Number(42.0))
    .build();
  assert_eq!(response.status, ResponseStatus::Success);
  assert_eq!(response.message, "Done");
}
#[test]
fn test_success_response() {
  let response = success_response("Success!", DataValue::String("data".to_string()));
  assert_eq!(response.status, ResponseStatus::Success);
  assert_eq!(response.message, "Success!");
}
#[test]
fn test_info_response() {
  let response = info_response("Info", DataValue::Bool(true));
  assert_eq!(response.status, ResponseStatus::Info);
  assert_eq!(response.message, "Info");
}
#[test]
fn test_error_response() {
  let response = error_response("Error occurred", DataValue::Number(0.0));
  assert_eq!(response.status, ResponseStatus::Error);
  assert_eq!(response.message, "Error occurred");
}
#[test]
fn test_data_empty_string() {
  let data = data_empty_string();
  match data {
    DataValue::String(s) => assert_eq!(s, ""),
    _ => panic!("Expected empty string"),
  }
}
#[test]
fn test_data_string() {
  let data = data_string("hello");
  match data {
    DataValue::String(s) => assert_eq!(s, "hello"),
    _ => panic!("Expected string"),
  }
}
#[test]
fn test_models_into_data_array_empty() {
  #[derive(serde::Serialize)]
  struct Item {
    name: String,
  }
  let items: Vec<Item> = vec![];
  let result = models_into_data_array(items);
  assert!(result.is_ok());
  match result.unwrap() {
    DataValue::Array(arr) => assert_eq!(arr.len(), 0),
    _ => panic!("Expected array"),
  }
}
#[test]
fn test_models_into_data_array_multiple() {
  #[derive(serde::Serialize)]
  struct Item {
    id: i32,
    name: String,
  }
  let items = vec![
    Item {
      id: 1,
      name: "First".to_string(),
    },
    Item {
      id: 2,
      name: "Second".to_string(),
    },
    Item {
      id: 3,
      name: "Third".to_string(),
    },
  ];
  let result = models_into_data_array(items);
  assert!(result.is_ok());
  match result.unwrap() {
    DataValue::Array(arr) => {
      assert_eq!(arr.len(), 3);
      assert_eq!(arr[0]["id"], 1);
      assert_eq!(arr[1]["name"], "Second");
    }
    _ => panic!("Expected array"),
  }
}
#[test]
fn test_models_into_data_array_preserves_order() {
  #[derive(serde::Serialize)]
  struct Num(i32);
  let items = vec![Num(1), Num(2), Num(3)];
  let result = models_into_data_array(items);
  assert!(result.is_ok());
  match result.unwrap() {
    DataValue::Array(arr) => {
      assert_eq!(arr[0].as_i64().unwrap(), 1);
      assert_eq!(arr[1].as_i64().unwrap(), 2);
      assert_eq!(arr[2].as_i64().unwrap(), 3);
    }
    _ => panic!("Expected array"),
  }
}
