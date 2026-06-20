#[macro_export]
macro_rules! crud_get_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command(rename_all = "camelCase")]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      id: Option<String>,
    ) -> Result<
      crate::models::Response<serde_json::Value>,
      crate::models::Response<serde_json::Value>,
    > {
      use crate::models::{Response, Status};
      if let Some(id) = id {
        let doc = state
          .data
          .repository_service
          .find_by_id($table, &id)
          .await
          .map_err(|e| Response::error(Status::Error, e.to_string()))?
          .ok_or_else(|| Response::error(Status::NotFound, "Entity not found"))?;
        Ok(Response::success("Entity found", doc))
      } else {
        Err(Response::error(Status::Error, "ID is required"))
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
    ) -> Result<
      crate::models::Response<Vec<serde_json::Value>>,
      crate::models::Response<serde_json::Value>,
    > {
      use crate::models::{Response, Status};
      let docs = state
        .data
        .repository_service
        .find_many($table, None, page, limit, None, true)
        .await
        .map_err(|e| Response::error(Status::Error, e.to_string()))?;
      Ok(Response::success("Entities retrieved", docs))
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
    ) -> Result<
      crate::models::Response<serde_json::Value>,
      crate::models::Response<serde_json::Value>,
    > {
      use crate::models::{Response, Status};
      let doc = state
        .data
        .repository_service
        .insert($table, data)
        .await
        .map_err(|e| Response::error(Status::Error, e.to_string()))?;
      Ok(Response::success("Entity created", doc))
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
    ) -> Result<
      crate::models::Response<serde_json::Value>,
      crate::models::Response<serde_json::Value>,
    > {
      use crate::models::{Response, Status};
      let doc = state
        .data
        .repository_service
        .update($table, &id, data)
        .await
        .map_err(|e| Response::error(Status::Error, e.to_string()))?;
      Ok(Response::success("Entity updated", doc))
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
    ) -> Result<
      crate::models::Response<serde_json::Value>,
      crate::models::Response<serde_json::Value>,
    > {
      use crate::models::{Response, Status};
      let _ = state
        .data
        .repository_service
        .delete($table, &id)
        .await
        .map_err(|e| Response::error(Status::Error, e.to_string()))?;
      Ok(Response::success("Entity deleted", serde_json::Value::Null))
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
    ) -> Result<
      crate::models::Response<serde_json::Value>,
      crate::models::Response<serde_json::Value>,
    > {
      use crate::models::{Response, Status};
      let doc = state
        .data
        .repository_service
        .patch($table, &id, patch)
        .await
        .map_err(|e| Response::error(Status::Error, e.to_string()))?;
      Ok(Response::success("Entity patched", doc))
    }
  };
}
