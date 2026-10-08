//! Bounded execution-history service using a local in-memory implementation.

use parking_lot::Mutex;
use std::collections::VecDeque;

/// A simple bounded history service with FIFO eviction.
pub struct BoundedHistoryService<T> {
  entries: Mutex<VecDeque<T>>,
  capacity: usize,
}

impl<T> BoundedHistoryService<T> {
  /// Construct an in-memory bounded history service (capacity 100, newest first).
  pub fn new_in_memory(capacity: usize) -> Self {
    Self {
      entries: Mutex::new(VecDeque::with_capacity(capacity)),
      capacity,
    }
  }

  /// Add an entry to the history.
  pub fn add_entry(&self, entry: T) {
    let mut entries = self.entries.lock();
    if entries.len() >= self.capacity {
      entries.pop_front();
    }
    entries.push_back(entry);
  }

  /// Get all entries in the history.
  pub fn get_all(&self) -> Vec<T>
  where
    T: Clone,
  {
    self.entries.lock().iter().cloned().collect()
  }

  /// Get the current length of the history.
  pub fn len(&self) -> usize {
    self.entries.lock().len()
  }

  /// Check if the history is empty.
  pub fn is_empty(&self) -> bool {
    self.entries.lock().is_empty()
  }

  /// Clear all entries.
  pub fn clear(&self) {
    self.entries.lock().clear();
  }
}

/// Capacity matches the legacy `EXECUTION_HISTORY_CAP`.
pub type HistorySvc =
  BoundedHistoryService<crate::domain::entities::execution_history::ExecutionHistory>;

/// Construct an in-memory bounded history service (capacity 100, newest first).
pub fn new_history_service() -> HistorySvc {
  BoundedHistoryService::new_in_memory(100)
}
