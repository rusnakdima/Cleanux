//! Statistics Service - analytics for tasks, todos, and completion trends.

pub mod entities;
pub mod errors;
pub mod service_trait;

pub use entities::{CategoryItem, StatisticsPeriod, TodoStatistics, TrendItem, UserStatistics};
pub use errors::StatisticsError;
pub use service_trait::StatisticsService;
