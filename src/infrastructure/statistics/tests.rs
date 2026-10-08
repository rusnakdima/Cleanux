//! Tests for StatisticsService trait and statistics operations.

use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use crate::infrastructure::statistics::entities::{
    CategoryItem, StatisticsPeriod, TodoStatistics, TrendItem, UserStatistics,
};
use crate::infrastructure::statistics::errors::StatisticsError;
use crate::infrastructure::statistics::service_trait::StatisticsService;

// =============================================================================
// Mock StatisticsService implementation for testing
// =============================================================================

pub struct MockStatisticsService {
    /// Todo statistics to return
    pub todo_stats: Option<TodoStatistics>,
    /// User statistics to return
    pub user_stats: Option<UserStatistics>,
    /// Category breakdown to return
    pub category_breakdown: Vec<CategoryItem>,
    /// Completion trend to return
    pub completion_trend: Vec<TrendItem>,
    /// Whether to fail on get_todo_stats
    pub fail_todo_stats: bool,
    /// Whether to fail on get_user_stats
    pub fail_user_stats: bool,
    /// Whether to fail on get_category_breakdown
    pub fail_category_breakdown: bool,
    /// Whether to fail on get_completion_trend
    pub fail_completion_trend: bool,
    /// Records of all get_todo_stats calls
    pub todo_stats_calls: Arc<Mutex<Vec<(String, String)>>>,
    /// Records of all get_user_stats calls
    pub user_stats_calls: Arc<Mutex<Vec<(String, StatisticsPeriod)>>>,
    /// Records of all get_category_breakdown calls
    pub category_calls: Arc<Mutex<Vec<String>>>,
    /// Records of all get_completion_trend calls
    pub trend_calls: Arc<Mutex<Vec<(String, u32)>>>,
}

