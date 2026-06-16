#[macro_export]
macro_rules! service_method_full {
  ($method:ident => $inner:ident) => {
    pub fn $method(
      &self,
    ) -> ::std::result::Result<$crate::models::ResponseModel, $crate::models::ResponseModel> {
      self.$inner().map_err(|e| e.into_response())
    }
  };
}
