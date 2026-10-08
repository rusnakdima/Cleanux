//! HealthServiceTrait — domain trait for system health monitoring.
//!
//! Defines the contract for health snapshots and comparison.
//! Implementation lives in `infrastructure`.

use crate::domain::HealthSnapshot;
use crate::domain::SnapshotComparison;
use crate::error::Result;

pub trait HealthServiceTrait: Send + Sync {
    fn take_snapshot(&mut self) -> Result<HealthSnapshot>;
    fn compare_snapshots(
        &self,
        before: &str,
        after: &str,
    ) -> Result<SnapshotComparison>;
}