impl Default for MockStatisticsService {
    fn default() -> Self {
        Self {
            todo_stats: None,
            user_stats: None,
            category_breakdown: Vec::new(),
            completion_trend: Vec::new(),
            fail_todo_stats: false,
            fail_user_stats: false,
            fail_category_breakdown: false,
            fail_completion_trend: false,
            todo_stats_calls: Arc::new(Mutex::new(Vec::new())),
            user_stats_calls: Arc::new(Mutex::new(Vec::new())),
            category_calls: Arc::new(Mutex::new(Vec::new())),
            trend_calls: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl MockStatisticsService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a service with predefined todo statistics
    pub fn with_todo_stats(stats: TodoStatistics) -> Self {
        Self {
            todo_stats: Some(stats),
            ..Default::default()
        }
    }

    /// Create a service with predefined user statistics
    pub fn with_user_stats(stats: UserStatistics) -> Self {
        Self {
            user_stats: Some(stats),
            ..Default::default()
        }
    }

    /// Create a service with predefined category breakdown
    pub fn with_categories(categories: Vec<CategoryItem>) -> Self {
        Self {
            category_breakdown: categories,
            ..Default::default()
        }
    }

    /// Create a service with predefined completion trend
    pub fn with_trend(trend: Vec<TrendItem>) -> Self {
        Self {
            completion_trend: trend,
            ..Default::default()
        }
    }

    /// Create a service that fails on todo stats
    pub fn with_todo_stats_failure(msg: &str) -> Self {
        Self {
            fail_todo_stats: true,
            ..Default::default()
        }
    }

    /// Create a service that fails on user stats
    pub fn with_user_stats_failure(msg: &str) -> Self {
        Self {
            fail_user_stats: true,
            ..Default::default()
        }
    }

    /// Get recorded get_todo_stats calls
    pub fn get_todo_calls(&self) -> Vec<(String, String)> {
        self.todo_stats_calls.lock().unwrap().clone()
    }

    /// Get recorded get_user_stats calls
    pub fn get_user_calls(&self) -> Vec<(String, StatisticsPeriod)> {
        self.user_stats_calls.lock().unwrap().clone()
    }

    /// Get recorded get_category_breakdown calls
    pub fn get_category_calls(&self) -> Vec<String> {
        self.category_calls.lock().unwrap().clone()
    }

    /// Get recorded get_completion_trend calls
    pub fn get_trend_calls(&self) -> Vec<(String, u32)> {
        self.trend_calls.lock().unwrap().clone()
    }
}

#[async_trait]
impl StatisticsService for MockStatisticsService {
    async fn get_todo_stats(
        &self,
        todo_id: &str,
        user_id: &str,
    ) -> Result<TodoStatistics, StatisticsError> {
        self.todo_stats_calls
            .lock()
            .unwrap()
            .push((todo_id.to_string(), user_id.to_string()));

        if self.fail_todo_stats {
            return Err(StatisticsError::TodoNotFound("Mock failure".to_string()));
        }

        self.todo_stats.clone().ok_or_else(|| {
            StatisticsError::TodoNotFound(format!("todo_id not found: {}", todo_id))
        })
    }

    async fn get_user_stats(
        &self,
        user_id: &str,
        period: StatisticsPeriod,
    ) -> Result<UserStatistics, StatisticsError> {
        self.user_stats_calls
            .lock()
            .unwrap()
            .push((user_id.to_string(), period));

        if self.fail_user_stats {
            return Err(StatisticsError::UserNotFound("Mock failure".to_string()));
        }

        self.user_stats.clone().ok_or_else(|| {
            StatisticsError::UserNotFound(format!("user_id not found: {}", user_id))
        })
    }

    async fn get_category_breakdown(
        &self,
        user_id: &str,
    ) -> Result<Vec<CategoryItem>, StatisticsError> {
        self.category_calls
            .lock()
            .unwrap()
            .push(user_id.to_string());

        if self.fail_category_breakdown {
            return Err(StatisticsError::Storage("Mock failure".to_string()));
        }

        Ok(self.category_breakdown.clone())
    }

    async fn get_completion_trend(
        &self,
        user_id: &str,
        days: u32,
    ) -> Result<Vec<TrendItem>, StatisticsError> {
        self.trend_calls
            .lock()
            .unwrap()
            .push((user_id.to_string(), days));

        if self.fail_completion_trend {
            return Err(StatisticsError::Storage("Mock failure".to_string()));
        }

        Ok(self.completion_trend.clone())
    }
}

// =============================================================================
// StatisticsError tests
// =============================================================================

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn test_todo_not_found_error() {
        let err = StatisticsError::TodoNotFound("todo_123".to_string());
        assert!(err.to_string().contains("todo_123"));
        assert!(err.to_string().contains("Todo not found"));
    }

    #[test]
    fn test_user_not_found_error() {
        let err = StatisticsError::UserNotFound("user_456".to_string());
        assert!(err.to_string().contains("user_456"));
        assert!(err.to_string().contains("User not found"));
    }

    #[test]
    fn test_storage_error() {
        let err = StatisticsError::Storage("Database connection failed".to_string());
        assert!(err.to_string().contains("Storage error"));
        assert!(err.to_string().contains("Database connection failed"));
    }

    #[test]
    fn test_serialization_error() {
        let err = StatisticsError::Serialization("Invalid JSON".to_string());
        assert!(err.to_string().contains("Serialization error"));
        assert!(err.to_string().contains("Invalid JSON"));
    }

    #[test]
    fn test_error_debug_format() {
        let err = StatisticsError::TodoNotFound("id".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("TodoNotFound"));
    }
}

// =============================================================================
// StatisticsPeriod tests
// =============================================================================

#[cfg(test)]
mod statistics_period_tests {
    use super::*;

    #[test]
    fn test_statistics_period_variants() {
        let period = StatisticsPeriod::Week;
        assert!(matches!(period, StatisticsPeriod::Week));

        let period = StatisticsPeriod::Month;
        assert!(matches!(period, StatisticsPeriod::Month));

        let period = StatisticsPeriod::Quarter;
        assert!(matches!(period, StatisticsPeriod::Quarter));

        let period = StatisticsPeriod::Year;
        assert!(matches!(period, StatisticsPeriod::Year));
    }

    #[test]
    fn test_statistics_period_debug() {
        let period = StatisticsPeriod::Month;
        let debug = format!("{:?}", period);
        assert!(debug.contains("Month"));
    }
}

// =============================================================================
// StatisticsEntities tests
// =============================================================================

#[cfg(test)]
mod entity_tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_todo_statistics() {
        let stats = TodoStatistics {
            total_tasks: 10,
            completed_tasks: 7,
            completion_rate: 0.7,
            overdue_count: 2,
        };

