//! Template parity tests: SSR-rendered RSX pages vs golden sections
//! in `templates/cleanux.html`.

use cleanux::app::Page;
use dioxus::prelude::*;
use std::path::{Path, PathBuf};

fn template_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("templates")
        .join("cleanux.html")
}

fn template() -> String {
    std::fs::read_to_string(template_path()).expect("templates/cleanux.html not found")
}

// ---------------------------------------------------------------------------
// SSR helpers — each test SSR-renders the relevant page directly
// ---------------------------------------------------------------------------

fn ssr_page(route: &str) -> String {
    use dioxus_ssr::render_element;

    let page = match route {
        "dashboard" => Page::Dashboard,
        "clean" => Page::Clean,
        "files" => Page::Files,
        "power" => Page::Power,
        "settings" => Page::Settings,
        _ => panic!("unknown route: {route}"),
    };
    render_element(rsx! { PageHarness { page } })
}

/// `Signal::new` must run inside a Dioxus runtime, so AppState is built in this
/// harness component — dioxus_ssr::render_element drives it inside its own
/// VirtualDom runtime (same pattern as Calculator's ParityHarness).
#[component]
fn PageHarness(page: Page) -> Element {
    use cleanux::app::AppState;
    use cleanux::presentation::pages::{
        CleanPage, DashboardPage, FilesPage, PowerPage, SettingsPage,
    };

    let state = AppState {
        page: Signal::new(page),
        dark_mode: Signal::new(true),
    };

    match page {
        Page::Dashboard => rsx! { DashboardPage { state } },
        Page::Clean => rsx! { CleanPage { state } },
        Page::Files => rsx! { FilesPage { state } },
        Page::Power => rsx! { PowerPage { state } },
        Page::Settings => rsx! { SettingsPage { state } },
    }
}

// ---------------------------------------------------------------------------
// Verification engine
// ---------------------------------------------------------------------------

use cleanux::verification::{compare_html, extract_page_section, CompareOptions};

fn parity(route: &str) -> cleanux::verification::Report {
    let mut opts = CompareOptions::template_mode();
    opts.ignore_attrs
        .extend(["onclick", "oninput", "onchange", "ondragstart", "ondrop",
                 "ondragover", "ondragleave", "onkeydown", "checked", "value",
                 "selected", "disabled", "data-nav", "data-mnav", "data-page",
                 "data-device", "id", "for", "aria-label", "aria-expanded",
                 "aria-controls", "draggable", "style", "step", "min", "max",
                 "placeholder", "type"].map(String::from));
    let golden = extract_page_section(&template(), route)
        .unwrap_or_else(|| panic!("no section for route {route}"));
    let actual = ssr_page(route);

    let golden = golden
        .replace(" class=\"page ", " class=\"")
        .replace(" class=\"page\"", " class=\"\"")
        .replace(" page", "")
        .replace("hidden ", "")
        .replace(" hidden", "");

    compare_html(&golden, &actual, &opts)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

fn assert_parity(route: &str) {
    let r = parity(route);
    println!(
        "{route}: score={:.3} verdict={}",
        r.score,
        r.verdict.label()
    );
    if !r.entries.is_empty() {
        for entry in r.entries.iter().take(10) {
            println!("  {entry:?}");
        }
    }
    assert!(r.score >= 0.85, "{route} score {} < 0.85", r.score);
}

#[test]
fn test_dashboard_parity() {
    assert_parity("dashboard");
}

#[test]
fn test_clean_parity() {
    assert_parity("clean");
}

#[test]
fn test_files_parity() {
    assert_parity("files");
}

#[test]
fn test_power_parity() {
    assert_parity("power");
}

#[test]
fn test_settings_parity() {
    assert_parity("settings");
}
