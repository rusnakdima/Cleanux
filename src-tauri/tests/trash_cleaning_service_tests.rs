use cleanux_lib::models::{ResponseStatus, TrashFileModel};
use cleanux_lib::services::trash_cleaning_service::TrashCleaningService;
use std::fs;

#[test]
fn test_get_trash_files_empty() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".local/share/Trash/files")).unwrap();

  let service = TrashCleaningService;
  let result = service.getTrashFiles();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}

#[test]
fn test_get_trash_files_with_items() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  let trash_dir = home.join(".local/share/Trash/files");
  fs::create_dir_all(&trash_dir).unwrap();
  fs::write(trash_dir.join("file1.txt"), "test content 1").unwrap();
  fs::write(trash_dir.join("file2.txt"), "test content 2").unwrap();

  let service = TrashCleaningService;
  let result = service.getTrashFiles();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}

#[test]
fn test_clear_selected_trash_files_empty() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  let service = TrashCleaningService;
  let result = service.clearSelectedTrashFiles(vec![]);

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}

#[test]
fn test_clear_selected_trash_files_nonexistent() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  let service = TrashCleaningService;
  let result = service.clearSelectedTrashFiles(vec!["/nonexistent/path/file.txt".to_string()]);

  assert!(result.is_err());
}

#[test]
fn test_clear_trash_empty() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".local/share/Trash/files")).unwrap();

  let service = TrashCleaningService;
  let result = service.clearTrash();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}

#[test]
fn test_clear_trash_with_files() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  let trash_dir = home.join(".local/share/Trash/files");
  fs::create_dir_all(&trash_dir).unwrap();
  fs::write(trash_dir.join("file1.txt"), "test content").unwrap();
  fs::write(trash_dir.join("file2.txt"), "test content").unwrap();

  let service = TrashCleaningService;
  let result = service.clearTrash();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}

#[test]
fn test_trash_file_model_serialization() {
  let model = TrashFileModel {
    name: "test_file.txt".to_string(),
    path: "/home/user/.local/share/Trash/files/test_file.txt".to_string(),
    size: 1024,
    deletedDate: "2024-01-15 10:30:00".to_string(),
  };

  let json = serde_json::to_string(&model).unwrap();
  assert!(json.contains("test_file.txt"));
  assert!(json.contains("path"));
  assert!(json.contains("size"));
  assert!(json.contains("deletedDate"));
  assert!(json.contains("1024"));
}

#[test]
fn test_trash_file_model_deserialization() {
  let json = r#"{"name":"file.txt","path":"/trash/file.txt","size":2048,"deletedDate":"2024-01-15 10:30:00"}"#;
  let model: TrashFileModel = serde_json::from_str(json).unwrap();

  assert_eq!(model.name, "file.txt");
  assert_eq!(model.path, "/trash/file.txt");
  assert_eq!(model.size, 2048);
  assert_eq!(model.deletedDate, "2024-01-15 10:30:00");
}

#[test]
fn test_trash_file_model_clone() {
  let model = TrashFileModel {
    name: "clone_test.txt".to_string(),
    path: "/trash/clone_test.txt".to_string(),
    size: 512,
    deletedDate: "2024-01-15 10:30:00".to_string(),
  };

  let cloned = model.clone();
  assert_eq!(cloned.name, model.name);
  assert_eq!(cloned.path, model.path);
  assert_eq!(cloned.size, model.size);
  assert_eq!(cloned.deletedDate, model.deletedDate);
}

#[test]
fn test_scan_nested_trash_directories() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  let trash_dir = home.join(".local/share/Trash/files");
  fs::create_dir_all(&trash_dir).unwrap();
  fs::write(trash_dir.join("root_file.txt"), "root content").unwrap();

  let sub_dir = trash_dir.join("subdir");
  fs::create_dir_all(&sub_dir).unwrap();
  fs::write(sub_dir.join("nested_file.txt"), "nested content").unwrap();

  let service = TrashCleaningService;
  let result = service.getTrashFiles();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}