        assert_eq!(stats.total_tasks, 10);
        assert_eq!(stats.completed_tasks, 7);
        assert!((stats.completion_rate - 0.7).abs() < f64::EPSILON);
        assert_eq!(stats.overdue_count, 2);
    }

    #[test]
    fn test_user_statistics() {
        let stats = UserStatistics {
            tasks_completed: 150,
            time_tracked_minutes: 3000,
            current_streak: 5,
            longest_streak: 12,
        };

        assert_eq!(stats.tasks_completed, 150);
        assert_eq!(stats.time_tracked_minutes, 3000);
        assert_eq!(stats.current_streak, 5);
        assert_eq!(stats.longest_streak, 12);
    }

    #[test]
    fn test_category_item() {
        let item = CategoryItem {
            category_id: "cat_work".to_string(),
            category_name: "Work".to_string(),
            task_count: 25,
            completion_rate: 0.8,
        };

        assert_eq!(item.category_id, "cat_work");
        assert_eq!(item.category_name, "Work");
        assert_eq!(item.task_count, 25);
        assert!((item.completion_rate - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn test_trend_item() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let trend = TrendItem {
            date,
            completed: 8,
            created: 5,
        };

        assert_eq!(trend.date, date);
        assert_eq!(trend.completed, 8);
        assert_eq!(trend.created, 5);
    }

    #[test]
    fn test_statistics_clone() {
        let stats = TodoStatistics {
            total_tasks: 10,
            completed_tasks: 5,
            completion_rate: 0.5,
            overdue_count: 1,
        };
        let cloned = stats.clone();
        assert_eq!(cloned.total_tasks, stats.total_tasks);
        assert_eq!(cloned.completed_tasks, stats.completed_tasks);
    }
}

// =============================================================================
// StatisticsService tests - get_todo_stats
// =============================================================================

#[cfg(test)]
mod get_todo_stats_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_todo_stats_success() {
        let stats = TodoStatistics {
            total_tasks: 20,
            completed_tasks: 15,
            completion_rate: 0.75,
            overdue_count: 3,
        };
        let service = MockStatisticsService::with_todo_stats(stats);

        let result = service.get_todo_stats("todo_123", "user_456").await;

        assert!(result.is_ok());
        let returned = result.unwrap();
        assert_eq!(returned.total_tasks, 20);
        assert_eq!(returned.completed_tasks, 15);
    }

    #[tokio::test]
    async fn test_get_todo_stats_records_parameters() {
        let stats = TodoStatistics {
            total_tasks: 10,
            completed_tasks: 5,
            completion_rate: 0.5,
            overdue_count: 0,
        };
        let service = MockStatisticsService::with_todo_stats(stats);

        service
            .get_todo_stats("todo_abc", "user_xyz")
            .await
            .unwrap();

        let calls = service.get_todo_calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "todo_abc");
        assert_eq!(calls[0].1, "user_xyz");
    }

    #[tokio::test]
    async fn test_get_todo_stats_failure() {
        let service = MockStatisticsService::with_todo_stats_failure("not found");

        let result = service.get_todo_stats("todo_missing", "user_456").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            StatisticsError::TodoNotFound(msg) => {
                assert!(msg.contains("todo_missing"));
            }
            _ => panic!("Expected TodoNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_todo_stats_no_stats_configured() {
        let service = MockStatisticsService::new();

        let result = service.get_todo_stats("todo_123", "user_456").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            StatisticsError::TodoNotFound(msg) => {
                assert!(msg.contains("todo_123"));
            }
            _ => panic!("Expected TodoNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_todo_stats_multiple_calls() {
        let stats = TodoStatistics {
            total_tasks: 5,
            completed_tasks: 3,
            completion_rate: 0.6,
            overdue_count: 1,
        };
        let service = MockStatisticsService::with_todo_stats(stats);

        let _ = service.get_todo_stats("todo_1", "user_1").await;
        let _ = service.get_todo_stats("todo_2", "user_2").await;
        let _ = service.get_todo_stats("todo_3", "user_1").await;

        let calls = service.get_todo_calls();
        assert_eq!(calls.len(), 3);
    }
}

// =============================================================================
// StatisticsService tests - get_user_stats
// =============================================================================

#[cfg(test)]
mod get_user_stats_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_user_stats_success() {
        let stats = UserStatistics {
            tasks_completed: 100,
            time_tracked_minutes: 5000,
            current_streak: 7,
            longest_streak: 30,
        };
        let service = MockStatisticsService::with_user_stats(stats);

        let result = service
            .get_user_stats("user_123", StatisticsPeriod::Month)
            .await;

        assert!(result.is_ok());
        let returned = result.unwrap();
        assert_eq!(returned.tasks_completed, 100);
        assert_eq!(returned.current_streak, 7);
    }

    #[tokio::test]
    async fn test_get_user_stats_records_parameters() {
        let stats = UserStatistics {
            tasks_completed: 50,
            time_tracked_minutes: 1000,
            current_streak: 3,
            longest_streak: 10,
        };
        let service = MockStatisticsService::with_user_stats(stats);

        service
            .get_user_stats("user_abc", StatisticsPeriod::Week)
            .await
            .unwrap();

        let calls = service.get_user_calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "user_abc");
        assert!(matches!(calls[0].1, StatisticsPeriod::Week));
    }

