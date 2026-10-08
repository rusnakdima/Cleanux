//! Query handlers — all read-only query operations from handler files.
//!
//! This module aggregates all query functions from the original handler files.

use crate::error::AppError;
use crate::response::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============================================================================
// Automation Query Handlers
// ============================================================================

use crate::global_state::routine_service;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAction {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
}

/// List all available quick actions.
pub fn list_quick_actions() -> Response<Vec<QuickAction>> {
    let actions = vec![
        QuickAction {
            id: "clean_cache".to_string(),
            name: "Clean Cache".to_string(),
            description: "Remove temporary cache files".to_string(),
            icon: "delete_sweep".to_string(),
        },
        QuickAction {
            id: "optimize_memory".to_string(),
            name: "Optimize Memory".to_string(),
            description: "Drop filesystem caches to free RAM".to_string(),
            icon: "memory".to_string(),
        },
        QuickAction {
            id: "clean_logs".to_string(),
            name: "Clean Logs".to_string(),
            description: "Remove old rotated log files".to_string(),
            icon: "description".to_string(),
        },
        QuickAction {
            id: "docker_prune".to_string(),
            name: "Docker Prune".to_string(),
            description: "Remove unused Docker images and containers".to_string(),
            icon: "inventory_2".to_string(),
        },
    ];
    Response::success(actions, Some("Quick actions retrieved"))
}

/// Get all automation recipes.
pub async fn get_all_recipes() -> Result<Response<Vec<Value>>, AppError> {
    let svc = routine_service();
    let recipes = svc.get_routines()?;
    let value: Vec<Value> = recipes.into_iter().map(serde_json::to_value).filter_map(|r| r.ok()).collect();
    Ok(Response::success(value, Some("Recipes retrieved")))
}

/// Generate a cleaning report from recent history.
pub async fn generate_cleaning_report() -> Result<Response<Value>, AppError> {
    use crate::global_state::get_history_entries;
    let history = get_history_entries();

    let total_cleaned: u64 = history.iter()
        .map(|e| e.space_reclaimed)
        .sum();

    let report = serde_json::json!({
        "total_runs": history.len(),
        "total_bytes_cleaned": total_cleaned,
        "entries": history,
    });

    Ok(Response::success(report, Some("Cleaning report generated")))
}

/// Export cleaning report as formatted text for file save.
pub async fn export_cleaning_report() -> Result<Response<String>, AppError> {
    use crate::global_state::get_history_entries;
    let history = get_history_entries();

    let total_cleaned: u64 = history.iter()
        .map(|e| e.space_reclaimed)
        .sum();

    let mut report = String::new();
    report.push_str("===========================================\n");
    report.push_str("           CLEANUX CLEANING REPORT\n");
    report.push_str("===========================================\n\n");
    report.push_str(&format!("Generated: {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    report.push_str(&format!("Total Cleaning Runs: {}\n", history.len()));
    report.push_str(&format!("Total Space Reclaimed: {}\n\n", format_size(total_cleaned)));

    if history.is_empty() {
        report.push_str("No cleaning history available.\n");
    } else {
        report.push_str("-------------------------------------------");
        report.push_str("CLEANING HISTORY\n");
        report.push_str("-------------------------------------------");
        report.push_str("\n");
        for (i, entry) in history.iter().enumerate() {
            report.push_str(&format!("#{}\n", i + 1));
            report.push_str(&format!("  Recipe: {}\n", entry.recipe_name));
            report.push_str(&format!("  Started: {}\n", entry.started_at.format("%Y-%m-%d %H:%M:%S")));
            if let Some(completed) = entry.completed_at {
                report.push_str(&format!("  Completed: {}\n", completed.format("%Y-%m-%d %H:%M:%S")));
            }
            report.push_str(&format!("  Status: {:?}\n", entry.status));
            report.push_str(&format!("  Space Reclaimed: {}\n", format_size(entry.space_reclaimed)));
            report.push_str(&format!("  Items Affected: {}\n", entry.items_affected));
            if let Some(ref err) = entry.error_message {
                report.push_str(&format!("  Error: {}\n", err));
            }
            report.push_str("\n");
        }
    }

    report.push_str("===========================================\n");
    report.push_str("            END OF REPORT\n");
    report.push_str("===========================================\n");

    Ok(Response::success(report, Some("Report exported")))
}

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Compare two health snapshots by ID.
pub async fn compare_snapshots(id1: &str, id2: &str) -> Result<Response<Value>, AppError> {
    use crate::global_state::health_snapshot_service;

    let svc = health_snapshot_service();
    let s1 = svc.get(id1).await.map_err(|e| AppError::Internal(e.to_string().into()))?;
    let s2 = svc.get(id2).await.map_err(|e| AppError::Internal(e.to_string().into()))?;

    let comparison = serde_json::json!({
        "before": s1,
        "after": s2,
    });

    Ok(Response::success(comparison, Some("Snapshots compared")))
}

/// Scan for application residue (partial uninstall leftovers).
/// Scans ~/.local/share, ~/.config, and ~/.cache for orphaned app directories.
pub async fn scan_app_residue() -> Result<Response<Vec<Value>>, AppError> {
    use crate::infrastructure::sys_utils::walkdir_size;
    use std::collections::HashSet;

    let home = dirs::home_dir().unwrap_or_default();

    // Known application directories to exclude (these are legitimate app data)
    let known_apps: HashSet<&str> = [
        // Desktop environments and system
        "akonadi", "akonadi-copying", "akonadi-icons", "baloo", "dragonplayer", "kactivitymanagerd",
        "kcrash", "kde", "kded5", "keyring", "kmail", "krunner", "kscreen-locker",
        "ksplash", "kwalletd", "kwin", "plasma-desktop", "plasma-workspace", "polkit-1",
        "session", "systemd", "xdg-desktop-portal", "xdg-desktop-portal-kde",
        // File managers
        "nautilus", "nemo", "thunar", "dolphin", "caja", "pcmanfm",
        // Browsers (they manage their own cleanup)
        "google-chrome", "chromium", "firefox", "mozilla", "epiphany",
        // Shells
        "bash", "zsh", "fish", "oh-my-zsh",
        // Version control
        "git", "svn", "hg",
        // Common development tools (legitimate to keep)
        "code", "cursor", "zed", "sublime-text", "atom", "vscodium",
        "gitkraken", "postman", "insomnia", "wget", "curl",
        // Hidden system dirs
        ".Trash", "Trash",
    ]
    .into_iter()
    .collect();

    let mut all_items: Vec<(String, u64)> = Vec::new();

    // Scan ~/.local/share
    let local_share = home.join(".local/share");
    if local_share.exists() {
        if let Ok(entries) = std::fs::read_dir(&local_share) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    if !known_apps.contains(name.as_str()) && !name.starts_with('.') {
                        let size = get_dir_size(&path).await.unwrap_or(0);
                        if size > 0 {
                            all_items.push((path.to_string_lossy().into_owned(), size));
                        }
                    }
                }
            }
        }
    }

    // Scan ~/.config
    let config_dir = home.join(".config");
    if config_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&config_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    if !known_apps.contains(name.as_str()) && !name.starts_with('.') {
                        let size = get_dir_size(&path).await.unwrap_or(0);
                        if size > 0 {
                            all_items.push((path.to_string_lossy().into_owned(), size));
                        }
                    }
                }
            }
        }
    }

    // Scan ~/.cache (only subdirs that look like app residue, not system caches)
    let cache_dir = home.join(".cache");
    if cache_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&cache_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    // Skip system caches, include app caches as potential residue
                    let system_caches = ["mozilla", "google-chrome", "chromium", "fontconfig",
                                         "icon-cache", "thumbnails", "themes", "icons",
                                         "xdg-cache", "pipeline"];
                    if !system_caches.iter().any(|s| *s == name) && !name.starts_with('.') {
                        let size = get_dir_size(&path).await.unwrap_or(0);
                        if size > 0 {
                            all_items.push((path.to_string_lossy().into_owned(), size));
                        }
                    }
                }
            }
        }
    }

    // Sort by size descending
    all_items.sort_by(|a, b| b.1.cmp(&a.1));

    let items: Vec<Value> = all_items
        .into_iter()
        .take(100)
        .map(|(path, size)| {
            serde_json::json!({
                "name": std::path::Path::new(&path)
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.clone()),
                "path": path,
                "size": size,
            })
        })
        .collect();

    let count = items.len();
    Ok(Response::success(items, Some(&format!("Found {} residue entries", count))))
}

