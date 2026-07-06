use crate::models::response::{ResponseModel, Status};
use nosql_orm::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiSchema {
  pub schema_version: String,
  pub app: AppConfig,
  pub pages: Vec<Page>,
  pub layouts: Vec<Layout>,
  pub components: Vec<ComponentDef>,
  #[serde(default)]
  pub shared_components: Vec<ComponentDef>,
  pub services: Vec<ServiceDef>,
  pub modules: Vec<ModuleDef>,
  #[serde(rename = "i18N")]
  pub i18n: I18nConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
  pub id: String,
  pub name: String,
  pub version: String,
  pub description: String,
  pub identifier: String,
  pub settings: AppSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
  pub default_locale: String,
  pub supported_locales: Vec<String>,
  pub theme: String,
  pub themes: Vec<String>,
  pub color_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
  pub id: String,
  pub name: String,
  pub route: String,
  pub layout: String,
  pub meta: PageMeta,
  #[serde(default)]
  pub sections: serde_json::Value,
  #[serde(default)]
  pub canvas_elements: Vec<CanvasElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMeta {
  pub title: String,
  pub icon: Option<String>,
  pub breadcrumb: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasElement {
  pub id: String,
  pub component_id: String,
  pub props: serde_json::Value,
  pub grid_position: GridPosition,
  pub data_binding: Option<DataBinding>,
  #[serde(default)]
  pub events: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridPosition {
  pub column: i32,
  pub row: i32,
  pub col_span: i32,
  pub row_span: i32,
  #[serde(default)]
  pub col_start: Option<i32>,
  #[serde(default)]
  pub row_start: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataBinding {
  pub entity: String,
  #[serde(default)]
  pub field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
  pub id: String,
  pub name: String,
  pub slots: std::collections::HashMap<String, LayoutSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutSlot {
  pub name: String,
  pub elements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDef {
  pub id: String,
  pub name: String,
  pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceDef {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleDef {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nConfig {
  pub locales: std::collections::HashMap<String, LocaleMap>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocaleMap {
  pub nav: std::collections::HashMap<String, String>,
  pub actions: std::collections::HashMap<String, String>,
  pub messages: std::collections::HashMap<String, String>,
}

pub struct SchemaService {
  provider: Arc<JsonProvider>,
}

impl SchemaService {
  pub fn new(provider: Arc<JsonProvider>) -> Self {
    Self { provider }
  }

  pub async fn get_schema(&self, id: &str) -> Result<ResponseModel, String> {
    match self.provider.find_by_id("schemas", id).await {
      Ok(Some(data)) => {
        let schema: UiSchema =
          serde_json::from_value(data).map_err(|e| format!("Invalid schema format: {}", e))?;
        Ok(ResponseModel {
          status: Status::Success,
          message: "Schema loaded".into(),
          data: serde_json::to_value(schema).unwrap_or_else(|_| serde_json::Value::Null),
        })
      }
      Ok(None) => Ok(ResponseModel {
        status: Status::NotFound,
        message: format!("Schema {} not found", id),
        data: serde_json::Value::Null,
      }),
      Err(e) => Err(e.to_string()),
    }
  }

  pub async fn save_schema(&self, schema: UiSchema) -> Result<ResponseModel, String> {
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

    Ok(ResponseModel {
      status: Status::Success,
      message: "Schema saved".into(),
      data: serde_json::json!({ "id": id }),
    })
  }

  pub async fn get_all_schemas(&self) -> Result<ResponseModel, String> {
    match self.provider.find_all("schemas").await {
      Ok(items) => Ok(ResponseModel {
        status: Status::Success,
        message: format!("Found {} schemas", items.len()),
        data: serde_json::to_value(items).unwrap_or_else(|_| serde_json::Value::Null),
      }),
      Err(e) => Err(e.to_string()),
    }
  }

  pub async fn delete_schema(&self, id: &str) -> Result<ResponseModel, String> {
    match self.provider.delete("schemas", id).await {
      Ok(true) => Ok(ResponseModel {
        status: Status::Deleted,
        message: format!("Schema {} deleted", id),
        data: serde_json::Value::Null,
      }),
      Ok(false) => Ok(ResponseModel {
        status: Status::NotFound,
        message: format!("Schema {} not found", id),
        data: serde_json::Value::Null,
      }),
      Err(e) => Err(e.to_string()),
    }
  }
}