    #[tokio::test]
    async fn test_get_user_stats_all_periods() {
        let stats = UserStatistics {
            tasks_completed: 50,
            time_tracked_minutes: 1000,
            current_streak: 3,
            longest_streak: 10,
        };
        let service = MockStatisticsService::with_user_stats(stats);

        let periods = [
            StatisticsPeriod::Week,
            StatisticsPeriod::Month,
            StatisticsPeriod::Quarter,
            StatisticsPeriod::Year,
        ];

        for period in periods {
            service
                .get_user_stats("user_test", period)
                .await
                .unwrap();
        }

        let calls = service.get_user_calls();
        assert_eq!(calls.len(), 4);
    }

    #[tokio::test]
    async fn test_get_user_stats_failure() {
        let service = MockStatisticsService::with_user_stats_failure("user not found");

        let result = service
            .get_user_stats("user_missing", StatisticsPeriod::Month)
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            StatisticsError::UserNotFound(msg) => {
                assert!(msg.contains("user_missing"));
            }
            _ => panic!("Expected UserNotFound error"),
        }
    }
}

// =============================================================================
// StatisticsService tests - get_category_breakdown
// =============================================================================

#[cfg(test)]
mod get_category_breakdown_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_category_breakdown_success() {
        let categories = vec![
            CategoryItem {
                category_id: "cat_work".to_string(),
                category_name: "Work".to_string(),
                task_count: 30,
                completion_rate: 0.8,
            },
            CategoryItem {
                category_id: "cat_personal".to_string(),
                category_name: "Personal".to_string(),
                task_count: 20,
                completion_rate: 0.6,
            },
        ];
        let service = MockStatisticsService::with_categories(categories);

        let result = service.get_category_breakdown("user_123").await;

        assert!(result.is_ok());
        let returned = result.unwrap();
        assert_eq!(returned.len(), 2);
        assert_eq!(returned[0].category_name, "Work");
        assert_eq!(returned[1].category_name, "Personal");
    }

    #[tokio::test]
    async fn test_get_category_breakdown_records_user_id() {
        let service = MockStatisticsService::with_categories(vec![]);

        service.get_category_breakdown("user_xyz").await.unwrap();

        let calls = service.get_category_calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], "user_xyz");
    }

    #[tokio::test]
    async fn test_get_category_breakdown_empty() {
        let service = MockStatisticsService::with_categories(vec![]);

        let result = service.get_category_breakdown("user_123").await;

        assert!(result.is_ok());
        let returned = result.unwrap();
        assert!(returned.is_empty());
    }

    #[tokio::test]
    async fn test_get_category_breakdown_multiple_calls() {
        let categories = vec![CategoryItem {
            category_id: "cat_test".to_string(),
            category_name: "Test".to_string(),
            task_count: 5,
            completion_rate: 0.5,
        }];
        let service = MockStatisticsService::with_categories(categories);

        let _ = service.get_category_breakdown("user_1").await;
        let _ = service.get_category_breakdown("user_2").await;

        let calls = service.get_category_calls();
        assert_eq!(calls.len(), 2);
    }
}

// =============================================================================
// StatisticsService tests - get_completion_trend
// =============================================================================

