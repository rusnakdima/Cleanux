//! JSON file-based storage for Cleanux entities

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct JsonStorage {
    pub base_path: PathBuf,
}

impl JsonStorage {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    pub fn load<T: serde::de::DeserializeOwned + Default>(
        &self,
        filename: &str,
    ) -> Result<T, String> {
        let path = self.base_path.join(filename);
        if !path.exists() {
            return Ok(T::default());
        }
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read {}: {}", filename, e))?;
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse {}: {}", filename, e))
    }

    pub fn save<T: serde::Serialize>(&self, filename: &str, data: &T) -> Result<(), String> {
        let path = self.base_path.join(filename);
        let content = serde_json::to_string_pretty(data)
            .map_err(|e| format!("Failed to serialize {}: {}", filename, e))?;
        std::fs::write(&path, content).map_err(|e| format!("Failed to write {}: {}", filename, e))
    }
}
