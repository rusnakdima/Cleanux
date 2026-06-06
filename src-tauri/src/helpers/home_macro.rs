#[macro_export]
macro_rules! home {
  () => {{
    dirs::home_dir().expect("Home directory not found")
  }};
}

#[macro_export]
macro_rules! config_dir {
  () => {{
    dirs::config_dir().expect("Config directory not found")
  }};
}
