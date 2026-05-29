/* helpers module */

#[path = "response.helper.rs"]
pub mod response_helper;

#[path = "filesystem.helper.rs"]
pub mod filesystem_helper;

#[path = "process.helper.rs"]
pub mod process_helper;

#[path = "validation.helper.rs"]
pub mod validation_helper;

pub use filesystem_helper::{
  calculate_dir_size, collect_cache_file_models, collect_log_file_models,
  collect_trash_file_models, format_size, get_dir_size, remove_dir_contents,
  remove_paths_with_errors, scan_large_file_models,
};
pub use process_helper::{pkexec_rm_paths, stderr_message};
pub use response_helper::{
  data_empty_string, data_string, error_response, info_response, models_into_data_array,
  success_response, ResponseBuilder,
};
