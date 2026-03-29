/* helpers */
use crate::helpers::{data_empty_string, error_response};
/* models */
use crate::models::{DataValue, ResponseModel, ResponseStatus};
/* sys lib */
use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::json;
use std::fs;
use std::path::Path;

pub struct FilePreviewService;

impl FilePreviewService {
  pub fn preview_file(path: String) -> Result<ResponseModel, ResponseModel> {
    let file_path = Path::new(&path);

    if !file_path.exists() {
      return Err(error_response(
        "File not found",
        DataValue::String(String::new()),
      ));
    }

    let extension = file_path
      .extension()
      .and_then(|e| e.to_str())
      .unwrap_or("")
      .to_lowercase();

    let file_type = classify_file_type(&path, &extension);

    let name = file_path
      .file_name()
      .and_then(|n| n.to_str())
      .unwrap_or("unknown")
      .to_string();

    let response_data = match file_type {
      FileKind::Image => {
        let bytes =
          fs::read(&path).map_err(|e| error_response(format!("Failed to read file: {}", e), data_empty_string()))?;
        let base64 = STANDARD.encode(&bytes);
        let mime_type = match extension.as_str() {
          "png" => "image/png",
          "gif" => "image/gif",
          "bmp" => "image/bmp",
          "webp" => "image/webp",
          "svg" => "image/svg+xml",
          _ => "image/jpeg",
        };
        let data_url = format!("data:{};base64,{}", mime_type, base64);
        json!({
          "name": name,
          "path": path,
          "type": "image",
          "imageUrl": data_url
        })
      }
      FileKind::Text => {
        let bytes =
          fs::read(&path).map_err(|e| error_response(format!("Failed to read file: {}", e), data_empty_string()))?;
        let content = String::from_utf8_lossy(&bytes).into_owned();
        let truncated_content = if content.len() > 50000 {
          format!(
            "{}...\n\n[Content truncated - file too large]",
            &content[..50000]
          )
        } else {
          content
        };
        json!({
          "name": name,
          "path": path,
          "type": "text",
          "content": truncated_content
        })
      }
      FileKind::Binary | FileKind::Unknown => json!({
        "name": name,
        "path": path,
        "type": match file_type {
          FileKind::Binary => "binary",
          _ => "unknown",
        }
      }),
    };

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "File preview retrieved".to_string(),
      data: DataValue::Object(response_data),
    })
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FileKind {
  Image,
  Text,
  Binary,
  Unknown,
}

fn classify_file_type(path: &str, extension: &str) -> FileKind {
  let image_ext = [
    "jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "ico",
  ];
  let text_ext = [
    "txt", "md", "json", "xml", "html", "css", "js", "ts", "rs", "py", "java", "c", "cpp", "h",
    "hpp", "go", "rb", "php", "sh", "bash", "zsh", "yaml", "yml", "toml", "ini", "cfg", "log",
    "conf", "properties", "env", "gitignore", "dockerignore", "editorconfig",
  ];

  if image_ext.contains(&extension) {
    return FileKind::Image;
  }

  let meta_len = fs::metadata(path).ok().map(|m| m.len()).unwrap_or(u64::MAX);

  if text_ext.contains(&extension) {
    return sniff_text_by_utf8(path, meta_len);
  }

  sniff_printable_ascii(path, meta_len)
}

fn sniff_text_by_utf8(path: &str, meta_len: u64) -> FileKind {
  if meta_len >= 1024 * 1024 {
    return FileKind::Text;
  }
  match fs::read(path) {
    Ok(bytes) => {
      if String::from_utf8(bytes.clone()).is_ok() {
        let is_binary = bytes.iter().take(8000).any(|&b| b == 0);
        if !is_binary {
          FileKind::Text
        } else {
          FileKind::Binary
        }
      } else {
        FileKind::Binary
      }
    }
    Err(_) => FileKind::Binary,
  }
}

fn sniff_printable_ascii(path: &str, meta_len: u64) -> FileKind {
  if meta_len >= 1024 * 1024 {
    return FileKind::Binary;
  }
  match fs::read(path) {
    Ok(bytes) => {
      let printable = bytes.iter().take(8000).all(|&b| {
        b == 0 || (32..127).contains(&b) || b == 9 || b == 10 || b == 13
      });
      if printable {
        FileKind::Text
      } else {
        FileKind::Binary
      }
    }
    Err(_) => FileKind::Unknown,
  }
}
