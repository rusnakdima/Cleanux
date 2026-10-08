//! Statistics service trait.

use async_trait::async_trait;

use super::{
    CategoryItem, StatisticsError, StatisticsPeriod, TodoStatistics, TrendItem, UserStatistics,
};

/// Statistics service for analytics across tasks and todos.
#[async_trait]
pub trait StatisticsService: Send + Sync {
    /// Get statistics for a specific todo.
    async fn get_todo_stats(
        &self,
        todo_id: &str,
        user_id: &str,
    ) -> Result<TodoStatistics, StatisticsError>;

    /// Get user-level statistics for the given period.
    async fn get_user_stats(
        &self,
        user_id: &str,
        period: StatisticsPeriod,
    ) -> Result<UserStatistics, StatisticsError>;

    /// Get category breakdown for a user.
    async fn get_category_breakdown(
        &self,
        user_id: &str,
    ) -> Result<Vec<CategoryItem>, StatisticsError>;

    /// Get daily completion trend for the last N days.
    async fn get_completion_trend(
        &self,
        user_id: &str,
        days: u32,
    ) -> Result<Vec<TrendItem>, StatisticsError>;
}
