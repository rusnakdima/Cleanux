//! Clipboard service — manages clipboard history, pinning, and cleanup.
//!
//! Provides clipboard history management with support for pinning items
//! and clearing history.

use crate::error::AppError;
use parking_lot::RwLock;

/// Result type alias for this module.
pub type Result<T, E = AppError> = std::result::Result<T, E>;

/// A clipboard history item.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ClipboardItem {
    pub id: u64,
    pub content: String,
    pub content_type: String,
    pub preview: String,
    pub timestamp: i64,
    pub pinned: bool,
}

/// Global clipboard history storage.
static CLIPBOARD_HISTORY: RwLock<Vec<ClipboardItem>> = RwLock::new(Vec::new());

/// Maximum number of items to keep in history.
const MAX_HISTORY_SIZE: usize = 100;

static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// Get all clipboard history items, sorted by timestamp (newest first).
/// Pinned items appear first, then unpinned items.
pub async fn get_clipboard_history() -> Result<Vec<ClipboardItem>> {
    let history = CLIPBOARD_HISTORY.read();
    let mut items: Vec<ClipboardItem> = history.clone();

    // Sort: pinned first, then by timestamp descending
    items.sort_by(|a, b| match (a.pinned, b.pinned) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => b.timestamp.cmp(&a.timestamp),
    });

    Ok(items)
}

/// Add a new item to clipboard history.
/// Duplicates are allowed (user may copy same content multiple times).
pub async fn add_clipboard_item(content: String) -> Result<ClipboardItem> {
    let preview = truncate_preview(&content, 100);
    let content_type = detect_content_type(&content);
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    let item = ClipboardItem {
        id,
        content,
        content_type,
        preview,
        timestamp,
        pinned: false,
    };

    let mut history =
        CLIPBOARD_HISTORY.write();

    // Enforce size limit (skip oldest unpinned items)
    while history.len() >= MAX_HISTORY_SIZE {
        // Find oldest unpinned item
        if let Some(pos) = history.iter().position(|i| !i.pinned) {
            history.remove(pos);
        } else {
            // All items are pinned — can't add more without exceeding limit
            break;
        }
    }

    history.push(item.clone());
    Ok(item)
}

/// Clear all unpinned items from history.
/// Pinned items are preserved.
pub async fn clear_history() -> Result<usize> {
    let mut history =
        CLIPBOARD_HISTORY.write();
    let initial_len = history.len();
    history.retain(|item| item.pinned);
    let cleared = initial_len - history.len();
    Ok(cleared)
}

/// Clear all items from history, including pinned items.
pub async fn clear_all_history() -> Result<usize> {
    let mut history =
        CLIPBOARD_HISTORY.write();
    let cleared = history.len();
    history.clear();
    Ok(cleared)
}

/// Pin or unpin a clipboard item by ID.
pub async fn pin_item(id: u64) -> Result<ClipboardItem> {
    let mut history =
        CLIPBOARD_HISTORY.write();

    history
        .iter_mut()
        .find(|i| i.id == id)
        .map(|item| {
            item.pinned = !item.pinned;
            item.clone()
        })
        .ok_or_else(|| AppError::NotFound(format!("clipboard item {} not found", id).into()))
}

/// Delete a specific clipboard item by ID.
pub async fn delete_item(id: u64) -> Result<()> {
    let mut history =
        CLIPBOARD_HISTORY.write();
    let initial_len = history.len();
    history.retain(|item| item.id != id);

    if history.len() == initial_len {
        return Err(AppError::NotFound(format!("clipboard item {} not found", id).into()));
    }
    Ok(())
}

/// Get a single clipboard item by ID.
pub async fn get_item(id: u64) -> Result<ClipboardItem> {
    let history = CLIPBOARD_HISTORY.read();
    history
        .iter()
        .find(|i| i.id == id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("clipboard item {} not found", id).into()))
}

// ---------------------------------------------------------------------------
// Internal helpers (mirrored from clipboard_page.rs)
// ---------------------------------------------------------------------------

fn truncate_preview(content: &str, max_len: usize) -> String {
    if content.len() <= max_len {
        content.to_string()
    } else {
        format!("{}...", &content[..max_len])
    }
}

fn detect_content_type(content: &str) -> String {
    if content.starts_with("file://")
        || content
            .contains('\n')
            && content.lines().all(|l| l.starts_with('/') || l.starts_with('~'))
    {
        "file".to_string()
    } else if content.starts_with("data:image/") {
        "image".to_string()
    } else {
        "text".to_string()
    }
}

/// Format a Unix timestamp into a human-readable relative string.
pub fn format_timestamp(ts: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let diff = now - ts;
    if diff < 60 {
        "Just now".to_string()
    } else if diff < 3600 {
        format!("{}m ago", diff / 60)
    } else if diff < 86400 {
        format!("{}h ago", diff / 3600)
    } else {
        format!("{}d ago", diff / 86400)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_and_get_clipboard_item() {
        let item = add_clipboard_item("test content".to_string()).await.unwrap();
        assert_eq!(item.content, "test content");
        assert_eq!(item.content_type, "text");
        assert!(!item.pinned);

        let history = get_clipboard_history().await.unwrap();
        assert_eq!(history.len(), 1);
    }

    #[tokio::test]
    async fn test_pin_item() {
        let item = add_clipboard_item("pin me".to_string()).await.unwrap();
        assert!(!item.pinned);

        let pinned = pin_item(item.id).await.unwrap();
        assert!(pinned.pinned);

        // Toggle off
        let unpinned = pin_item(item.id).await.unwrap();
        assert!(!unpinned.pinned);
    }

    #[tokio::test]
    async fn test_delete_item() {
        let item = add_clipboard_item("delete me".to_string()).await.unwrap();
        delete_item(item.id).await.unwrap();

        let history = get_clipboard_history().await.unwrap();
        assert!(history.iter().all(|i| i.id != item.id));
    }

    #[tokio::test]
    async fn test_clear_history_keeps_pinned() {
        let pinned = add_clipboard_item("pinned item".to_string()).await.unwrap();
        let _unpinned = add_clipboard_item("unpinned item".to_string()).await.unwrap();

        pin_item(pinned.id).await.unwrap();
        let cleared = clear_history().await.unwrap();
        assert_eq!(cleared, 1);

        let history = get_clipboard_history().await.unwrap();
        assert_eq!(history.len(), 1);
        assert!(history[0].pinned);
    }

    #[tokio::test]
    async fn test_detect_content_type() {
        assert_eq!(detect_content_type("hello world"), "text");
        assert_eq!(detect_content_type("data:image/png;base64,abc"), "image");
        assert_eq!(
            detect_content_type("/home/user/file.txt\n/home/user2/file2.txt"),
            "file"
        );
    }

    #[tokio::test]
    async fn test_not_found() {
        let result = get_item(99999).await;
        assert!(result.is_err());

        let result = pin_item(99999).await;
        assert!(result.is_err());
    }
}
