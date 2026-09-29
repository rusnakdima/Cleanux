//! Hardcoded RSX pages mirroring `templates/cleanux.html` sections.
//!
//! Each page renders a `<section data-page="…">` root so the parity
//! verifier can diff it 1:1 against the matching template section.

mod dashboard_page;
mod clean_page;
mod files_page;
mod power_page;
mod settings_page;

pub use dashboard_page::DashboardPage;
pub use clean_page::CleanPage;
pub use files_page::FilesPage;
pub use power_page::PowerPage;
pub use settings_page::SettingsPage;

use crate::app::AppState;
use dioxus::prelude::*;

/// App shell wrapping all pages — sidebar + topbar + mobile nav.
#[allow(unused)]
pub fn root_app(state: AppState) -> Element {
    rsx! { div { "RootApp placeholder" } }
}
