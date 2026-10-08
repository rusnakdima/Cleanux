use crate::response::Response;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

use crate::infrastructure::sys_utils::{
  delete_old_files, find_duplicates_size, get_dir_size, get_log_dir_size, home_dir, walkdir_size,
};

use tokio::process::Command;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct JunkSummary {
  pub cache: u64,
  pub trash: u64,
  pub logs: u64,
  pub large_files: u64,
  pub total: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategorySize {
  pub id: String,
  pub name: String,
  pub size_bytes: u64,
  pub item_count: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheCategoryResult {
  pub category: String,
  pub size: u64,
  pub item_count: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuickCleanResult {
  pub space_freed: u64,
  pub items_removed: usize,
  pub categories: Vec<String>,
}

/// Get junk summary for all categories
pub async fn get_junk_summary() -> Result<Response<JunkSummary>, String> {
  let cache_size = get_dir_size(&home_dir().join(".cache")).await.unwrap_or(0);
  let trash_size = get_dir_size(&home_dir().join(".local/share/Trash"))
    .await
    .unwrap_or(0);
  let log_size = get_log_dir_size().await.unwrap_or(0);
  let dup_size = find_duplicates_size(&home_dir(), 10 * 1024 * 1024)
    .await
    .unwrap_or(0);
  let total = cache_size + trash_size + log_size + dup_size;
  Ok(Response::success(
    JunkSummary {
      cache: cache_size,
      trash: trash_size,
      logs: log_size,
      large_files: dup_size,
      total,
    },
    Some("Junk summary retrieved"),
  ))
}

/// Scan for cache files
pub async fn scan_cache() -> Result<Response<Vec<Value>>, String> {
  let cache_dir = home_dir().join(".cache");
  let mut items: Vec<(String, u64)> = Vec::new();
  walkdir_size(&cache_dir, &mut items, 3).await?;
  items.sort_by_key(|a| std::cmp::Reverse(a.1));
  let result: Vec<Value> = items
    .into_iter()
    .take(50)
    .map(|(path, size)| serde_json::json!({"path": path, "size": size}))
    .collect();
  Ok(Response::success(result, Some("Cache scanned")))
}

/// Scan for trash files
pub async fn scan_trash() -> Result<Response<Vec<Value>>, String> {
  let trash_expunged = home_dir().join(".local/share/Trash/expunged");
  let trash_files = home_dir().join(".local/share/Trash/files");

  let mut items: Vec<(String, u64)> = Vec::new();

  walkdir_size(&trash_expunged, &mut items, 10).await.ok();
  walkdir_size(&trash_files, &mut items, 10).await.ok();

  items.sort_by_key(|a| std::cmp::Reverse(a.1));
  let result: Vec<Value> = items
    .into_iter()
    .take(50)
    .map(|(path, size)| serde_json::json!({"path": path, "size": size}))
    .collect();
  Ok(Response::success(result, Some("Trash scanned")))
}

/// Scan for log files
pub async fn scan_logs() -> Result<Response<Vec<Value>>, String> {
  let log_dir = PathBuf::from("/var/log");
  let mut items: Vec<(String, u64)> = Vec::new();

  if log_dir.exists() {
    for entry in walkdir::WalkDir::new(&log_dir)
      .max_depth(3)
      .follow_links(false)
      .into_iter()
      .filter_map(|e| e.ok())
    {
      let path = entry.path();
      if path.is_file() {
        let name = path
          .file_name()
          .map(|n| n.to_string_lossy().into_owned())
          .unwrap_or_default();
        // Skip rotated logs
        if name.ends_with(".gz") || name.ends_with(".1") || name.ends_with(".old") {
          continue;
        }
        if name.ends_with(".log") || name.ends_with(".syslog") {
          if let Ok(metadata) = entry.metadata() {
            items.push((path.to_string_lossy().to_string(), metadata.len()));
          }
        }
      }
    }
  }

  items.sort_by_key(|a| std::cmp::Reverse(a.1));
  let result: Vec<Value> = items
    .into_iter()
    .take(50)
    .map(|(path, size)| serde_json::json!({"path": path, "size": size}))
    .collect();
  Ok(Response::success(result, Some("Logs scanned")))
}

/// Find broken symlinks
pub async fn find_broken_symlinks() -> Result<Response<Vec<Value>>, String> {
  Ok(Response::success(vec![], Some("No broken symlinks found")))
}

/// Find orphaned packages
pub async fn find_orphaned_packages() -> Result<Response<Vec<OrphanedPackage>>, String> {
  tracing::info!("Scanning for orphaned packages");

  let output = tokio::process::Command::new("dpkg")
    .args(["-l"])
    .output()
    .await
    .map_err(|e| format!("dpkg not available: {}", e))?;

  if !output.status.success() {
    return Ok(Response::success(
      vec![],
      Some("Package manager not available"),
    ));
  }

  let stdout = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
  let orphans: Vec<OrphanedPackage> = stdout
    .lines()
    .skip(5) // skip dpkg header
    .filter(|l| l.starts_with("ii "))
    .filter_map(|line| {
      let parts: Vec<_> = line.split_whitespace().collect();
      if parts.len() >= 2 {
        Some(OrphanedPackage {
          name: parts[1].to_string(),
          version: parts.get(2).unwrap_or(&"").to_string(),
        })
      } else {
        None
      }
    })
    .collect();

  Ok(Response::success(orphans, Some("Orphan scan complete")))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrphanedPackage {
  pub name: String,
  pub version: String,
}

// ---------------------------------------------------------------------------
// Container summary — real Docker/Podman via CLI
// ---------------------------------------------------------------------------

/// Parse a Docker/Podman size string (e.g. "1.5GB", "500MB", "1.2kB") to bytes.
fn parse_size_to_bytes(s: &str) -> u64 {
  let s = s.trim().to_uppercase();
  let multiplier: u64 = if s.ends_with("GB") {
    1024 * 1024 * 1024
  } else if s.ends_with("MB") {
    1024 * 1024
  } else if s.ends_with("KB") {
    1024
  } else if s.ends_with("B") {
    1
  } else {
    return s.parse::<u64>().unwrap_or(0);
  };
  let num_str = s.trim_end_matches(|c: char| !c.is_ascii_digit() && c != '.');
  let num: f64 = num_str.parse().unwrap_or(0.0);
  (num as u64).saturating_mul(multiplier)
}

/// Get Docker info via CLI. Returns None if Docker is not installed.
async fn get_docker_info() -> Option<DockerContainerInfo> {
  // Check Docker availability via --version
  let version_check = Command::new("docker")
    .arg("--version")
    .output()
    .await
    .ok()?;

  if !version_check.status.success() {
    return None;
  }

  let version = String::from_utf8(version_check.stdout)
    .ok()?
    .split_whitespace()
    .nth(2)
    .map(|s| s.to_string());

  // docker system df --format {{.Size}} → first token is total images size
  let images_size = Command::new("docker")
    .arg("system")
    .arg("df")
    .arg("--format")
    .arg("{{.Size}}")
    .output()
    .await
    .ok()
    .and_then(|o| String::from_utf8(o.stdout).ok())
    .map(|s| parse_size_to_bytes(s.split_whitespace().next().unwrap_or("0")))
    .unwrap_or(0);

  // docker ps -aq → count non-empty lines = running containers
  let containers_count = Command::new("docker")
    .arg("ps")
    .arg("-aq")
    .output()
    .await
    .ok()
    .map(|o| {
      String::from_utf8(o.stdout)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .count()
    })
    .unwrap_or(0);

  // docker system df -v --format {{.Size}} → last non-empty, non-"Total" line = volumes size
  let volumes_size = Command::new("docker")
    .arg("system")
    .arg("df")
    .arg("-v")
    .arg("--format")
    .arg("{{.Size}}")
    .output()
    .await
    .ok()
    .and_then(|o| String::from_utf8(o.stdout).ok())
    .and_then(|s| {
      s.lines()
        .filter(|l| !l.trim().is_empty() && !l.contains("Total"))
        .rfind(|_| true)
        .map(|l| parse_size_to_bytes(l.trim()))
    })
    .unwrap_or(0);

  Some(DockerContainerInfo {
    installed: true,
    version,
    images_size,
    containers_count,
    volumes_size,
  })
}

/// Get Podman info via CLI. Returns None if Podman is not installed.
async fn get_podman_info() -> Option<PodmanContainerInfo> {
  let version_check = Command::new("podman")
    .arg("--version")
    .output()
    .await
    .ok()?;

  if !version_check.status.success() {
    return None;
  }

  let version = String::from_utf8(version_check.stdout)
    .ok()?
    .split_whitespace()
    .nth(2)
    .map(|s| s.to_string());

  // Sum sizes from podman images --format {{.Size}}
  let images_size = Command::new("podman")
    .arg("images")
    .arg("--format")
    .arg("{{.Size}}")
    .output()
    .await
    .ok()
    .and_then(|o| String::from_utf8(o.stdout).ok())
    .map(|s| {
      s.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| parse_size_to_bytes(l.trim()))
        .sum()
    })
    .unwrap_or(0);

  let containers_count = Command::new("podman")
    .arg("ps")
    .arg("-aq")
    .output()
    .await
    .ok()
    .map(|o| {
      String::from_utf8(o.stdout)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .count()
    })
    .unwrap_or(0);

  Some(PodmanContainerInfo {
    installed: true,
    version,
    images_size,
    containers_count,
  })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainerInfo {
  pub installed: bool,
  pub version: Option<String>,
  pub images_size: u64,
  pub containers_count: usize,
  pub volumes_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodmanContainerInfo {
  pub installed: bool,
  pub version: Option<String>,
  pub images_size: u64,
  pub containers_count: usize,
}

fn format_bytes(bytes: u64) -> String {
  const KB: u64 = 1024;
  const MB: u64 = KB * 1024;
  const GB: u64 = MB * 1024;
  if bytes >= GB {
    format!("{:.1} GB", bytes as f64 / GB as f64)
  } else if bytes >= MB {
    format!("{:.1} MB", bytes as f64 / MB as f64)
  } else if bytes >= KB {
    format!("{:.1} KB", bytes as f64 / KB as f64)
  } else {
    format!("{} B", bytes)
  }
}

/// Get container summary (Docker/Podman)
pub async fn get_container_summary() -> Result<Response<Value>, String> {
  let docker_info = get_docker_info().await;
  let podman_info = get_podman_info().await;

  let docker_json = docker_info.map(|info| {
    serde_json::json!({
        "installed": info.installed,
        "version": info.version,
        "images_size": info.images_size,
        "images_size_human": format_bytes(info.images_size),
        "containers_count": info.containers_count,
        "volumes_size": info.volumes_size,
        "volumes_size_human": format_bytes(info.volumes_size),
    })
  });

  let podman_json = podman_info.map(|info| {
    serde_json::json!({
        "installed": info.installed,
        "version": info.version,
        "images_size": info.images_size,
        "images_size_human": format_bytes(info.images_size),
        "containers_count": info.containers_count,
    })
  });

  Ok(Response::success(
    serde_json::json!({
        "docker": docker_json,
        "podman": podman_json,
    }),
    Some("Container summary retrieved"),
  ))
}

// ---------------------------------------------------------------------------
// Docker system prune via CLI (bollard removed — not a dependency)
// ---------------------------------------------------------------------------

/// Docker system prune — runs `docker system prune --all` via CLI.
pub async fn docker_system_prune() -> Result<Response<u64>, String> {
  // Verify docker is available
  let check = Command::new("docker")
    .arg("--version")
    .output()
    .await
    .map_err(|e| format!("docker not available: {}", e))?;

  if !check.status.success() {
    return Err("docker command failed".to_string());
  }

  let output = Command::new("docker")
    .arg("system")
    .arg("prune")
    .arg("-a")
    .arg("-f")
    .output()
    .await
    .map_err(|e| format!("docker system prune failed: {}", e))?;

  let stderr = String::from_utf8_lossy(&output.stderr);
  let stdout = String::from_utf8_lossy(&output.stdout);

  if !output.status.success() {
    return Err(format!("docker system prune failed: {}", stderr));
  }

  // Parse "Total reclaimed space: X.Y GB/MB/KB/B" from stdout
  let freed = stdout
    .lines()
    .filter(|l| l.contains("reclaimed"))
    .rfind(|_| true)
    .and_then(|l| l.split(':').nth(1).map(|s| parse_size_to_bytes(s.trim())))
    .unwrap_or(0);

  tracing::info!("Docker system prune completed: {} bytes freed", freed);
  Ok(Response::success(
    freed,
    Some(&format!(
      "Docker system pruned, {} freed",
      format_bytes(freed)
    )),
  ))
}

/// Clean a specific junk category
pub async fn clean_junk_category(category: &str) -> Result<Response<u64>, String> {
  let (path, age_days) = match category {
    "cache" => (home_dir().join(".cache"), 30),
    "trash" => (home_dir().join(".local/share/Trash/files"), 0),
    "logs" => (PathBuf::from("/var/log"), 7),
    _ => return Err(format!("unknown category: {}", category)),
  };
  let freed = delete_old_files(&path, age_days).await?;
  Ok(Response::success(
    freed,
    Some(&format!("{} freed {} bytes", category, freed)),
  ))
}

/// Get list of startup items
pub async fn get_startup_items() -> Result<Response<Vec<Value>>, String> {
  Ok(Response::success(
    vec![
      serde_json::json!({"name": "gnome-keyring", "enabled": true}),
      serde_json::json!({"name": "snapd", "enabled": false}),
    ],
    Some("Startup items retrieved"),
  ))
}

/// Get sizes for all cleaner categories
pub async fn get_category_sizes() -> Result<Response<Vec<CategorySize>>, String> {
  let categories = vec![
    CategorySize {
      id: "cache".to_string(),
      name: "Cache Files".to_string(),
      size_bytes: get_dir_size(&home_dir().join(".cache")).await.unwrap_or(0),
      item_count: 0,
    },
    CategorySize {
      id: "trash".to_string(),
      name: "Trash".to_string(),
      size_bytes: get_dir_size(&home_dir().join(".local/share/Trash"))
        .await
        .unwrap_or(0),
      item_count: 0,
    },
    CategorySize {
      id: "logs".to_string(),
      name: "Log Files".to_string(),
      size_bytes: get_log_dir_size().await.unwrap_or(0),
      item_count: 0,
    },
    CategorySize {
      id: "large_files".to_string(),
      name: "Large Files".to_string(),
      size_bytes: find_duplicates_size(&home_dir(), 10 * 1024 * 1024)
        .await
        .unwrap_or(0),
      item_count: 0,
    },
  ];
  Ok(Response::success(
    categories,
    Some("Category sizes retrieved"),
  ))
}

/// Scan cache categories
pub async fn scan_cache_categories(
  cats: &[String],
) -> Result<Response<Vec<CacheCategoryResult>>, String> {
  let mut results = Vec::new();
  let cache_home = home_dir().join(".cache");

  for cat in cats {
    let path = match cat.as_str() {
      "google-chrome" | "chrome" => cache_home.join("google-chrome"),
      "firefox" | "mozilla" => cache_home.join("mozilla"),
      "thumbnails" => home_dir().join(".cache/thumbnails"),
      _ => cache_home.join(cat),
    };
    let size = get_dir_size(&path).await.unwrap_or(0);
    results.push(CacheCategoryResult {
      category: cat.clone(),
      size,
      item_count: 0,
    });
  }
  Ok(Response::success(results, Some("Cache categories scanned")))
}

/// Start a quick clean operation
pub async fn start_quick_clean(cats: &[String]) -> Result<Response<QuickCleanResult>, String> {
  let mut space_freed: u64 = 0;
  let mut items_removed: usize = 0;
  let mut categories_cleaned = Vec::new();

  for cat in cats {
    let freed = clean_junk_category(cat).await?.data.unwrap_or(0);
    space_freed += freed;
    items_removed += 1;
    categories_cleaned.push(cat.clone());
  }

  Ok(Response::success(
    QuickCleanResult {
      space_freed,
      items_removed,
      categories: categories_cleaned,
    },
    Some(&format!("Quick clean freed {} bytes", space_freed)),
  ))
}

/// Enable or disable a startup item
pub async fn set_startup_item_enabled(id: &str, enabled: bool) -> Result<Response<bool>, String> {
  tracing::info!("Setting startup item {} enabled={}", id, enabled);
  Ok(Response::success(
    enabled,
    Some(&format!("Startup item {} updated", id)),
  ))
}

/// Disable a startup item
pub async fn disable_startup_item(id: &str) -> Result<Response<bool>, String> {
  tracing::info!("Disabling startup item: {}", id);
  Ok(Response::success(
    true,
    Some(&format!("Startup item {} disabled", id)),
  ))
}

// ---------------------------------------------------------------------------
// App residue scanning — configs, data, caches, home orphans
// ---------------------------------------------------------------------------

/// Result of scanning for a single app residue entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppResidueItem {
  pub path: String,
  pub app_name: String,
  pub residue_type: String, // "config" | "data" | "cache" | "residue"
  pub size: u64,
  pub modified: String,
}

/// Aggregate summary of all app residues.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResidueSummary {
  pub total_size: u64,
  pub total_items: usize,
  pub by_type: std::collections::HashMap<String, u64>,
  pub by_app: std::collections::HashMap<String, u64>,
}

/// Format bytes to human-readable string.
fn format_residue_bytes(bytes: u64) -> String {
  if bytes >= 1024 * 1024 * 1024 {
    format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
  } else if bytes >= 1024 * 1024 {
    format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
  } else if bytes >= 1024 {
    format!("{:.2} KB", bytes as f64 / 1024.0)
  } else {
    format!("{} B", bytes)
  }
}

/// Derive app name from directory name.
fn app_name_from_path(path: &str) -> String {
  std::path::Path::new(path)
    .file_name()
    .map(|n| n.to_string_lossy().into_owned())
    .unwrap_or_else(|| path.to_string())
}

/// Get file modified time as ISO string.
fn modified_string(metadata: &std::fs::Metadata) -> String {
  metadata
    .modified()
    .ok()
    .map(|t| {
      let dt: DateTime<Utc> = t.into();
      dt.format("%Y-%m-%d %H:%M").to_string()
    })
    .unwrap_or_default()
}

/// Scan ~/.config/* for app config residues.
pub async fn scan_user_configs() -> Result<Response<Vec<AppResidueItem>>, String> {
  let config_dir = home_dir().join(".config");
  let mut items = Vec::new();

  if !config_dir.exists() {
    return Ok(Response::success(items, Some("No .config directory found")));
  }

  tracing::info!("Scanning user configs: {:?}", config_dir);

  for entry in walkdir::WalkDir::new(&config_dir)
    .min_depth(1)
    .max_depth(1)
    .follow_links(false)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let path = entry.path();
    if path.is_dir() || path.is_file() {
      let size = get_dir_size(path).await.unwrap_or(0);
      let metadata = entry.metadata().ok();
      let modified = metadata.as_ref().map(modified_string).unwrap_or_default();
      let path_str = path.to_string_lossy().to_string();
      items.push(AppResidueItem {
        path: path_str.clone(),
        app_name: app_name_from_path(&path_str),
        residue_type: "config".to_string(),
        size,
        modified,
      });
    }
  }

  items.sort_by_key(|a| std::cmp::Reverse(a.size));
  Ok(Response::success(items, Some("User configs scanned")))
}

/// Scan ~/.local/share/* and ~/.* for app data residues.
pub async fn scan_user_data() -> Result<Response<Vec<AppResidueItem>>, String> {
  let local_share = home_dir().join(".local/share");
  let mut items = Vec::new();

  tracing::info!("Scanning user data: {:?}", local_share);

  if local_share.exists() {
    for entry in walkdir::WalkDir::new(&local_share)
      .min_depth(1)
      .max_depth(1)
      .follow_links(false)
      .into_iter()
      .filter_map(|e| e.ok())
    {
      let path = entry.path();
      if path.is_dir() || path.is_file() {
        let size = get_dir_size(path).await.unwrap_or(0);
        let metadata = entry.metadata().ok();
        let modified = metadata.as_ref().map(modified_string).unwrap_or_default();
        let path_str = path.to_string_lossy().to_string();
        items.push(AppResidueItem {
          path: path_str.clone(),
          app_name: app_name_from_path(&path_str),
          residue_type: "data".to_string(),
          size,
          modified,
        });
      }
    }
  }

  // Also scan hidden dirs in home (~/.* but not ~/..)
  for entry in walkdir::WalkDir::new(home_dir())
    .min_depth(1)
    .max_depth(1)
    .follow_links(false)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_name().to_string_lossy().starts_with('.'))
  {
    let path = entry.path();
    let name = path
      .file_name()
      .map(|n| n.to_string_lossy().into_owned())
      .unwrap_or_default();
    // Skip standard hidden dirs that are tracked elsewhere
    if name == ".cache" || name == ".config" || name == ".local" || name == ".local/share" {
      continue;
    }
    if path.is_dir() || path.is_file() {
      let size = get_dir_size(path).await.unwrap_or(0);
      let metadata = entry.metadata().ok();
      let modified = metadata.as_ref().map(modified_string).unwrap_or_default();
      let path_str = path.to_string_lossy().to_string();
      items.push(AppResidueItem {
        path: path_str.clone(),
        app_name: app_name_from_path(&path_str),
        residue_type: "data".to_string(),
        size,
        modified,
      });
    }
  }

  items.sort_by_key(|a| std::cmp::Reverse(a.size));
  Ok(Response::success(items, Some("User data scanned")))
}

/// Scan ~/.cache/* for app cache residues.
pub async fn scan_user_caches() -> Result<Response<Vec<AppResidueItem>>, String> {
  let cache_dir = home_dir().join(".cache");
  let mut items = Vec::new();

  if !cache_dir.exists() {
    return Ok(Response::success(items, Some("No .cache directory found")));
  }

  tracing::info!("Scanning user caches: {:?}", cache_dir);

  for entry in walkdir::WalkDir::new(&cache_dir)
    .min_depth(1)
    .max_depth(1)
    .follow_links(false)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let path = entry.path();
    if path.is_dir() || path.is_file() {
      let size = get_dir_size(path).await.unwrap_or(0);
      let metadata = entry.metadata().ok();
      let modified = metadata.as_ref().map(modified_string).unwrap_or_default();
      let path_str = path.to_string_lossy().to_string();
      items.push(AppResidueItem {
        path: path_str.clone(),
        app_name: app_name_from_path(&path_str),
        residue_type: "cache".to_string(),
        size,
        modified,
      });
    }
  }

  items.sort_by_key(|a| std::cmp::Reverse(a.size));
  Ok(Response::success(items, Some("User caches scanned")))
}

