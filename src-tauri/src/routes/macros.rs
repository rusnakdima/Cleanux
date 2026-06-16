#[macro_export]
macro_rules! crud_get_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      id: Option<String>,
    ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
      use crate::models::{DataValue, ResponseModel, ResponseStatus};

      if let Some(id) = id {
        let doc = state
          .data
          .repository_service
          .find_by_id($table, &id)
          .await
          .map_err(|e| ResponseModel::from(e))?
          .ok_or_else(|| ResponseModel {
            status: ResponseStatus::Error,
            message: "Entity not found".to_string(),
            data: DataValue::String(String::new()),
          })?;
        Ok(ResponseModel {
          status: ResponseStatus::Success,
          message: "Entity found".to_string(),
          data: DataValue::Object(doc),
        })
      } else {
        Err(ResponseModel {
          status: ResponseStatus::Error,
          message: "ID is required".to_string(),
          data: DataValue::String(String::new()),
        })
      }
    }
  };
}

#[macro_export]
macro_rules! crud_get_all_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      page: Option<u64>,
      limit: Option<u64>,
    ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
      use crate::models::{DataValue, ResponseModel, ResponseStatus};

      let docs = state
        .data
        .repository_service
        .find_many($table, None, page, limit, None, true)
        .await
        .map_err(|e| ResponseModel::from(e))?;
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: "Entities retrieved".to_string(),
        data: DataValue::Array(docs),
      })
    }
  };
}

#[macro_export]
macro_rules! crud_create_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      data: serde_json::Value,
    ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
      use crate::models::{DataValue, ResponseModel, ResponseStatus};

      let doc = state
        .data
        .repository_service
        .insert($table, data)
        .await
        .map_err(|e| ResponseModel::from(e))?;
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: "Entity created".to_string(),
        data: DataValue::Object(doc),
      })
    }
  };
}

#[macro_export]
macro_rules! crud_update_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      id: String,
      data: serde_json::Value,
    ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
      use crate::models::{DataValue, ResponseModel, ResponseStatus};

      let doc = state
        .data
        .repository_service
        .update($table, &id, data)
        .await
        .map_err(|e| ResponseModel::from(e))?;
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: "Entity updated".to_string(),
        data: DataValue::Object(doc),
      })
    }
  };
}

#[macro_export]
macro_rules! crud_delete_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      id: String,
    ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
      use crate::models::{DataValue, ResponseModel, ResponseStatus};

      let _ = state
        .data
        .repository_service
        .delete($table, &id)
        .await
        .map_err(|e| ResponseModel::from(e))?;
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: "Entity deleted".to_string(),
        data: DataValue::String(String::new()),
      })
    }
  };
}

#[macro_export]
macro_rules! crud_patch_command {
  ($route:ident, $table:expr) => {
    #[allow(dead_code)]
    #[tauri::command]
    pub async fn $route(
      state: tauri::State<'_, crate::AppState>,
      id: String,
      patch: serde_json::Value,
    ) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
      use crate::models::{DataValue, ResponseModel, ResponseStatus};

      let doc = state
        .data
        .repository_service
        .patch($table, &id, patch)
        .await
        .map_err(|e| ResponseModel::from(e))?;
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: "Entity patched".to_string(),
        data: DataValue::Object(doc),
      })
    }
  };
}