#[cfg(test)]
mod get_completion_trend_tests {
    use super::*;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_get_completion_trend_success() {
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        let trend = vec![
            TrendItem {
                date: date1,
                completed: 5,
                created: 3,
            },
            TrendItem {
                date: date2,
                completed: 7,
                created: 4,
            },
        ];
        let service = MockStatisticsService::with_trend(trend);

        let result = service.get_completion_trend("user_123", 30).await;

        assert!(result.is_ok());
        let returned = result.unwrap();
        assert_eq!(returned.len(), 2);
    }

    #[tokio::test]
    async fn test_get_completion_trend_records_parameters() {
        let service = MockStatisticsService::with_trend(vec![]);

        service.get_completion_trend("user_abc", 14).await.unwrap();

        let calls = service.get_trend_calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "user_abc");
        assert_eq!(calls[0].1, 14);
    }

    #[tokio::test]
    async fn test_get_completion_trend_empty() {
        let service = MockStatisticsService::with_trend(vec![]);

        let result = service.get_completion_trend("user_123", 7).await;

        assert!(result.is_ok());
        let returned = result.unwrap();
        assert!(returned.is_empty());
    }

    #[tokio::test]
    async fn test_get_completion_trend_different_day_counts() {
        let service = MockStatisticsService::with_trend(vec![]);

        let _ = service.get_completion_trend("user_1", 7).await;
        let _ = service.get_completion_trend("user_1", 14).await;
        let _ = service.get_completion_trend("user_1", 30).await;
        let _ = service.get_completion_trend("user_1", 90).await;

        let calls = service.get_trend_calls();
        assert_eq!(calls[0].1, 7);
        assert_eq!(calls[1].1, 14);
        assert_eq!(calls[2].1, 30);
        assert_eq!(calls[3].1, 90);
    }
}

// =============================================================================
// StatisticsService trait bounds tests
// =============================================================================

#[cfg(test)]
mod trait_bounds_tests {
    use super::*;

    #[test]
    fn test_mock_service_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MockStatisticsService>();
    }

    #[test]
    fn test_trait_object_bounds() {
        fn assert_send_sync<T: Send + Sync>() {}
        fn requires_service(_: impl Send + Sync) {}
        requires_service(MockStatisticsService::new());
    }
}

// =============================================================================
// StatisticsService integration tests
// =============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_statistics_workflow() {
        let todo_stats = TodoStatistics {
            total_tasks: 50,
            completed_tasks: 35,
            completion_rate: 0.7,
            overdue_count: 5,
        };
        let user_stats = UserStatistics {
            tasks_completed: 200,
            time_tracked_minutes: 10000,
            current_streak: 10,
            longest_streak: 45,
        };
        let categories = vec![
            CategoryItem {
                category_id: "work".to_string(),
                category_name: "Work".to_string(),
                task_count: 100,
                completion_rate: 0.75,
            },
            CategoryItem {
                category_id: "personal".to_string(),
                category_name: "Personal".to_string(),
                task_count: 50,
                completion_rate: 0.6,
            },
        ];

        let service = MockStatisticsService::with_todo_stats(todo_stats)
            .with_user_stats(user_stats)
            .with_categories(categories);

        // Get todo stats
        let todo_result = service.get_todo_stats("todo_1", "user_1").await;
        assert!(todo_result.is_ok());

        // Get user stats
        let user_result = service
            .get_user_stats("user_1", StatisticsPeriod::Month)
            .await;
        assert!(user_result.is_ok());

        // Get category breakdown
        let cat_result = service.get_category_breakdown("user_1").await;
        assert!(cat_result.is_ok());
        assert_eq!(cat_result.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_concurrent_statistics_requests() {
        let service = Arc::new(MockStatisticsService::with_todo_stats(TodoStatistics {
            total_tasks: 10,
            completed_tasks: 5,
            completion_rate: 0.5,
            overdue_count: 1,
        }));

        let mut handles = vec![];

        for i in 0..5 {
            let svc = service.clone();
            let handle = tokio::spawn(async move {
                svc.get_todo_stats(&format!("todo_{}", i), "user_1")
                    .await
                    .unwrap()
            });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.unwrap();
            assert_eq!(result.total_tasks, 10);
        }

        let calls = service.get_todo_calls();
        assert_eq!(calls.len(), 5);
    }
}
