#[macro_export]
macro_rules! service_method_full {
  ($method:ident => $inner:ident) => {
    pub fn $method(
      &self,
    ) -> ::std::result::Result<
      $crate::models::Response<serde_json::Value>,
      $crate::models::Response<serde_json::Value>,
    > {
      self.$inner().map_err(|e| e.into_response())
    }
  };
}
