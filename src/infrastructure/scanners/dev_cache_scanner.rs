//! Developer cache scanner — real filesystem operations for npm, pip, Cargo, Go, Maven, Gradle.

use std::path::{Path, PathBuf};
use tokio::fs;

/// Return the user's home directory.
fn home() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
}

/// Total size of a directory (async, recursive).
async fn dir_size(path: &Path) -> u64 {
    if !path.exists() {
        return 0;
    }
    let mut size = 0u64;
    let mut stack = vec![path.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let mut entries = match fs::read_dir(&dir).await {
            Ok(v) => v,
            Err(_) => continue,
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(meta) = entry.metadata().await {
                if meta.is_file() {
                    size += meta.len();
                } else if meta.is_dir() {
                    stack.push(entry.path());
                }
            }
        }
    }
    size
}

/// Remove a directory and all its contents, ignoring errors.
async fn remove_dir_all_if_exists(path: &Path) {
    if path.exists() {
        let _ = fs::remove_dir_all(path).await;
    }
}

/// Remove a file if it exists.
async fn remove_file_if_exists(path: &Path) {
    if path.exists() {
        let _ = fs::remove_file(path).await;
    }
}

// ---------------------------------------------------------------------------
// Individual cleaners
// ---------------------------------------------------------------------------

/// Clean npm cache (`~/.npm` and `npm cache clean --force`).
pub async fn clean_npm() -> u64 {
    let cache = home().join(".npm");
    let size = dir_size(&cache).await;
    // Try CLI first (cleans internal index too)
    let _ = tokio::process::Command::new("npm")
        .args(["cache", "clean", "--force"])
        .output()
        .await;
    // Then remove the on-disk cache directory
    remove_dir_all_if_exists(&cache).await;
    size
}

/// Clean pip cache (`~/.cache/pip`).
pub async fn clean_pip() -> u64 {
    let cache = home().join(".cache/pip");
    let size = dir_size(&cache).await;
    remove_dir_all_if_exists(&cache).await;
    size
}

/// Clean Cargo cache (`~/.cargo/registry/cache/` and `~/.cargo/.package-cache`).
pub async fn clean_cargo() -> u64 {
    let cargo_home = home().join(".cargo");
    let pkg_cache = cargo_home.join("registry/cache");
    let dl_cache = cargo_home.join("registry/cache");
    let package_cache = cargo_home.join(".package-cache");

    let size = dir_size(&pkg_cache).await
        + dir_size(&dl_cache).await
        + if package_cache.exists() {
            package_cache.metadata().map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };

    remove_dir_all_if_exists(&pkg_cache).await;
    remove_dir_all_if_exists(&dl_cache).await;
    remove_file_if_exists(&package_cache).await;
    size
}

/// Clean Go module cache (`$GOPATH/pkg/mod` or `~/go/pkg/mod`).
pub async fn clean_go() -> u64 {
    let go_path = std::env::var("GOPATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home().join("go"));
    let cache = go_path.join("pkg/mod/cache");
    let size = dir_size(&cache).await;
    remove_dir_all_if_exists(&cache).await;
    size
}

/// Clean Maven local repository (`~/.m2/repository`) — only snapshot/non-release artifacts.
pub async fn clean_maven() -> u64 {
    let repo = home().join(".m2/repository");
    if !repo.exists() {
        return 0;
    }
    let mut removed: u64 = 0;
    let mut stack = vec![repo.clone()];
    while let Some(dir) = stack.pop() {
        let mut entries = match fs::read_dir(&dir).await {
            Ok(v) => v,
            Err(_) => continue,
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(meta) = entry.metadata().await {
                if meta.is_dir() {
                    // Snapshots live under <group>/<artifact>/<version>-SNAPSHOT/
                    if entry.file_name().to_string_lossy().ends_with("SNAPSHOT") {
                        removed += dir_size(&entry.path()).await;
                        remove_dir_all_if_exists(&entry.path()).await;
                    } else {
                        stack.push(entry.path());
                    }
                }
            }
        }
    }
    removed
}

/// Clean Gradle cache (`~/.gradle/caches`).
pub async fn clean_gradle() -> u64 {
    let cache = home().join(".gradle/caches");
    let size = dir_size(&cache).await;
    remove_dir_all_if_exists(&cache).await;
    size
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Dev cache summary — sizes of all dev-tool cache directories.
#[derive(Debug, Clone, serde::Serialize)]
#[derive(Default)]
pub struct DevCacheSummary {
    pub npm_bytes: u64,
    pub pip_bytes: u64,
    pub cargo_bytes: u64,
    pub go_bytes: u64,
    pub maven_bytes: u64,
    pub gradle_bytes: u64,
}

/// Get combined dev cache summary (async).
pub async fn dev_cache_summary() -> DevCacheSummary {
    let npm = home().join(".npm");
    let pip = home().join(".cache/pip");
    let cargo = home().join(".cargo");
    let go_path = std::env::var("GOPATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home().join("go"));
    let maven = home().join(".m2/repository");
    let gradle = home().join(".gradle/caches");

    DevCacheSummary {
        npm_bytes: dir_size(&npm).await,
        pip_bytes: dir_size(&pip).await,
        cargo_bytes: dir_size(&cargo).await,
        go_bytes: dir_size(&go_path.join("pkg/mod")).await,
        maven_bytes: dir_size(&maven).await,
        gradle_bytes: dir_size(&gradle).await,
    }
}
