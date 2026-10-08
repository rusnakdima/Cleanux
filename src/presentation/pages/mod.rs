//! Hardcoded RSX pages mirroring `templates/cleanux.html` sections.
//!
//! Each page renders a `<section data-page="…">` root so the parity
//! verifier can diff it 1:1 against the matching template section.

mod app_residue_cleaner_page;
mod backup_page;
mod clean_page;
mod clipboard_page;
mod container_cleaner_page;
mod dashboard_page;
mod dev_cleaner_page;
mod disk_usage_page;
mod downloads_page;
mod duplicate_finder_page;
mod files_page;
mod kernel_cleaner_page;
mod log_manager_page;
mod media_cleaner_page;
mod memory_optimizer_page;
mod package_deep_clean_page;
mod power_page;
mod processes_page;
mod reports_page;
mod settings_page;
mod startup_page;
mod system_repair_page;

pub use app_residue_cleaner_page::AppResidueCleanerPage;
pub use backup_page::BackupPage;
pub use clean_page::CleanPage;
pub use clipboard_page::ClipboardPage;
pub use container_cleaner_page::ContainerCleanerPage;
pub use dashboard_page::DashboardPage;
pub use dev_cleaner_page::DevCleanerPage;
pub use disk_usage_page::DiskUsagePage;
pub use downloads_page::DownloadsPage;
pub use duplicate_finder_page::DuplicateFinderPage;
pub use files_page::FilesPage;
pub use kernel_cleaner_page::KernelCleanerPage;
pub use log_manager_page::LogManagerPage;
pub use media_cleaner_page::MediaCleanerPage;
pub use memory_optimizer_page::MemoryOptimizerPage;
pub use package_deep_clean_page::PackageDeepCleanPage;
pub use power_page::PowerPage;
pub use processes_page::ProcessesPage;
pub use reports_page::ReportsPage;
pub use settings_page::SettingsPage;
pub use startup_page::StartupPage;
pub use system_repair_page::SystemRepairPage;

use crate::app::AppState;
use dioxus::prelude::*;

/// App shell wrapping all pages — sidebar + topbar + mobile nav.
#[allow(unused)]
pub fn root_app(state: AppState) -> Element {
  rsx! { div { "RootApp placeholder" } }
}