// ============================================================================
// Cleaner Query Handlers
// ============================================================================

use crate::infrastructure::sys_utils::{
    delete_old_files, find_duplicates_size, get_dir_size, get_log_dir_size, home_dir, walkdir_size,
};

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
    let log_dir = std::path::PathBuf::from("/var/log");
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

/// Find broken symlinks in common cache/temp directories
pub async fn find_broken_symlinks() -> Result<Response<Vec<Value>>, String> {
    let search_paths = vec![
        dirs::home_dir().map(|h| h.join(".cache")),
        Some(std::path::PathBuf::from("/tmp")),
        dirs::home_dir().map(|h| h.join(".local/share")),
    ];
    let mut broken = Vec::new();
    for path_opt in search_paths.into_iter().flatten() {
        if let Ok(entries) = std::fs::read_dir(&path_opt) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_symlink() {
                    if let Ok(target) = std::fs::read_link(&path) {
                        if !target.exists() {
                            broken.push(serde_json::json!({
                                "path": path.to_string_lossy(),
                                "target": target.to_string_lossy(),
                            }));
                        }
                    }
                }
            }
        }
    }
    Ok(Response::success(broken, Some("Broken symlinks retrieved")))
}

/// Find orphaned packages
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrphanedPackage {
    pub name: String,
    pub version: String,
}

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

// Container summary helpers
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

