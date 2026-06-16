use serde_json::Value;
use std::sync::Arc;

use nosql_orm::provider::DatabaseProvider;
use nosql_orm::query::Filter;

pub struct RepositoryService {
  json_provider: Arc<nosql_orm::providers::JsonProvider>,
}

impl RepositoryService {
  pub fn new(json_provider: nosql_orm::providers::JsonProvider) -> Self {
    Self {
      json_provider: Arc::new(json_provider),
    }
  }

  pub async fn find_by_id(
    &self,
    collection: &str,
    id: &str,
  ) -> Result<Option<Value>, nosql_orm::error::OrmError> {
    self.json_provider.find_by_id(collection, id).await
  }

  pub async fn find_many(
    &self,
    collection: &str,
    filter: Option<Filter>,
    skip: Option<u64>,
    limit: Option<u64>,
    sort_by: Option<&str>,
    sort_asc: bool,
  ) -> Result<Vec<Value>, nosql_orm::error::OrmError> {
    self
      .json_provider
      .find_many(collection, filter.as_ref(), skip, limit, sort_by, sort_asc)
      .await
  }

  pub async fn insert(
    &self,
    collection: &str,
    data: Value,
  ) -> Result<Value, nosql_orm::error::OrmError> {
    self.json_provider.insert(collection, data).await
  }

  pub async fn update(
    &self,
    collection: &str,
    id: &str,
    data: Value,
  ) -> Result<Value, nosql_orm::error::OrmError> {
    self.json_provider.update(collection, id, data).await
  }

  pub async fn patch(
    &self,
    collection: &str,
    id: &str,
    patch: Value,
  ) -> Result<Value, nosql_orm::error::OrmError> {
    self.json_provider.patch(collection, id, patch).await
  }

  pub async fn delete(
    &self,
    collection: &str,
    id: &str,
  ) -> Result<bool, nosql_orm::error::OrmError> {
    self.json_provider.delete(collection, id).await
  }

  pub async fn count(
    &self,
    collection: &str,
    filter: Option<&Filter>,
  ) -> Result<u64, nosql_orm::error::OrmError> {
    self.json_provider.count(collection, filter).await
  }
}

impl Clone for RepositoryService {
  fn clone(&self) -> Self {
    Self {
      json_provider: self.json_provider.clone(),
    }
  }
}
