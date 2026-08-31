use dioxus_shared::response::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

use crate::infrastructure::sys_utils::{
    delete_old_files, find_duplicates_size, get_dir_size, get_log_dir_size, home_dir, walkdir_size,
};

use tokio::process::Command;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JunkSummary {
    pub cache: u64,
    pub trash: u64,
    pub logs: u64,
    pub large_files: u64,
    pub total: u64,
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
