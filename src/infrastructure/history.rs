//! Bounded execution-history service using the canonical
//! `dioxus_shared::services::BoundedHistoryService`.

use crate::domain::entities::execution_history::ExecutionHistory;
use dioxus_shared::services::bounded_history::BoundedHistoryService;

/// Capacity matches the legacy `EXECUTION_HISTORY_CAP`.
pub type HistorySvc = BoundedHistoryService<ExecutionHistory>;

/// Construct an in-memory bounded history service (capacity 100, newest first).
pub fn new_history_service() -> HistorySvc {
    BoundedHistoryService::new_in_memory(100)
}
