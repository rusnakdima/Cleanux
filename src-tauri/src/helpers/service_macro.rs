/// Macro to generate service method boilerplate with error conversion.
///
/// Usage:
/// ```rust
/// service_method_full!(method_name => method_name_inner);
/// ```
///
/// This generates:
/// ```rust
/// pub fn method_name(&self) -> Result<ResponseModel, ResponseModel> {
///     self.method_name_inner().map_err(|e| e.into_response())
/// }
/// ```
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
