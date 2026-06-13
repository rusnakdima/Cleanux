#[macro_export]
macro_rules! define_singleton_service {
  ($name:ident, $service_type:ty, $init_method:ident) => {
    static $name: std::sync::OnceLock<$service_type> = std::sync::OnceLock::new();
    pub fn get_service() -> &'static $service_type {
      $name.get_or_init(|| {
        let svc = <$service_type>::new();
        svc.$init_method().ok();
        svc
      })
    }
  };
}