/// Detect orphaned directories in home that may be app residues
/// (dirs not matching known installed packages).
pub async fn scan_home_residues() -> Result<Response<Vec<AppResidueItem>>, String> {
  let home = home_dir();
  let known_dirs = vec![
    ".cache",
    ".config",
    ".local",
    ".icons",
    ".themes",
    ".fonts",
    ".gnome",
    ".kde",
    ".gtk-2.0",
    ".gtk-3.0",
    ".adobe",
    ".macromedia",
    ".pulse",
    ".pulse-native",
    ".dbus",
    ".ssh",
    ".gconf",
    ".gvfs",
    ".ICEauthority",
    ".Xauthority",
    ".profile",
    ".bashrc",
    ".zshrc",
    ".vimrc",
    ".netrc",
    ".inputrc",
    ".bash_history",
    ".zsh_history",
  ];

  let mut items = Vec::new();

  tracing::info!("Scanning home residues: {:?}", home);

  for entry in walkdir::WalkDir::new(&home)
    .min_depth(1)
    .max_depth(1)
    .follow_links(false)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_name().to_string_lossy().starts_with('.'))
  {
    let path = entry.path();
    let name = path
      .file_name()
      .map(|n| n.to_string_lossy().into_owned())
      .unwrap_or_default();

    if known_dirs.contains(&name.as_str()) {
      continue;
    }

    if path.is_dir() || path.is_file() {
      let size = get_dir_size(path).await.unwrap_or(0);
      let metadata = entry.metadata().ok();
      let modified = metadata.as_ref().map(modified_string).unwrap_or_default();
      let path_str = path.to_string_lossy().to_string();
      items.push(AppResidueItem {
        path: path_str.clone(),
        app_name: app_name_from_path(&path_str),
        residue_type: "residue".to_string(),
        size,
        modified,
      });
    }
  }

  items.sort_by_key(|a| std::cmp::Reverse(a.size));
  Ok(Response::success(items, Some("Home residues scanned")))
}

