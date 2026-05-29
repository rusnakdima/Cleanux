/* sys lib */
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use parking_lot::RwLock;

pub struct CacheService {
  string_cache: Arc<RwLock<HashMap<String, CacheEntry<String>>>>,
  object_cache: Arc<RwLock<HashMap<String, CacheEntry<serde_json::Value>>>>,
  default_ttl_secs: u64,
}

struct CacheEntry<V> {
  value: V,
  timestamp_secs: u64,
  ttl_secs: u64,
}

impl CacheEntry<String> {
  fn is_expired(&self) -> bool {
    get_current_secs() - self.timestamp_secs >= self.ttl_secs
  }
}

impl CacheEntry<serde_json::Value> {
  fn is_expired(&self) -> bool {
    get_current_secs() - self.timestamp_secs >= self.ttl_secs
  }
}

fn get_current_secs() -> u64 {
  std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .map(|d| d.as_secs())
    .unwrap_or(0)
}

impl CacheService {
  pub fn new() -> Self {
    Self {
      string_cache: Arc::new(RwLock::new(HashMap::new())),
      object_cache: Arc::new(RwLock::new(HashMap::new())),
      default_ttl_secs: 60,
    }
  }

  pub fn with_ttl(ttl_secs: u64) -> Self {
    Self {
      string_cache: Arc::new(RwLock::new(HashMap::new())),
      object_cache: Arc::new(RwLock::new(HashMap::new())),
      default_ttl_secs: ttl_secs,
    }
  }

  pub fn get_string(&self, key: &str) -> Option<String> {
    let cache = self.string_cache.read();
    cache.get(key).and_then(|entry| {
      if entry.is_expired() {
        None
      } else {
        Some(entry.value.clone())
      }
    })
  }

  pub fn set_string(&self, key: String, value: String, ttl_secs: Option<u64>) {
    let ttl = ttl_secs.unwrap_or(self.default_ttl_secs);
    let entry = CacheEntry {
      value,
      timestamp_secs: get_current_secs(),
      ttl_secs: ttl,
    };
    self.string_cache.write().insert(key, entry);
  }

  pub fn get_object(&self, key: &str) -> Option<serde_json::Value> {
    let cache = self.object_cache.read();
    cache.get(key).and_then(|entry| {
      if entry.is_expired() {
        None
      } else {
        Some(entry.value.clone())
      }
    })
  }

  pub fn set_object(&self, key: String, value: serde_json::Value, ttl_secs: Option<u64>) {
    let ttl = ttl_secs.unwrap_or(self.default_ttl_secs);
    let entry = CacheEntry {
      value,
      timestamp_secs: get_current_secs(),
      ttl_secs: ttl,
    };
    self.object_cache.write().insert(key, entry);
  }

  pub fn invalidate(&self, key: &str) {
    self.string_cache.write().remove(key);
    self.object_cache.write().remove(key);
  }

  pub fn clear(&self) {
    self.string_cache.write().clear();
    self.object_cache.write().clear();
  }

  pub fn cleanup_expired(&self) {
    let mut string_cache = self.string_cache.write();
    string_cache.retain(|_, entry| !entry.is_expired());

    let mut object_cache = self.object_cache.write();
    object_cache.retain(|_, entry| !entry.is_expired());
  }
}

impl Default for CacheService {
  fn default() -> Self {
    Self::new()
  }
}

pub fn compute_cache_key(operation: &str, params: &[(&str, &str)]) -> String {
  let mut hasher = std::collections::hash_map::DefaultHasher::new();
  operation.hash(&mut hasher);
  for (key, value) in params {
    key.hash(&mut hasher);
    value.hash(&mut hasher);
  }
  format!("{:x}", hasher.finish())
}