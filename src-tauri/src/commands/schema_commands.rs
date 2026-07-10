use crate::services::schema_service::UiSchema;
use crate::AppState;
use crate::Response;

#[tauri::command]
pub async fn get_cleanux_schema(
  state: tauri::State<'_, AppState>,
) -> Result<Response<serde_json::Value>, String> {
  state.schema.schema_service.get_schema("cleanux").await
}

#[tauri::command]
pub async fn save_cleanux_schema(
  state: tauri::State<'_, AppState>,
  schema: UiSchema,
) -> Result<Response<serde_json::Value>, String> {
  state.schema.schema_service.save_schema(schema).await
}
