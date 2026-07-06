use crate::models::response::ResponseModel;
use crate::services::schema_service::{SchemaService, UiSchema};
use std::sync::Arc;
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub async fn get_schema(
  state: State<'_, SchemaState>,
  id: String,
) -> Result<ResponseModel, String> {
  state.schema_service.get_schema(&id).await
}

#[tauri::command(rename_all = "camelCase")]
pub async fn save_schema(
  state: State<'_, SchemaState>,
  schema: UiSchema,
) -> Result<ResponseModel, String> {
  state.schema_service.save_schema(schema).await
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_all_schemas(state: State<'_, SchemaState>) -> Result<ResponseModel, String> {
  state.schema_service.get_all_schemas().await
}

#[tauri::command(rename_all = "camelCase")]
pub async fn delete_schema(
  state: State<'_, SchemaState>,
  id: String,
) -> Result<ResponseModel, String> {
  state.schema_service.delete_schema(&id).await
}

pub struct SchemaState {
  pub schema_service: Arc<SchemaService>,
}

impl SchemaState {
  pub fn new(json_provider: Arc<nosql_orm::providers::JsonProvider>) -> Self {
    Self {
      schema_service: Arc::new(SchemaService::new(json_provider)),
    }
  }
}
