//! ssr_dump — standalone test that SSR-renders each Cleanux page and prints HTML to stdout.
//!
//! Run with: cargo test -p cleanux --test ssr_dump -- --nocapture

use cleanux::app::AppState;
use cleanux::app::Page;
use cleanux::presentation::pages::{CleanPage, DashboardPage, FilesPage, PowerPage, SettingsPage};
use dioxus::prelude::*;
use dioxus_ssr::render_element;

#[component]
fn PageHarness(page: Page) -> Element {
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

fn ssr_page(route: &str) -> String {
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

#[test]
fn ssr_dump_dashboard() {
  println!("{}", "=".repeat(80));
  println!("// PAGE: dashboard");
  println!("{}", "=".repeat(80));
  println!("{}", ssr_page("dashboard"));
}

#[test]
fn ssr_dump_clean() {
  println!("{}", "=".repeat(80));
  println!("// PAGE: clean");
  println!("{}", "=".repeat(80));
  println!("{}", ssr_page("clean"));
}

#[test]
fn ssr_dump_files() {
  println!("{}", "=".repeat(80));
  println!("// PAGE: files");
  println!("{}", "=".repeat(80));
  println!("{}", ssr_page("files"));
}

#[test]
fn ssr_dump_power() {
  println!("{}", "=".repeat(80));
  println!("// PAGE: power");
  println!("{}", "=".repeat(80));
  println!("{}", ssr_page("power"));
}

#[test]
fn ssr_dump_settings() {
  println!("{}", "=".repeat(80));
  println!("// PAGE: settings");
  println!("{}", "=".repeat(80));
  println!("{}", ssr_page("settings"));
}
