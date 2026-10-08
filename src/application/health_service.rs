//! HealthService - orchestrates system health monitoring

use crate::domain::HealthSnapshot;
use crate::error::Result;

pub trait HealthServiceTrait: Send + Sync {
  fn take_snapshot(&mut self) -> Result<HealthSnapshot>;
  fn compare_snapshots(
    &self,
    before: &str,
    after: &str,
  ) -> Result<crate::domain::SnapshotComparison>;
}
