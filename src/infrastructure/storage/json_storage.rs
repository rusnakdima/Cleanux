//! JSON storage implementation

use anyhow::Result;
use std::path::PathBuf;

/// JSON file-based storage
pub struct JsonStorage {
    data_dir: PathBuf,
}

impl JsonStorage {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    pub async fn load<T: serde::de::DeserializeOwned>(&self, name: &str) -> Result<Option<T>> {
        let path = self.data_dir.join(format!("{}.json", name));
        if !path.exists() {
            return Ok(None);
        }
        let content = tokio::fs::read_to_string(&path).await?;
        let data = serde_json::from_str(&content)?;
        Ok(Some(data))
    }

    pub async fn save<T: serde::Serialize>(&self, name: &str, data: &T) -> Result<()> {
        let path = self.data_dir.join(format!("{}.json", name));
        let content = serde_json::to_string_pretty(data)?;
        tokio::fs::write(&path, content).await?;
        Ok(())
    }
}