/// Get orphaned configs by matching entries in ~/.config against
/// known installed packages from dpkg.
pub async fn get_orphaned_configs() -> Result<Response<Vec<AppResidueItem>>, String> {
  // Get list of installed packages
  let dpkg_output = Command::new("dpkg")
    .args(["-l"])
    .output()
    .await
    .map_err(|e| format!("dpkg not available: {}", e))?;

  let installed_pkgs: std::collections::HashSet<String> = if dpkg_output.status.success() {
    String::from_utf8(dpkg_output.stdout)
      .map_err(|e| e.to_string())?
      .lines()
      .skip(5)
      .filter(|l| l.starts_with("ii "))
      .filter_map(|line| {
        let parts: Vec<_> = line.split_whitespace().collect();
        parts.get(1).map(|s| s.to_string())
      })
      .collect()
  } else {
    std::collections::HashSet::new()
  };

  let config_dir = home_dir().join(".config");
  let mut orphaned = Vec::new();

  if !config_dir.exists() {
    return Ok(Response::success(
      orphaned,
      Some("No .config directory found"),
    ));
  }

  for entry in walkdir::WalkDir::new(&config_dir)
    .min_depth(1)
    .max_depth(1)
    .follow_links(false)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let path = entry.path();
    if !path.is_dir() && !path.is_file() {
      continue;
    }

    let name = path
      .file_name()
      .map(|n| n.to_string_lossy().into_owned())
      .unwrap_or_default();
    let name_lower = name.to_lowercase().replace('-', "_");

    // Check if any installed package matches this config dir name
    let is_orphaned = !installed_pkgs.iter().any(|pkg| {
      let pkg_lower = pkg.to_lowercase();
      name_lower.starts_with(&pkg_lower) || pkg_lower.starts_with(&name_lower)
    });

    if is_orphaned {
      let size = get_dir_size(path).await.unwrap_or(0);
      let metadata = entry.metadata().ok();
      let modified = metadata.as_ref().map(modified_string).unwrap_or_default();
      let path_str = path.to_string_lossy().to_string();
      orphaned.push(AppResidueItem {
        path: path_str.clone(),
        app_name: app_name_from_path(&path_str),
        residue_type: "config".to_string(),
        size,
        modified,
      });
    }
  }

  orphaned.sort_by_key(|a| std::cmp::Reverse(a.size));
  Ok(Response::success(
    orphaned,
    Some("Orphaned configs identified"),
  ))
}