async fn get_docker_info() -> Option<DockerContainerInfo> {
    let version_check = tokio::process::Command::new("docker")
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

    let images_size = tokio::process::Command::new("docker")
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

    let containers_count = tokio::process::Command::new("docker")
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

    let volumes_size = tokio::process::Command::new("docker")
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

async fn get_podman_info() -> Option<PodmanContainerInfo> {
    let version_check = tokio::process::Command::new("podman")
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

    let images_size = tokio::process::Command::new("podman")
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

    let containers_count = tokio::process::Command::new("podman")
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

pub fn format_bytes(bytes: u64) -> String {
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
    Ok(Response::success(categories, Some("Category sizes retrieved")))
}

/// Scan cache categories
pub async fn scan_cache_categories(cats: &[String]) -> Result<Response<Vec<CacheCategoryResult>>, String> {
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

/// Get list of startup items (reads from systemd user services and XDG autostart)
pub async fn get_startup_items() -> Result<Response<Vec<Value>>, String> {
    let mut items = Vec::new();

    // Read systemd user services
    if let Ok(entries) = std::fs::read_dir(std::path::Path::new("/usr/lib/systemd/user")) {
        for entry in entries.filter_map(|e| e.ok()) {
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() && entry.path().extension().map_or(false, |e| e == "service") {
                    let name = entry.file_name().to_string_lossy().replace(".service", "");
                    let content = std::fs::read_to_string(entry.path()).unwrap_or_default();
                    let enabled = !content.contains("Hidden=true");
                    items.push(serde_json::json!({
                        "name": name,
                        "enabled": enabled,
                        "source": "systemd"
                    }));
                }
            }
        }
    }

    // Read XDG autostart (user-level)
    if let Some(home) = dirs::home_dir() {
        let autostart = home.join(".config").join("autostart");
        if let Ok(entries) = std::fs::read_dir(&autostart) {
            for entry in entries.filter_map(|e| e.ok()) {
                if entry.path().extension().map_or(false, |e| e == "desktop") {
                    let name = entry.file_name().to_string_lossy().replace(".desktop", "");
                    let content = std::fs::read_to_string(entry.path()).unwrap_or_default();
                    let hidden = content.contains("Hidden=true");
                    let enabled = !hidden && content.contains("X-GNOME-Autostart-enabled=true");
                    items.push(serde_json::json!({
                        "name": name,
                        "enabled": enabled,
                        "source": "xdg"
                    }));
                }
            }
        }
    }

    Ok(Response::success(items, Some("Startup items retrieved")))
}

// ============================================================================
// CRUD Query Handlers
// ============================================================================

/// Find a cleaning profile by ID
pub async fn find_cleaning_profile(id: &str) -> Result<Response<Value>, String> {
    use crate::global_state::cleaning_profile_service;
    let svc = cleaning_profile_service();
    let profile = svc.get(id).await.map_err(|e| e.to_string())?;
    match profile {
        Some(p) => {
            let value = serde_json::to_value(p).unwrap_or_else(|_| serde_json::json!({"id": id}));
            Ok(Response::success(value, Some("Profile found")))
        }
        None => Ok(Response::success(
            serde_json::json!({"id": id, "error": "not found"}),
            Some("Profile not found"),
        )),
    }
}

/// Find all cleaning profiles
pub async fn find_all_cleaning_profiles() -> Result<Response<Vec<Value>>, String> {
    use crate::global_state::cleaning_profile_service;
    let svc = cleaning_profile_service();
    let profiles = svc.list().await.map_err(|e| e.to_string())?;
    let values: Vec<Value> = profiles
        .into_iter()
        .map(|p| serde_json::to_value(p).unwrap_or_else(|_| serde_json::json!({})))
        .collect();
    Ok(Response::success(values, Some("Profiles retrieved")))
}

/// Find an automation recipe by ID
pub async fn find_automation_recipe(id: &str) -> Result<Response<Value>, String> {
    use crate::global_state::routine_service;
    let svc = routine_service();
    let recipe = svc.get_routine_by_id(id).map_err(|e| e.to_string())?;
    match recipe {
        Some(r) => {
            let value = serde_json::to_value(r).unwrap_or_else(|_| serde_json::json!({"id": id}));
            Ok(Response::success(value, Some("Recipe found")))
        }
        None => Ok(Response::success(
            serde_json::json!({"id": id, "error": "not found"}),
            Some("Recipe not found"),
        )),
    }
}

/// Find all automation recipes
pub async fn find_all_automation_recipes() -> Result<Response<Vec<Value>>, String> {
    use crate::global_state::routine_service;
    let svc = routine_service();
    let recipes = svc.get_routines().map_err(|e| e.to_string())?;
    let values: Vec<Value> = recipes
        .into_iter()
        .map(|r| serde_json::to_value(r).unwrap_or_else(|_| serde_json::json!({"id": null})))
        .collect();
    Ok(Response::success(values, Some("Recipes retrieved")))
}

/// Find a health snapshot by ID
pub async fn find_health_snapshot(id: &str) -> Result<Response<Value>, String> {
    use crate::global_state::health_snapshot_service;
    let svc = health_snapshot_service();
    let snapshot = svc.get(id).await.map_err(|e| e.to_string())?;
    match snapshot {
        Some(s) => {
            let value = serde_json::to_value(s).unwrap_or_else(|_| serde_json::json!({"id": id}));
            Ok(Response::success(value, Some("Snapshot found")))
        }
        None => Ok(Response::success(
            serde_json::json!({"id": id, "error": "not found"}),
            Some("Snapshot not found"),
        )),
    }
}

/// Find all health snapshot IDs (newest first)
/// Returns only IDs — fetch individual snapshots by ID via find_health_snapshot.
pub async fn find_all_health_snapshots() -> Result<Response<Vec<Value>>, String> {
    use crate::global_state::health_snapshot_service;
    let svc = health_snapshot_service();
    let snapshots = svc.list().await.map_err(|e| e.to_string())?;
    let mut ids: Vec<Value> = snapshots
        .into_iter()
        .map(|s| {
            serde_json::json!({
                "id": s.id,
                "timestamp": s.timestamp.to_rfc3339(),
                "overall_score": s.overall_score,
            })
        })
        .collect();
    // Sort by timestamp descending (newest first)
    ids.sort_by(|a, b| {
        let ts_a = a.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
        let ts_b = b.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
        ts_b.cmp(ts_a)
    });
    Ok(Response::success(ids, Some("Snapshot IDs retrieved")))
}

/// Find execution history entries (newest first)
pub async fn find_execution_history() -> Result<Response<Vec<Value>>, String> {
    use crate::global_state::get_history_entries;
    let entries = get_history_entries();
    let mut values: Vec<Value> = entries
        .into_iter()
        .map(|e| {
            serde_json::to_value(e).unwrap_or_else(|_| serde_json::json!({
                "id": null, "action": "unknown", "timestamp": null
            }))
        })
        .collect();
    values.sort_by(|a, b| {
        let ts_a = a.get("started_at").and_then(|v| v.as_str()).unwrap_or("");
        let ts_b = b.get("started_at").and_then(|v| v.as_str()).unwrap_or("");
        ts_b.cmp(ts_a)
    });
    Ok(Response::success(values, Some("History retrieved")))
}

/// Find cleaning reports (derived from execution history)
pub async fn find_cleaning_reports() -> Result<Response<Vec<Value>>, String> {
    use crate::global_state::get_history_entries;
    let entries = get_history_entries();
    let reports: Vec<Value> = entries
        .iter()
        .filter(|e| matches!(e.status, crate::domain::ExecutionStatus::Completed))
        .map(|e| {
            serde_json::json!({
                "id": e.id,
                "date": e.started_at.to_rfc3339(),
                "items_cleaned": e.items_affected,
                "space_reclaimed": e.space_reclaimed,
                "recipe_name": e.recipe_name,
            })
        })
        .collect();
    Ok(Response::success(reports, Some("Reports retrieved")))
}

// ============================================================================
// Health Query Handlers
// ============================================================================

use crate::infrastructure::sys_utils::{get_cpu_temp, get_gpu_temp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureInfo {
    pub cpu: Option<f64>,
    pub gpu: Option<f64>,
    pub max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub disk_percent: f32,
}

/// Get CPU temperature
pub async fn get_cpu_temperature() -> Result<Response<f64>, String> {
    let temp = get_cpu_temp().await.unwrap_or(f64::NAN);
    Ok(Response::success(temp, Some("CPU temperature retrieved")))
}

/// Get GPU temperature
pub async fn get_gpu_temperature() -> Result<Response<Option<f64>>, String> {
    let gpu_temp = get_gpu_temp().await.ok();
    Ok(Response::success(gpu_temp, Some("GPU temperature retrieved")))
}

/// Get all temperatures
pub async fn get_temperatures() -> Result<Response<TemperatureInfo>, String> {
    let cpu_temp = get_cpu_temp().await.unwrap_or(f64::NAN);
    let gpu_temp = get_gpu_temp().await.ok();
    let max_temp = gpu_temp.map_or(cpu_temp, |g| cpu_temp.max(g));

    Ok(Response::success(
        TemperatureInfo {
            cpu: if cpu_temp.is_nan() {
                None
            } else {
                Some(cpu_temp)
            },
            gpu: gpu_temp,
            max: max_temp,
        },
        Some("Temperatures retrieved"),
    ))
}

/// Get system statistics
pub async fn get_system_stats() -> Result<Response<SystemStats>, String> {
    let sys = sysinfo::System::new_all();
    let cpu_percent = sys.global_cpu_usage();
    let memory_percent = if sys.total_memory() > 0 {
        (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0
    } else {
        0.0
    };

    let disks = sysinfo::Disks::new_with_refreshed_list();
    let disk_percent = if let Some(disk) = disks.iter().find(|d| d.mount_point().eq("/")) {
        let total = disk.total_space() as f32;
        let used = disk.available_space() as f32;
        if total > 0.0 {
            ((total - used) / total) * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    };

    Ok(Response::success(
        SystemStats {
            cpu_percent,
            memory_percent,
            disk_percent,
        },
        Some("System stats retrieved"),
    ))
}

/// Get health history (last 30 days — populated when snapshots are saved)
pub async fn get_health_history() -> Result<Response<Vec<Value>>, String> {
    use crate::global_state::health_snapshot_service;
    let svc = health_snapshot_service();
    let snapshots = svc.list().await.map_err(|e| e.to_string())?;
    // Sort newest first
    let mut values: Vec<Value> = snapshots
        .into_iter()
        .map(|s| serde_json::to_value(s).unwrap_or_else(|_| serde_json::json!({})))
        .collect();
    values.sort_by(|a, b| {
        let ts_a = a.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
        let ts_b = b.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
        ts_b.cmp(ts_a)
    });
    Ok(Response::success(values, Some("Health history retrieved")))
}

/// Get health trends (populated when health snapshots are saved)
pub async fn get_health_trends() -> Result<Response<Value>, String> {
    use crate::global_state::health_snapshot_service;
    let svc = health_snapshot_service();
    let snapshots = svc.list().await.map_err(|e| e.to_string())?;

    if snapshots.len() < 2 {
        return Ok(Response::success(
            serde_json::json!({
                "trend": "insufficient_data",
                "change_percent": 0.0,
                "days_tracked": snapshots.len()
            }),
            Some("Health trends retrieved"),
        ));
    }

    // Sort by timestamp ascending
    let mut sorted = snapshots;
    sorted.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    // Split into two halves for comparison
    let mid = sorted.len() / 2;
    let older: Vec<_> = sorted[..mid].iter().map(|s| s.overall_score).collect();
    let newer: Vec<_> = sorted[mid..].iter().map(|s| s.overall_score).collect();

    let avg_old = older.iter().sum::<f64>() / older.len() as f64;
    let avg_new = newer.iter().sum::<f64>() / newer.len() as f64;

    let change_percent = if avg_old > 0.0 {
        ((avg_new - avg_old) / avg_old) * 100.0
    } else {
        0.0
    };

    let trend = if change_percent > 5.0 {
        "improving"
    } else if change_percent < -5.0 {
        "declining"
    } else {
        "stable"
    };

    Ok(Response::success(
        serde_json::json!({
            "trend": trend,
            "change_percent": change_percent,
            "days_tracked": sorted.len(),
            "avg_score_old": avg_old,
            "avg_score_new": avg_new
        }),
        Some("Health trends retrieved"),
    ))
}

// ============================================================================
// Journal Query Handlers
// ============================================================================

use std::path::PathBuf;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalUsage {
    pub total_bytes: u64,
    pub total_entries: u64,
    pub oldest_timestamp: Option<String>,
    pub newest_timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalSize {
    pub bytes: u64,
    pub human: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotatedLog {
    pub path: String,
    pub size: u64,
    pub size_human: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogrotateConfig {
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargestLogFile {
    pub path: String,
    pub size: u64,
    pub size_human: String,
    pub modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogManagerSummary {
    pub journal_size_bytes: u64,
    pub journal_size_human: String,
    pub var_log_size_bytes: u64,
    pub var_log_size_human: String,
    pub rotated_logs_count: usize,
    pub largest_log_files: Vec<LargestLogFile>,
}

fn journal_parse_size_to_bytes(s: &str) -> u64 {
    let s = s.trim().to_uppercase();
    let multiplier: u64 = if s.ends_with('G') {
        1024 * 1024 * 1024
    } else if s.ends_with('M') {
        1024 * 1024
    } else if s.ends_with('K') {
        1024
    } else {
        return s.parse::<u64>().unwrap_or(0);
    };
    let num_str: String = s.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
    let num: f64 = num_str.parse().unwrap_or(0.0);
    (num as u64).saturating_mul(multiplier)
}

fn journal_format_bytes(bytes: u64) -> String {
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

/// Get journal usage statistics
pub async fn get_journal_usage() -> Result<Response<JournalUsage>, AppError> {
    let entries_output = Command::new("journalctl")
        .args(["--no-pager", "-b", "-n", "1", "--output=short-iso"])
        .output()
        .await;

    let oldest = entries_output
        .as_ref()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout.clone()).ok())
        .and_then(|s| s.lines().next().map(|l| l.to_string()));

    let newest_output = Command::new("journalctl")
        .args(["--no-pager", "--output=short-iso"])
        .output()
        .await;

    let newest = newest_output
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.lines().last().map(|l| l.to_string()));

    let count_output = Command::new("journalctl")
        .args(["--no-pager", "--output=short-iso"])
        .output()
        .await
        .map_err(|e| AppError::Internal(format!("journalctl failed: {}", e)))?;

    let total_entries = String::from_utf8_lossy(&count_output.stdout)
        .lines()
        .count() as u64;

    // Parse disk usage: "Archived and active journals take up X.XM (room for Y.YM)."
    let disk_output = Command::new("journalctl")
        .args(["--disk-usage"])
        .output()
        .await
        .map_err(|e| AppError::Internal(format!("journalctl --disk-usage failed: {}", e)))?;

    let total_bytes = String::from_utf8_lossy(&disk_output.stdout)
        .lines()
        .next()
        .and_then(|line| {
            // Extract "X.XM" before the first space or paren
            let mut multiplier: u64 = 0;
            if line.contains("T") {
                line.split('T').next()?.trim().parse::<f64>().ok()?;
                multiplier = 1024 * 1024 * 1024 * 1024;
            } else if line.contains("G") {
                line.split('G').next()?.trim().parse::<f64>().ok()?;
                multiplier = 1024 * 1024 * 1024;
            } else if line.contains("M") {
                line.split('M').next()?.trim().parse::<f64>().ok()?;
                multiplier = 1024 * 1024;
            } else if line.contains("K") {
                line.split('K').next()?.trim().parse::<f64>().ok()?;
                multiplier = 1024;
            } else if line.contains("B") && !line.contains("KB") && !line.contains("MB") && !line.contains("GB") && !line.contains("TB") {
                line.split('B').next()?.trim().parse::<f64>().ok()?;
                multiplier = 1;
            } else {
                return None;
            };
            // Extract number part
            let num_str = line.split_whitespace().next()?;
            let num: f64 = num_str.parse().ok()?;
            Some((num * multiplier as f64) as u64)
        })
        .unwrap_or(0);

    Ok(Response::success(
        JournalUsage {
            total_bytes,
            total_entries,
            oldest_timestamp: oldest,
            newest_timestamp: newest,
        },
        Some("Journal usage retrieved"),
    ))
}

/// Get journal size on disk
pub async fn get_journal_size() -> Result<Response<JournalSize>, AppError> {
    let output = Command::new("journalctl")
        .args(["--disk-usage"])
        .output()
        .await
        .map_err(|e| AppError::Internal(format!("journalctl disk-usage failed: {}", e)))?;

    if !output.status.success() {
        return Err(AppError::Internal("journalctl --disk-usage failed".to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next().unwrap_or("");
    let size_str = line
        .split(':')
        .nth(1)
        .map(|s| s.trim())
        .unwrap_or("0");

    let bytes = journal_parse_size_to_bytes(size_str);
    Ok(Response::success(
        JournalSize {
            bytes,
            human: size_str.to_string(),
        },
        Some("Journal size retrieved"),
    ))
}

/// Get /var/log usage
pub async fn get_var_log_usage() -> Result<Response<u64>, AppError> {
    let path = PathBuf::from("/var/log");
    if !path.exists() {
        return Ok(Response::success(0u64, Some("/var/log not found")));
    }

    let size = crate::infrastructure::sys_utils::get_dir_size(&path)
        .await
        .unwrap_or(0);

    Ok(Response::success(size, Some("Var log usage retrieved")))
}

/// Get largest log files in /var/log
pub async fn get_largest_log_files(limit: usize) -> Result<Response<Vec<LargestLogFile>>, AppError> {
    let log_dir = PathBuf::from("/var/log");
    let mut files: Vec<(String, u64, String)> = Vec::new();

    if log_dir.exists() {
        for entry in walkdir::WalkDir::new(&log_dir)
            .max_depth(4)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = entry.metadata() {
                    let modified = metadata
                        .modified()
                        .ok()
                        .and_then(|t| {
                            chrono::DateTime::<chrono::Utc>::from(t)
                                .format("%Y-%m-%d %H:%M:%S")
                                .to_string()
                                .into()
                        })
                        .unwrap_or_else(|| "unknown".to_string());

                    files.push((
                        path.to_string_lossy().to_string(),
                        metadata.len(),
                        modified,
                    ));
                }
            }
        }
    }

    files.sort_by_key(|a| std::cmp::Reverse(a.1));
    let result: Vec<LargestLogFile> = files
        .into_iter()
        .take(limit)
        .map(|(path, size, modified)| LargestLogFile {
            path,
            size,
            size_human: journal_format_bytes(size),
            modified,
        })
        .collect();

    Ok(Response::success(result, Some("Largest log files retrieved")))
}

/// Get rotated logs (gz, .1, .old, etc.)
pub async fn get_rotated_logs() -> Result<Response<Vec<RotatedLog>>, AppError> {
    let log_dir = PathBuf::from("/var/log");
    let mut logs: Vec<RotatedLog> = Vec::new();

    if log_dir.exists() {
        for entry in walkdir::WalkDir::new(&log_dir)
            .max_depth(4)
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

                if name.ends_with(".gz")
                    || name.ends_with(".1")
                    || name.ends_with(".old")
                    || name.ends_with(".2")
                    || name.ends_with(".bz2")
                    || name.ends_with(".xz")
                {
                    if let Ok(metadata) = entry.metadata() {
                        let size = metadata.len();
                        logs.push(RotatedLog {
                            path: path.to_string_lossy().to_string(),
                            size,
                            size_human: journal_format_bytes(size),
                        });
                    }
                }
            }
        }
    }

    logs.sort_by_key(|a| std::cmp::Reverse(a.size));
    Ok(Response::success(logs, Some("Rotated logs retrieved")))
}

/// Get total size of rotated logs
pub async fn get_rotated_logs_size() -> Result<Response<u64>, AppError> {
    let logs = get_rotated_logs().await?;
    let total: u64 = logs.data.as_ref().map(|v| v.iter().map(|l| l.size).sum()).unwrap_or(0);
    Ok(Response::success(total, Some("Rotated logs size retrieved")))
}

/// Get logrotate configurations
pub async fn get_logrotate_configs() -> Result<Response<Vec<LogrotateConfig>>, AppError> {
    let mut configs: Vec<LogrotateConfig> = Vec::new();

    let config_dirs = vec![
        "/etc/logrotate.d",
        "/etc/logrotate.conf",
    ];

    for config_path in config_dirs {
        let path = PathBuf::from(config_path);
        if path.exists() {
            if path.is_file() {
                if let Ok(metadata) = path.metadata() {
                    configs.push(LogrotateConfig {
                        path: config_path.to_string(),
                        size: metadata.len(),
                    });
                }
            } else if path.is_dir() {
                if let Ok(entries) = std::fs::read_dir(&path) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let entry_path = entry.path();
                        if entry_path.is_file() {
                            if let Ok(metadata) = entry_path.metadata() {
                                configs.push(LogrotateConfig {
                                    path: entry_path.to_string_lossy().to_string(),
                                    size: metadata.len(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(Response::success(configs, Some("Logrotate configs retrieved")))
}

/// Analyze logrotate configuration for a given path
pub async fn analyze_logrotate(path: &str) -> Result<Response<Value>, AppError> {
    let config_dir = PathBuf::from("/etc/logrotate.d");
    let mut result = serde_json::json!({
        "path": path,
        "configs": Vec::<Value>::new(),
        "has_config": false,
    });

    if config_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&config_dir) {
            let mut configs: Vec<Value> = Vec::new();
            for entry in entries.filter_map(|e| e.ok()) {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    if let Ok(content) = std::fs::read_to_string(&entry_path) {
                        if content.contains(path) {
                            configs.push(serde_json::json!({
                                "config": entry_path.to_string_lossy().to_string(),
                                "content": content,
                            }));
                        }
                    }
                }
            }
            result["has_config"] = serde_json::json!(!configs.is_empty());
            result["configs"] = serde_json::json!(configs);
        }
    }

    Ok(Response::success(result, Some("Logrotate analysis complete")))
}

/// Scan log rotations for a path
pub async fn scan_log_rotations(path: &str) -> Result<Response<Vec<RotatedLog>>, AppError> {
    let target = PathBuf::from(path);
    let parent = target.parent().unwrap_or(&target);
    let basename = target.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();

    let mut rotations: Vec<RotatedLog> = Vec::new();

    if let Ok(entries) = std::fs::read_dir(parent) {
        for entry in entries.filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            let name = entry_path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();

            if name == basename || name.starts_with(&basename) {
                if name.ends_with(".gz")
                    || name.ends_with(".1")
                    || name.ends_with(".old")
                    || name.ends_with(".2")
                    || name.ends_with(".bz2")
                    || name.ends_with(".xz")
                    || name == format!("{}.1", basename)
                    || name == format!("{}.gz", basename)
                {
                    if let Ok(metadata) = entry_path.metadata() {
                        rotations.push(RotatedLog {
                            path: entry_path.to_string_lossy().to_string(),
                            size: metadata.len(),
                            size_human: journal_format_bytes(metadata.len()),
                        });
                    }
                }
            }
        }
    }

    rotations.sort_by_key(|a| std::cmp::Reverse(a.size));
    Ok(Response::success(rotations, Some("Log rotations scanned")))
}

/// Get combined log manager summary
pub async fn get_log_manager_summary() -> Result<Response<LogManagerSummary>, AppError> {
    let journal_size = get_journal_size().await?;
    let var_log_size = get_var_log_usage().await?;
    let largest_logs = get_largest_log_files(10).await?;
    let rotated = get_rotated_logs().await?;

    let var_log_size_val = var_log_size.data.unwrap_or(0);
    let rotated_count = rotated.data.as_ref().map(|v| v.len()).unwrap_or(0);
    let largest_from_response = largest_logs.data.unwrap_or_default();

    Ok(Response::success(
        LogManagerSummary {
            journal_size_bytes: journal_size.data.as_ref().map(|d| d.bytes).unwrap_or(0),
            journal_size_human: journal_size.data.as_ref().map(|d| d.human.clone()).unwrap_or_default(),
            var_log_size_bytes: var_log_size_val,
            var_log_size_human: journal_format_bytes(var_log_size_val),
            rotated_logs_count: rotated_count,
            largest_log_files: largest_from_response,
        },
        Some("Log manager summary retrieved"),
    ))
}

/// Get log summary (simple version)
pub async fn get_log_summary() -> Result<Response<Value>, AppError> {
    let journal_size = get_journal_size().await?;
    let var_log_size = get_var_log_usage().await?;
    let rotated = get_rotated_logs_size().await?;

    let var_log_size_val = var_log_size.data.unwrap_or(0);
    let rotated_size_val = rotated.data.unwrap_or(0);

    Ok(Response::success(
        serde_json::json!({
            "journal_size_bytes": journal_size.data.as_ref().map(|d| d.bytes).unwrap_or(0),
            "journal_size_human": journal_size.data.as_ref().map(|d| d.human.clone()).unwrap_or_default(),
            "var_log_size_bytes": var_log_size_val,
            "var_log_size_human": journal_format_bytes(var_log_size_val),
            "rotated_logs_size_bytes": rotated_size_val,
            "rotated_logs_size_human": journal_format_bytes(rotated_size_val),
        }),
        Some("Log summary retrieved"),
    ))
}

// ============================================================================
// Logging Query Handlers
// ============================================================================

use chrono::Utc;
use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogExport {
    pub format: String,
    pub entries: Vec<LogEntry>,
    pub total_count: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogFilter {
    pub level: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub target: Option<String>,
    pub limit: Option<usize>,
}

static LOG_BUFFER: std::sync::LazyLock<Mutex<VecDeque<LogEntry>>> =
    std::sync::LazyLock::new(|| Mutex::new(VecDeque::new()));

fn record_log(level: &str, message: &str, target: Option<String>) {
    let entry = LogEntry {
        timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        level: level.to_string(),
        message: message.to_string(),
        target,
    };

    if let Ok(mut buffer) = LOG_BUFFER.lock() {
        if buffer.len() >= 1000 {
            buffer.pop_front();
        }
        buffer.push_back(entry);
    }
}

/// Export logs in structured format (JSON)
pub async fn logs_export(filter: LogFilter) -> Result<Response<LogExport>, AppError> {
    let buffer = LOG_BUFFER.lock().map_err(|e| AppError::Lock(e.to_string()))?;

    let entries: Vec<LogEntry> = buffer
        .iter()
        .filter(|e| {
            if let Some(level) = &filter.level {
                if &e.level != level {
                    return false;
                }
            }
            if let Some(since) = &filter.since {
                if e.timestamp < *since {
                    return false;
                }
            }
            if let Some(until) = &filter.until {
                if e.timestamp > *until {
                    return false;
                }
            }
            if let Some(target) = &filter.target {
                if e.target.as_deref() != Some(target) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    let total_count = entries.len();
    let limit = filter.limit.unwrap_or(entries.len());
    let entries: Vec<LogEntry> = entries.into_iter().rev().take(limit).collect();

    Ok(Response::success(
        LogExport {
            format: "json".to_string(),
            entries,
            total_count,
        },
        Some("Logs exported"),
    ))
}

/// Query logs with optional filters
pub async fn logs_query(filter: LogFilter) -> Result<Response<Vec<LogEntry>>, AppError> {
    let buffer = LOG_BUFFER.lock().map_err(|e| AppError::Lock(e.to_string()))?;

    let entries: Vec<LogEntry> = buffer
        .iter()
        .filter(|e| {
            if let Some(level) = &filter.level {
                if !level.split(',').any(|l| e.level == l.trim()) {
                    return false;
                }
            }
            if let Some(since) = &filter.since {
                if e.timestamp < *since {
                    return false;
                }
            }
            if let Some(until) = &filter.until {
                if e.timestamp > *until {
                    return false;
                }
            }
            if let Some(target) = &filter.target {
                if e.target.as_deref() != Some(target) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    let total_count = entries.len();
    let limit = filter.limit.unwrap_or(entries.len());
    let entries: Vec<LogEntry> = entries.into_iter().rev().take(limit).collect();

    Ok(Response::success(entries, Some(&format!("{} entries found", total_count))))
}

// ============================================================================
// Monitoring Query Handlers
// ============================================================================

use crate::global_state::{get_monitoring_snapshot, monitoring_state};

/// Get the latest monitoring snapshot (CPU, memory, temps).
pub async fn get_monitoring_data() -> Result<Response<Option<crate::infrastructure::monitoring_service::MonitoringSnapshot>>, AppError> {
    let snap = get_monitoring_snapshot();
    Ok(Response::success(snap, Some("Monitoring data retrieved")))
}

/// Check whether the monitoring loop is currently active.
pub async fn is_monitoring_active() -> Result<Response<bool>, AppError> {
    let active = monitoring_state().is_running();
    Ok(Response::success(active, None))
}

// ============================================================================
// Opener Query Handlers
// ============================================================================

use std::path::Path;

/// Open a file with the OS default application
pub async fn opener_open(path: &str) -> Result<Response<bool>, AppError> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(AppError::InvalidPath(format!("path does not exist: {}", path.display())));
    }

    #[cfg(target_os = "linux")]
    {
        open::that(path).map_err(|e| AppError::Internal(format!("failed to open: {}", e)))?;
    }
    #[cfg(target_os = "macos")]
    {
        open::that(path).map_err(|e| AppError::Internal(format!("failed to open: {}", e)))?;
    }
    #[cfg(target_os = "windows")]
    {
        open::that(path).map_err(|e| AppError::Internal(format!("failed to open: {}", e)))?;
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        return Err(AppError::ValidationError("unsupported platform".to_string()));
    }

    tracing::info!("Opened: {}", path.display());
    Ok(Response::success(true, Some(&format!("Opened: {}", path.display()))))
}

/// Open a folder in the file manager and optionally select a file
pub async fn opener_open_folder(path: &str) -> Result<Response<bool>, AppError> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(AppError::InvalidPath(format!("path does not exist: {}", path.display())));
    }

    #[cfg(target_os = "linux")]
    {
        let folder = if path.is_file() {
            path.parent().unwrap_or(path)
        } else {
            path
        };
        open::that(folder).map_err(|e| AppError::Internal(format!("failed to open folder: {}", e)))?;
    }
    #[cfg(target_os = "macos")]
    {
        open::that(path).map_err(|e| AppError::Internal(format!("failed to open: {}", e)))?;
    }
    #[cfg(target_os = "windows")]
    {
        open::that(path).map_err(|e| AppError::Internal(format!("failed to open: {}", e)))?;
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        return Err(AppError::ValidationError("unsupported platform".to_string()));
    }

    tracing::info!("Opened folder: {}", path.display());
    Ok(Response::success(true, Some(&format!("Opened folder: {}", path.display()))))
}

/// Open a URL in the default browser
pub async fn opener_open_url(url: &str) -> Result<Response<bool>, AppError> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(AppError::ValidationError(format!("invalid URL: {}", url)));
    }

    open::that(url).map_err(|e| AppError::Internal(format!("failed to open URL: {}", e)))?;
    tracing::info!("Opened URL: {}", url);
    Ok(Response::success(true, Some(&format!("Opened URL: {}", url))))
}

/// Get MIME type for a file path
pub async fn opener_get_mime_type(path: &str) -> Result<Response<String>, AppError> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(AppError::InvalidPath(format!("path does not exist: {}", path.display())));
    }

    let mime = mime_guess::from_path(path)
        .first()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    Ok(Response::success(mime, Some("MIME type retrieved")))
}

// ============================================================================
// Settings Query Handlers
// ============================================================================

use crate::domain::settings::CleanuxSettings;
use crate::infrastructure::settings as settings_storage;
use crate::response::Response as DSResponse;

pub async fn get_settings() -> Result<DSResponse<CleanuxSettings>, AppError> {
    let settings = settings_storage::load_settings();
    Ok(DSResponse::success(settings, None))
}

/// Bridge alias for get_settings
pub async fn settings_get() -> Result<DSResponse<CleanuxSettings>, AppError> {
    get_settings().await
}

// ============================================================================
// Shell Pages Query Handlers
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub id: String,
    pub name: String,
    pub route: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellPagesOutput {
    pub pages: Vec<PageInfo>,
    pub active_page: String,
}

/// List all available shell pages
pub fn shell_pages_list() -> Response<ShellPagesOutput> {
    let pages = vec![
        PageInfo {
            id: "dashboard".to_string(),
            name: "Dashboard".to_string(),
            route: "/".to_string(),
            description: "System overview and health summary".to_string(),
        },
        PageInfo {
            id: "clean".to_string(),
            name: "Clean".to_string(),
            route: "/clean".to_string(),
            description: "Junk scanning and cleaning operations".to_string(),
        },
        PageInfo {
            id: "files".to_string(),
            name: "Files".to_string(),
            route: "/files".to_string(),
            description: "File analysis, duplicates, and large files".to_string(),
        },
        PageInfo {
            id: "power".to_string(),
            name: "Power".to_string(),
            route: "/power".to_string(),
            description: "Power management and thermal controls".to_string(),
        },
        PageInfo {
            id: "settings".to_string(),
            name: "Settings".to_string(),
            route: "/settings".to_string(),
            description: "Application settings and preferences".to_string(),
        },
    ];

    Response::success(
        ShellPagesOutput {
            pages,
            active_page: "dashboard".to_string(),
        },
        Some("Shell pages retrieved"),
    )
}

/// Get active page
pub fn shell_get_active_page() -> Response<String> {
    Response::success("dashboard".to_string(), Some("Active page retrieved"))
}

// ============================================================================
// Storage Query Handlers
// ============================================================================

use std::sync::Arc;
use std::sync::LazyLock;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use uuid::Uuid;
use crate::infrastructure::json_storage::JsonStorage;

const BACKUP_COLLECTION: &str = "backups.json";
const BACKUP_DIR: &str = "backups";

static STORAGE: LazyLock<Arc<JsonStorage>> =
    LazyLock::new(|| Arc::new(JsonStorage::new(crate::storage::app_data_dir("cleanux"))));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryInfo {
    pub path: String,
    pub size: u64,
    pub files: u64,
    pub subdirs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: String,
    pub name: String,
    pub date: String,
    pub size: u64,
    pub path: String,
}

fn load_backup_list() -> Result<Vec<BackupInfo>, String> {
    STORAGE.load(BACKUP_COLLECTION)
}

/// Get directory information
pub async fn scan_directory(path: &str) -> Result<Response<DirectoryInfo>, String> {
    tracing::info!("Scanning directory: {}", path);

    let mut total_size: u64 = 0;
    let mut file_count: u64 = 0;
    let mut dir_count: u64 = 0;

    let entries = walkdir::WalkDir::new(path).into_iter();
    for entry in entries.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            total_size += entry.metadata().map(|m| m.len()).unwrap_or(0);
            file_count += 1;
        } else if entry.file_type().is_dir() {
            dir_count += 1;
        }
    }

    Ok(Response::success(
        DirectoryInfo {
            path: path.to_string(),
            size: total_size,
            files: file_count,
            subdirs: dir_count,
        },
        Some("Directory scanned"),
    ))
}

/// Get directory size
pub async fn get_directory_size(path: &str) -> Result<Response<u64>, String> {
    let mut size: u64 = 0;

    let entries = walkdir::WalkDir::new(path).into_iter();
    for entry in entries.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            size += entry.metadata().map(|m| m.len()).unwrap_or(0);
        }
    }

    Ok(Response::success(size, Some("Directory size calculated")))
}

/// Find empty directories
pub async fn find_empty_directories(path: &str) -> Result<Response<Vec<String>>, String> {
    let mut empty_dirs = Vec::new();

    let entries = walkdir::WalkDir::new(path).into_iter();
    for entry in entries.filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            if let Ok(mut entries_in_dir) = std::fs::read_dir(entry.path()) {
                if entries_in_dir.next().is_none() {
                    empty_dirs.push(entry.path().display().to_string());
                }
            }
        }
    }

    Ok(Response::success(
        empty_dirs,
        Some("Empty directories found"),
    ))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub paths: Vec<String>,
    pub size: u64,
    pub hash: String,
}

/// Find duplicate files
pub async fn find_duplicates(path: &str) -> Result<Response<Vec<DuplicateGroup>>, String> {
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;

    tracing::info!("Finding duplicates in: {}", path);

    let mut files: HashMap<String, Vec<String>> = HashMap::new();
    let mut stack = vec![std::path::PathBuf::from(path)];

    while let Some(current) = stack.pop() {
        let mut entries = tokio::fs::read_dir(&current)
            .await
            .map_err(|e| e.to_string())?;
        while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else {
                match tokio::fs::read(&p).await {
                    Ok(content) => {
                        let hash = format!("{:x}", Sha256::digest(&content));
                        let path_str = p.to_string_lossy().to_string();
                        files.entry(hash).or_default().push(path_str);
                    }
                    Err(_) => continue,
                }
            }
        }
    }

    let duplicates: Vec<DuplicateGroup> = files
        .into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .map(|(hash, paths)| {
            let size = std::fs::metadata(&paths[0]).map(|m| m.len()).unwrap_or(0);
            DuplicateGroup { paths, size, hash }
        })
        .collect();

    Ok(Response::success(
        duplicates,
        Some("Duplicate scan complete"),
    ))
}

/// List all persisted backups, filtering out any whose archive files are missing.
pub async fn list_backups() -> Result<Response<Vec<BackupInfo>>, String> {
    tracing::info!("Listing backups");

    let backups = load_backup_list()?;

    let existing: Vec<BackupInfo> = backups
        .into_iter()
        .filter(|b| Path::new(&b.path).exists())
        .collect();

    Ok(Response::success(existing, Some("Backups retrieved")))
}

/// Scan browser caches — scans real ~/.cache browser profile directories.
pub async fn scan_browser_caches() -> Result<Response<Vec<Value>>, String> {
    let home = home_dir();
    let cache_home = home.join(".cache");

    let browser_paths = vec![
        ("Chrome", cache_home.join("google-chrome")),
        ("Chrome Beta", cache_home.join("google-chrome-beta")),
        ("Chrome Unstable", cache_home.join("google-chrome-unstable")),
        ("Chromium", cache_home.join("chromium")),
        ("Firefox", cache_home.join("mozilla")),
        ("Brave", cache_home.join("BraveSoftware")),
        ("Edge", cache_home.join("microsoft-edge")),
        ("Edge Beta", cache_home.join("microsoft-edge-beta")),
        ("Vivaldi", cache_home.join("vivaldi")),
        ("Opera", cache_home.join("opera")),
        ("Firefox development", cache_home.join("firefox-developer")),
    ];

    let mut caches: Vec<Value> = Vec::new();
    for (browser, path) in browser_paths {
        if path.exists() {
            let size = get_dir_size(&path).await.unwrap_or(0);
            if size > 0 {
                caches.push(serde_json::json!({
                    "browser": browser,
                    "path": path.to_string_lossy(),
                    "size": size,
                }));
            }
        }
    }

    caches.sort_by(|a, b| {
        let sa = a["size"].as_u64().unwrap_or(0);
        let sb = b["size"].as_u64().unwrap_or(0);
        sb.cmp(&sa) // descending
    });

    Ok(Response::success(caches, Some("Browser caches scanned")))
}

/// Scan thumbnail caches — sums all thumbnail subdirectories in ~/.cache/thumbnails.
pub async fn scan_thumbnail_caches() -> Result<Response<u64>, String> {
    let thumb_dir = home_dir().join(".cache/thumbnails");
    let total = if thumb_dir.exists() {
        get_dir_size(&thumb_dir).await.unwrap_or(0)
    } else {
        0
    };
    Ok(Response::success(total, Some(&format!("Thumbnail cache: {}", format_bytes(total)))))
}

// ============================================================================
// System Query Handlers
// ============================================================================

use crate::infrastructure::memory_service::MemoryService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub cached: u64,
    pub buffers: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapInfo {
    pub total: u64,
    pub used: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMemory {
    pub pid: u32,
    pub name: String,
    pub memory_mb: f64,
    pub cpu_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMemoryInfo {
    pub processes: Vec<ProcessMemory>,
    pub total_memory_mb: u64,
    pub used_memory_mb: u64,
}

// Get memory information
pub async fn get_memory_info() -> Result<Response<MemoryInfo>, String> {
    let service = MemoryService::new();
    let info = service.get_memory_info().await.map_err(|e| e.to_string())?;

    Ok(Response::success(
        MemoryInfo {
            total: info.total_kb * 1024,
            used: info.used_kb * 1024,
            available: info.available_kb * 1024,
            cached: info.cached_kb * 1024,
            buffers: info.buffers_kb * 1024,
        },
        Some("Memory info retrieved"),
    ))
}

/// Get swap information
pub async fn get_swap_info() -> Result<Response<SwapInfo>, String> {
    let content = std::fs::read_to_string("/proc/swaps")
        .map_err(|e| format!("Failed to read swaps: {}", e))?;

    let mut total: u64 = 0;
    let mut used: u64 = 0;

    for (idx, line) in content.lines().enumerate() {
        if idx == 0 {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            total += parts[2].parse::<u64>().unwrap_or(0) * 1024;
            used += parts[3].parse::<u64>().unwrap_or(0) * 1024;
        }
    }

    Ok(Response::success(
        SwapInfo { total, used },
        Some("Swap info retrieved"),
    ))
}

/// Get current kernel version
pub async fn get_current_kernel() -> Result<Response<String>, String> {
    let version = std::fs::read_to_string("/proc/version")
        .map_err(|e| format!("Failed to read version: {}", e))?;
    let version = version.trim().to_string();
    Ok(Response::success(version, Some("Kernel version retrieved")))
}

/// Get list of installed kernels — scans /boot for vmlinuz-* files using walkdir.
pub async fn get_installed_kernels() -> Result<Response<Vec<String>>, String> {
    let boot_dir = std::path::Path::new("/boot");
    let mut kernels: Vec<String> = Vec::new();

    for entry in walkdir::WalkDir::new(boot_dir)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let name = entry.file_name().to_string_lossy();
        if name.starts_with("vmlinuz-") {
            kernels.push(name.strip_prefix("vmlinuz-").ok_or_else(|| format!("Unexpected kernel filename format: {}", name))?.to_string());
        }
    }

    kernels.sort();
    Ok(Response::success(kernels, Some("Installed kernels retrieved")))
}

/// Get list of old kernels that can be removed — all installed kernels except the running one.
pub async fn get_old_kernels() -> Result<Response<Vec<String>>, String> {
    // Get running kernel version via uname -r
    let running_kernel = Command::new("uname")
        .arg("-r")
        .output()
        .await
        .map_err(|e| format!("Failed to run uname: {}", e))?;
    let running = String::from_utf8_lossy(&running_kernel.stdout).trim().to_string();

    let boot_dir = std::path::Path::new("/boot");
    let mut old_kernels: Vec<String> = Vec::new();

    for entry in walkdir::WalkDir::new(boot_dir)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let name = entry.file_name().to_string_lossy();
        if name.starts_with("vmlinuz-") {
            let version = name.strip_prefix("vmlinuz-").ok_or_else(|| format!("Unexpected kernel filename format: {}", name))?.to_string();
            if version != running {
                old_kernels.push(version);
            }
        }
    }

    old_kernels.sort();
    Ok(Response::success(old_kernels, Some("Old kernels retrieved")))
}

/// Get battery information from sysfs power_supply.
pub async fn get_battery_info() -> Result<Response<Value>, String> {
    use std::collections::HashMap;

    let supply_base = std::path::Path::new("/sys/class/power_supply");

    // Find the first BAT* entry
    let battery_dir = std::fs::read_dir(supply_base)
        .map_err(|e| format!("Failed to read /sys/class/power_supply: {}", e))?
        .filter_map(|e| e.ok())
        .find(|e| {
            e.file_name().to_string_lossy().starts_with("BAT")
        });

    let Some(battery) = battery_dir else {
        return Ok(Response::success(
            serde_json::json!({ "present": false }),
            Some("No battery found"),
        ));
    };

    let bat_path = battery.path();

    fn read_sysfs(path: &std::path::Path, file: &str) -> Option<String> {
        std::fs::read_to_string(path.join(file)).ok()?.trim().to_string().into()
    }

    let present = read_sysfs(&bat_path, "present").as_ref()
        .map(|v| v == "1").unwrap_or(false);
    let capacity = read_sysfs(&bat_path, "capacity")
        .and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
    let status = read_sysfs(&bat_path, "status")
        .unwrap_or_else(|| "Unknown".to_string());
    let technology = read_sysfs(&bat_path, "technology").unwrap_or_default();
    let manufacturer = read_sysfs(&bat_path, "manufacturer").unwrap_or_default();
    let model_name = read_sysfs(&bat_path, "model_name").unwrap_or_default();
    let serial_number = read_sysfs(&bat_path, "serial_number").unwrap_or_default();

    // Read charge / energy values if available
    let charge_now = read_sysfs(&bat_path, "charge_now")
        .and_then(|v| v.parse::<u64>().ok());
    let energy_now = read_sysfs(&bat_path, "energy_now")
        .and_then(|v| v.parse::<u64>().ok());
    let current_now = read_sysfs(&bat_path, "current_now")
        .and_then(|v| v.parse::<u64>().ok());

    let mut info: HashMap<String, serde_json::Value> = HashMap::new();
    info.insert("present".to_string(), serde_json::json!(present));
    info.insert("capacity".to_string(), serde_json::json!(capacity));
    info.insert("status".to_string(), serde_json::json!(status));
    if !technology.is_empty() {
        info.insert("technology".to_string(), serde_json::json!(technology));
    }
    if !manufacturer.is_empty() {
        info.insert("manufacturer".to_string(), serde_json::json!(manufacturer));
    }
    if !model_name.is_empty() {
        info.insert("model_name".to_string(), serde_json::json!(model_name));
    }
    if !serial_number.is_empty() {
        info.insert("serial_number".to_string(), serde_json::json!(serial_number));
    }
    if let Some(v) = charge_now {
        info.insert("charge_now".to_string(), serde_json::json!(v));
    }
    if let Some(v) = energy_now {
        info.insert("energy_now".to_string(), serde_json::json!(v));
    }
    if let Some(v) = current_now {
        info.insert("current_now".to_string(), serde_json::json!(v));
    }

    Ok(Response::success(
        serde_json::json!(info),
        Some("Battery info retrieved"),
    ))
}

/// Get list of system services — runs `systemctl list-units --type=service --all --no-pager`.
pub async fn get_all_services() -> Result<Response<Vec<Value>>, String> {
    let output = tokio::process::Command::new("systemctl")
        .args(["list-units", "--type=service", "--all", "--no-pager", "--no-legend"])
        .output()
        .await
        .map_err(|e| format!("Failed to run systemctl: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "systemctl failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut services: Vec<Value> = Vec::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // systemctl list-units output: UNIT LOAD ACTIVE SUB DESCRIPTION
        // Example: "firewalld.service loaded active running firewalld - dynamic firewall daemon"
        let parts: Vec<&str> = line.splitn(4, ' ').collect();
        if parts.len() >= 4 {
            let unit = parts[0].trim();
            let load = parts[1].trim();
            let active = parts[2].trim();
            // Description is the rest after the third space
            let desc_start = line[parts[0].len() + parts[1].len() + parts[2].len() + 3..].trim();
            services.push(serde_json::json!({
                "name": unit,
                "load": load,
                "active": active,
                "description": desc_start,
            }));
        }
    }

    Ok(Response::success(services, Some("Services retrieved")))
}

/// Get process memory information
pub async fn get_process_memory() -> Result<Response<ProcessMemoryInfo>, String> {
    let output = Command::new("ps")
        .args(["aux", "--sort=-rss"])
        .output()
        .await
        .map_err(|e| format!("Failed to run ps: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut processes = Vec::new();

    for line in stdout.lines().skip(1).take(20) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 11 {
            let pid = parts[1].parse::<u32>().unwrap_or(0);
            let name = parts[10..].join(" ");
            let rss_kb: u64 = parts[5].parse().unwrap_or(0);
            let memory_mb = rss_kb as f64 / 1024.0;
            let cpu_percent: f32 = parts[2].parse().unwrap_or(0.0);
            processes.push(ProcessMemory {
                pid,
                name,
                memory_mb,
                cpu_percent,
            });
        }
    }

    let service = MemoryService::new();
    let mem_info = service.get_memory_info().await.map_err(|e| e.to_string())?;
    let total_memory_mb = mem_info.total_kb / 1024;
    let used_memory_mb = mem_info.used_kb / 1024;

    Ok(Response::success(
        ProcessMemoryInfo {
            processes,
            total_memory_mb,
            used_memory_mb,
        },
        Some("Process memory info retrieved"),
    ))
}

// ============================================================================
// Allowlist Query Handlers
// ============================================================================

use crate::allowlist::is_allowed_path as allowlist_is_allowed;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistEntry {
    pub path: String,
    pub allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistStatus {
    pub allowed_paths: Vec<String>,
    pub total_count: usize,
}

static ALLOWLIST: LazyLock<parking_lot::RwLock<Vec<String>>> =
    LazyLock::new(|| parking_lot::RwLock::new(Vec::new()));

/// List all paths in the allowlist
pub fn allowlist_list() -> Response<Vec<String>> {
    let list = ALLOWLIST.read();
    Response::success(list.clone(), Some(&format!("{} paths in allowlist", list.len())))
}

/// Check if a path is allowed
pub fn allowlist_check(path: &str) -> Response<bool> {
    let path_buf = Path::new(path);
    let is_allowed = allowlist_is_allowed(path_buf);
    Response::success(is_allowed, Some(if is_allowed { "Path is allowed" } else { "Path is blocked" }))
}

/// Get allowlist status
pub fn allowlist_status() -> Response<AllowlistStatus> {
    let list = ALLOWLIST.read();
    Response::success(
        AllowlistStatus {
            allowed_paths: list.clone(),
            total_count: list.len(),
        },
        Some("Allowlist status retrieved"),
    )
}
