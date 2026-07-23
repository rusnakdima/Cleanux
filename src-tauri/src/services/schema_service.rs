use nosql_orm::prelude::*;
use std::sync::Arc;
use tauri_shared::response::{Response, Status};
use tauri_shared::schema::UiSchema;

pub struct SchemaService {
  provider: Arc<JsonProvider>,
}

impl SchemaService {
  pub fn new(provider: Arc<JsonProvider>) -> Self {
    Self { provider }
  }

  pub async fn get_schema(&self, id: &str) -> Result<Response<serde_json::Value>, String> {
    match self.provider.find_by_id("schemas", id).await {
      Ok(Some(data)) => {
        let schema: UiSchema =
          serde_json::from_value(data).map_err(|e| format!("Invalid schema format: {}", e))?;
        Ok(Response {
          status: Status::Success,
          message: "Schema loaded".into(),
          data: Some(serde_json::to_value(schema).unwrap_or_else(|_| serde_json::Value::Null)),
        })
      }
      Ok(None) => Ok(Response {
        status: Status::NotFound,
        message: format!("Schema {} not found", id),
        data: Some(serde_json::Value::Null),
      }),
      Err(e) => Err(e.to_string()),
    }
  }

  pub async fn save_schema(&self, schema: UiSchema) -> Result<Response<serde_json::Value>, String> {
    let id = schema.schema_version.clone();
    let data =
      serde_json::to_value(&schema).map_err(|e| format!("Failed to serialize schema: {}", e))?;

    match self.provider.find_by_id("schemas", &id).await {
      Ok(Some(_)) => {
        self
          .provider
          .update("schemas", &id, data)
          .await
          .map_err(|e| e.to_string())?;
      }
      Ok(None) => {
        self
          .provider
          .insert("schemas", data)
          .await
          .map_err(|e| e.to_string())?;
      }
      Err(e) => return Err(e.to_string()),
    }

    Ok(Response {
      status: Status::Success,
      message: "Schema saved".into(),
      data: Some(serde_json::json!({ "id": id })),
    })
  }

  pub async fn get_all_schemas(&self) -> Result<Response<serde_json::Value>, String> {
    match self.provider.find_all("schemas").await {
      Ok(items) => Ok(Response {
        status: Status::Success,
        message: format!("Found {} schemas", items.len()),
        data: Some(serde_json::to_value(items).unwrap_or_else(|_| serde_json::Value::Null)),
      }),
      Err(e) => Err(e.to_string()),
    }
  }

  pub async fn delete_schema(&self, id: &str) -> Result<Response<serde_json::Value>, String> {
    match self.provider.delete("schemas", id).await {
      Ok(true) => Ok(Response {
        status: Status::Deleted,
        message: format!("Schema {} deleted", id),
        data: Some(serde_json::Value::Null),
      }),
      Ok(false) => Ok(Response {
        status: Status::NotFound,
        message: format!("Schema {} not found", id),
        data: Some(serde_json::Value::Null),
      }),
      Err(e) => Err(e.to_string()),
    }
  }
}