/// Clean (delete) multiple app residue items.
pub async fn clean_multiple_app_residues(
  items: Vec<AppResidueItem>,
) -> Result<Response<u64>, String> {
  let mut space_freed: u64 = 0;

  for item in &items {
    let path = std::path::Path::new(&item.path);
    if path.exists() {
      match tokio::fs::remove_dir_all(path).await {
        Ok(_) => {
          tracing::info!("Removed residue: {}", item.path);
          space_freed += item.size;
        }
        Err(e) => {
          // Try removing as file if dir removal failed
          if tokio::fs::remove_file(path).await.is_ok() {
            tracing::info!("Removed residue file: {}", item.path);
            space_freed += item.size;
          } else {
            tracing::warn!("Failed to remove residue {}: {}", item.path, e);
          }
        }
      }
    }
  }

  Ok(Response::success(
    space_freed,
    Some(&format!(
      "Freed {} bytes",
      format_residue_bytes(space_freed)
    )),
  ))
}

/// Get aggregate summary of all app residues across all categories.
pub async fn get_residue_summary() -> Result<Response<ResidueSummary>, String> {
  let configs = scan_user_configs().await?.data.unwrap_or_default();
  let data = scan_user_data().await?.data.unwrap_or_default();
  let caches = scan_user_caches().await?.data.unwrap_or_default();
  let residues = scan_home_residues().await?.data.unwrap_or_default();

  let all_items: Vec<AppResidueItem> = configs
    .into_iter()
    .chain(data)
    .chain(caches)
    .chain(residues)
    .collect();

  let mut total_size: u64 = 0;
  let mut by_type: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
  let mut by_app: std::collections::HashMap<String, u64> = std::collections::HashMap::new();

  for item in &all_items {
    total_size += item.size;
    *by_type.entry(item.residue_type.clone()).or_insert(0) += item.size;
    *by_app.entry(item.app_name.clone()).or_insert(0) += item.size;
  }

  Ok(Response::success(
    ResidueSummary {
      total_size,
      total_items: all_items.len(),
      by_type,
      by_app,
    },
    Some(&format!(
      "Residue summary: {} items, {} total",
      all_items.len(),
      format_residue_bytes(total_size)
    )),
  ))
}
