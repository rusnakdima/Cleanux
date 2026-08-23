//! Page components for Cleanux
//!
//! Top-level page components for each route.

pub mod automation_page;
pub mod cleaner_page;
pub mod dashboard_page;
pub mod home_page;
pub mod logs_page;
pub mod profiles_page;
pub mod reports_page;
pub mod settings_page;
pub mod storage_page;
pub mod system_page;

// Re-export page components
pub use automation_page::Automation;
pub use cleaner_page::Cleaner;
pub use dashboard_page::Dashboard;
pub use home_page::Home;
pub use logs_page::LogManager;
pub use profiles_page::Profiles;
pub use reports_page::Reports;
pub use settings_page::Settings;
pub use storage_page::Storage;
pub use system_page::System;
