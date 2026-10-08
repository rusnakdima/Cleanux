//! Statistics domain entities.

use chrono::NaiveDate;

/// Todo-level statistics.
#[derive(Debug, Clone)]
pub struct TodoStatistics {
    pub total_tasks: u32,
    pub completed_tasks: u32,
    pub completion_rate: f64,
    pub overdue_count: u32,
}

/// User-level statistics.
#[derive(Debug, Clone)]
pub struct UserStatistics {
    pub tasks_completed: u32,
    pub time_tracked_minutes: u64,
    pub current_streak: u32,
    pub longest_streak: u32,
}

/// Category breakdown item.
#[derive(Debug, Clone)]
pub struct CategoryItem {
    pub category_id: String,
    pub category_name: String,
    pub task_count: u32,
    pub completion_rate: f64,
}

/// Daily completion trend item.
#[derive(Debug, Clone)]
pub struct TrendItem {
    pub date: NaiveDate,
    pub completed: u32,
    pub created: u32,
}

/// Statistics aggregation period.
#[derive(Debug, Clone, Copy)]
pub enum StatisticsPeriod {
    Week,
    Month,
    Quarter,
    Year,
}
