#[macro_export]
macro_rules! crud_get_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command(rename_all = "camelCase")]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      id: Option<String>,
    ) -> Result<crate::Response<serde_json::Value>, crate::Response<serde_json::Value>> {
      use nosql_orm::provider::DatabaseProvider;
      use tauri_shared::response::Response;
      if let Some(id) = id {
        let doc = state
          .data
          .json_provider
          .find_by_id($table, &id)
          .await
          .map_err(|e| Response::error(e.to_string()))?
          .ok_or_else(|| Response::error("Entity not found"))?;
        Ok(Response::success(doc, Some("Entity found")))
      } else {
        Err(Response::error("ID is required"))
      }
    }
  };
}
#[macro_export]
macro_rules! crud_get_all_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command(rename_all = "camelCase")]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      page: Option<u64>,
      limit: Option<u64>,
    ) -> Result<crate::Response<Vec<serde_json::Value>>, crate::Response<serde_json::Value>> {
      use nosql_orm::provider::DatabaseProvider;
      use tauri_shared::response::Response;
      let docs = state
        .data
        .json_provider
        .find_many($table, None, page, limit, None, true)
        .await
        .map_err(|e| Response::error(e.to_string()))?;
      Ok(Response::success(docs, Some("Entities retrieved")))
    }
  };
}
#[macro_export]
macro_rules! crud_create_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command(rename_all = "camelCase")]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      data: serde_json::Value,
    ) -> Result<crate::Response<serde_json::Value>, crate::Response<serde_json::Value>> {
      use nosql_orm::provider::DatabaseProvider;
      use tauri_shared::response::Response;
      let doc = state
        .data
        .json_provider
        .insert($table, data)
        .await
        .map_err(|e| Response::error(e.to_string()))?;
      Ok(Response::success(doc, Some("Entity created")))
    }
  };
}
#[macro_export]
macro_rules! crud_update_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command(rename_all = "camelCase")]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      id: String,
      data: serde_json::Value,
    ) -> Result<crate::Response<serde_json::Value>, crate::Response<serde_json::Value>> {
      use nosql_orm::provider::DatabaseProvider;
      use tauri_shared::response::Response;
      let doc = state
        .data
        .json_provider
        .update($table, &id, data)
        .await
        .map_err(|e| Response::error(e.to_string()))?;
      Ok(Response::success(doc, Some("Entity updated")))
    }
  };
}
#[macro_export]
macro_rules! crud_delete_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command(rename_all = "camelCase")]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      id: String,
    ) -> Result<crate::Response<serde_json::Value>, crate::Response<serde_json::Value>> {
      use nosql_orm::provider::DatabaseProvider;
      use tauri_shared::response::Response;
      let _ = state
        .data
        .json_provider
        .delete($table, &id)
        .await
        .map_err(|e| Response::error(e.to_string()))?;
      Ok(Response::success(
        serde_json::Value::Null,
        Some("Entity deleted"),
      ))
    }
  };
}
#[macro_export]
macro_rules! crud_patch_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command(rename_all = "camelCase")]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      id: String,
      patch: serde_json::Value,
    ) -> Result<crate::Response<serde_json::Value>, crate::Response<serde_json::Value>> {
      use nosql_orm::provider::DatabaseProvider;
      use tauri_shared::response::{Response, Status};
      let doc = state
        .data
        .json_provider
        .patch($table, &id, patch)
        .await
        .map_err(|e| Response::error(e.to_string()))?;
      Ok(Response::success(doc, Some("Entity patched")))
    }
  };
}
