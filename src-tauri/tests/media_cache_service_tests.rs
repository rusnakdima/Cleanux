use cleanux_lib::services::media_cache_service::{MediaCacheService, SteamInfo};
use std::fs;

#[test]
fn test_media_cache_service_get_summary() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".cache/thumbnails")).unwrap();
  fs::write(home.join(".cache/thumbnails").join("thumb.db"), "test").unwrap();

  let service = MediaCacheService;
  let result = service.get_media_cache_summary();

  assert_eq!(result.status, cleanux_lib::models::ResponseStatus::Success);
}

#[test]
fn test_media_cache_service_clean_steam_shader_cache() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".steam/steam/steamapps/shader")).unwrap();
  fs::write(
    home.join(".steam/steam/steamapps/shader").join("test.txt"),
    "test",
  )
  .unwrap();

  let service = MediaCacheService;
  let result = service.clean_steam_shader_cache();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_media_cache_service_clean_steam_download_cache() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".local/share/Steam")).unwrap();
  fs::write(home.join(".local/share/Steam").join("test.txt"), "test").unwrap();

  let service = MediaCacheService;
  let result = service.clean_steam_download_cache();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_media_cache_service_clean_spotify_cache() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".cache/spotify")).unwrap();
  fs::write(home.join(".cache/spotify").join("test.txt"), "test").unwrap();

  let service = MediaCacheService;
  let result = service.clean_spotify_cache();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_media_cache_service_clean_vlc_cache() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".cache/vlc")).unwrap();
  fs::write(home.join(".cache/vlc").join("test.txt"), "test").unwrap();

  let service = MediaCacheService;
  let result = service.clean_vlc_cache();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_media_cache_service_clean_thumbnail_cache() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".cache/thumbnails")).unwrap();
  fs::write(home.join(".cache/thumbnails").join("test.txt"), "test").unwrap();

  let service = MediaCacheService;
  let result = service.clean_thumbnail_cache();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_media_cache_service_clean_icon_cache() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".cache/icons")).unwrap();
  fs::write(home.join(".cache/icons").join("test.txt"), "test").unwrap();

  let service = MediaCacheService;
  let result = service.clean_icon_cache();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_steam_info_fields() {
  let info = SteamInfo {
    game_count: 10,
    shader_cache_size: 1024,
    download_cache_size: 2048,
  };

  assert_eq!(info.game_count, 10);
  assert_eq!(info.shader_cache_size, 1024);
  assert_eq!(info.download_cache_size, 2048);
}

#[test]
fn test_media_cache_service_get_steam_info() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".steam/steam")).unwrap();

  let service = MediaCacheService;
  let info = service.get_steam_info();

  assert_eq!(info.game_count, 0);
}

#[test]
fn test_media_cache_service_get_spotify_cache_size() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  let spotify_cache = home.join(".cache/spotify");
  fs::create_dir_all(&spotify_cache).unwrap();
  fs::write(spotify_cache.join("test.txt"), "test content").unwrap();

  let service = MediaCacheService;
  let size = service.get_spotify_cache_size();

  assert!(size >= 0);
}

#[test]
fn test_media_cache_service_get_vlc_cache_size() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".cache/vlc")).unwrap();
  fs::write(home.join(".cache/vlc").join("test.txt"), "test content").unwrap();

  let service = MediaCacheService;
  let size = service.get_vlc_cache_size();

  assert!(size >= 0);
}

#[test]
fn test_media_cache_service_get_thumbnail_cache_size() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".cache/thumbnails")).unwrap();
  fs::write(
    home.join(".cache/thumbnails").join("test.txt"),
    "test content",
  )
  .unwrap();

  let service = MediaCacheService;
  let size = service.get_thumbnail_cache_size();

  assert!(size >= 0);
}
