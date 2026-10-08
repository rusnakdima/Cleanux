//! Statistics service errors.

#[derive(Debug, thiserror::Error)]
pub enum StatisticsError {
    #[error("Todo not found: {0}")]
    TodoNotFound(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}
