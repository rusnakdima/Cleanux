//! HealthService - orchestrates system health monitoring

use crate::domain::HealthSnapshot;

pub trait HealthServiceTrait {
    fn take_snapshot(&mut self) -> Result<HealthSnapshot, String>;
    fn compare_snapshots(
        &self,
        before: &str,
        after: &str,
    ) -> Result<crate::domain::SnapshotComparison, String>;
}
