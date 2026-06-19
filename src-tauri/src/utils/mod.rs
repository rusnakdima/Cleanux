/* utils module */

#[path = "response.helper.rs"]
pub mod response_helper;

#[path = "filesystem.helper.rs"]
pub mod filesystem_helper;

#[path = "process.helper.rs"]
pub mod process_helper;

#[path = "validation.helper.rs"]
pub mod validation_helper;

#[path = "common-paths.helper.rs"]
pub mod common_paths;

pub use filesystem_helper::{
  calculate_dir_size, clean_cache_dir, collect_cache_file_models, collect_log_file_models,
  collect_trash_file_models, format_size, get_dir_size, home_dir, remove_dir_contents,
  remove_paths_with_errors, scan_large_file_models,
};
pub use process_helper::{
  get_command_output, pkexec, pkexec_rm_paths, pkexec_with_args, run_command, run_command_checked,
  run_command_ignore_error, run_command_raw, stderr_string, stdout_string,
};
pub use response_helper::{
  array_response, data_empty_string, data_string, error_response, info_response,
  models_into_data_array, success_response, ResponseBuilder,
};

#[path = "service_macro.rs"]
pub mod service_macro;
pub use crate::service_method_full;

#[path = "cache.helper.rs"]
pub mod cache_helper;
pub use cache_helper::TimedCache;
