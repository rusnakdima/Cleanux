//! App bootstrap: desktop launch, shared UI state, routing.
//!
//! Self-contained: no workspace library dependencies. Pages are hardcoded RSX
//! in `presentation::pages`. Every mutation mirrors into `global_state`.

use crate::global_state;
use dioxus::prelude::*;
use dioxus_shared::themes::{ThemeMode, ThemeVariant};
use dioxus_shared::ui::components::AppShell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    Clean,
    Files,
    Power,
    Settings,
}

impl Page {
    pub fn label(&self) -> &'static str {
        match self {
            Page::Dashboard => "Dashboard",
            Page::Clean => "Clean",
            Page::Files => "Files",
            Page::Power => "Power",
            Page::Settings => "Settings",
        }
    }

    pub fn sub(&self) -> &'static str {
        match self {
            Page::Dashboard => "System overview & cleanup status",
            Page::Clean => "Run scans and clear junk",
            Page::Files => "Large file management",
            Page::Power => "System power actions & schedules",
            Page::Settings => "Preferences & configuration",
        }
    }
}

/// UI state shared by all pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppState {
    pub page: Signal<Page>,
    pub dark_mode: Signal<bool>,
}

impl AppState {
    pub fn in_component() -> Self {
        Self {
            page: use_signal(|| Page::Dashboard),
            dark_mode: use_signal(|| true),
        }
    }

    pub fn navigate(&mut self, page: Page) {
        self.page.set(page);
    }

    pub fn toggle_theme(&mut self) {
        let new = !*self.dark_mode.read();
        self.dark_mode.set(new);
        global_state::set_dark_mode(new);
    }
}

/// Desktop launch.
pub fn run() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus_desktop::Config::new()
                .with_window(
                    dioxus_desktop::WindowBuilder::new()
                        .with_title("Cleanux")
                        .with_inner_size(dioxus_desktop::LogicalSize::new(1200.0, 800.0)),
                )
                .with_custom_head(
                    concat!(
                        r#"<link rel="preconnect" href="https://fonts.googleapis.com" />
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet" />
<link href="https://fonts.googleapis.com/icon?family=Material+Symbols+Rounded" rel="stylesheet" />
<style>
"#,
                        include_str!("../../dioxus-shared/assets/dist.css"),
                        r#"</style>"#
                    ).to_string(),
                ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let state = AppState::in_component();

    rsx! {
        AppShell {
            initial_mode: ThemeMode::Dark,
            initial_variant: ThemeVariant::MaterialDesign3,
            style { {include_str!("../assets/app.css")} }
            div {
                class: "min-h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-100 font-sans antialiased overflow-x-hidden",
                AppLayout { state }
            }
        }
    }
}

#[component]
fn AppLayout(state: AppState) -> Element {
    let current_page = state.page;
    let _dark = *state.dark_mode.read();

    rsx! {
        div {
            class: "fixed inset-0 z-40 bg-black/50 backdrop-blur-sm hidden lg:hidden",
            id: "drawer-overlay",
        }
        aside {
            class: "fixed inset-y-0 left-0 z-50 w-72 bg-white dark:bg-zinc-900 border-r border-zinc-200 dark:border-zinc-800 transform -translate-x-full lg:translate-x-0 transition-transform duration-200 ease-out flex flex-col lg:sticky lg:top-0 lg:h-screen",
            id: "sidebar",
            SidebarContent { state }
        }
        div {
            class: "flex-1 min-w-0 flex flex-col",
            header {
                class: "sticky top-0 z-30 h-16 bg-white/80 dark:bg-zinc-900/80 backdrop-blur-xl border-b border-zinc-200 dark:border-zinc-800 flex items-center px-4 lg:px-6 gap-3",
                Topbar { state }
            }
            main {
                class: "flex-1 p-4 lg:p-6 pb-24 lg:pb-6 max-w-full overflow-x-hidden",
                match *current_page.read() {
                    Page::Dashboard => rsx! { crate::presentation::pages::DashboardPage { state } },
                    Page::Clean    => rsx! { crate::presentation::pages::CleanPage { state } },
                    Page::Files    => rsx! { crate::presentation::pages::FilesPage { state } },
                    Page::Power    => rsx! { crate::presentation::pages::PowerPage { state } },
                    Page::Settings => rsx! { crate::presentation::pages::SettingsPage { state } },
                }
            }
        }
        MobileBottomNav { state }
    }
}

#[component]
fn SidebarContent(state: AppState) -> Element {
    let current_page = state.page;

    rsx! {
        div {
            class: "h-16 flex items-center justify-between px-5 border-b border-zinc-200 dark:border-zinc-800 flex-shrink-0",
            div { class: "flex items-center gap-2.5",
                div { class: "w-8 h-8 rounded-lg bg-gradient-to-br from-cyan-500 to-violet-500 flex items-center justify-center",
                    span { class: "material-symbols-rounded text-white text-lg", "cleaning_services" }
                }
                span { class: "font-bold text-lg tracking-tight", "Cleanux" }
            }
        }
        nav { class: "flex-1 p-3 space-y-1 overflow-y-auto",
            for (page, nav_id, label, icon) in [
                (Page::Dashboard, "dashboard", "Dashboard", "dashboard"),
                (Page::Clean,    "clean",    "Clean",     "cleaning_services"),
                (Page::Files,   "files",    "Files",     "folder"),
                (Page::Power,   "power",    "Power",     "bolt"),
                (Page::Settings,"settings", "Settings",  "settings"),
            ] {
                button {
                    class: format!(
                        "w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-colors {}",
                        if *current_page.read() == page {
                            "bg-zinc-100 dark:bg-zinc-800 text-zinc-900 dark:text-white"
                        } else {
                            "text-zinc-600 dark:text-zinc-400 hover:bg-zinc-100 dark:hover:bg-zinc-800"
                        }
                    ),
                    "data-nav": nav_id,
                    onclick: move |_| state.navigate(page),
                    span { class: "material-symbols-rounded text-xl", "{icon}" }
                    "{label}"
                }
            }
        }
        div { class: "p-3 border-t border-zinc-200 dark:border-zinc-800 flex-shrink-0",
            div { class: "flex items-center gap-3 px-3 py-2 rounded-xl bg-zinc-100 dark:bg-zinc-800",
                div { class: "w-8 h-8 rounded-full bg-gradient-to-br from-cyan-500 to-violet-500 flex items-center justify-center text-white text-sm font-semibold", "A" }
                div { class: "flex-1 min-w-0",
                    div { class: "text-sm font-medium truncate", "Admin" }
                    div { class: "text-xs text-zinc-500 dark:text-zinc-400 truncate", "System Online" }
                }
            }
        }
    }
}

#[component]
fn Topbar(state: AppState) -> Element {
    let current_page = *state.page.read();

    rsx! {
        button {
            class: "p-2 -ml-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800",
            span { class: "material-symbols-rounded", "menu" }
        }
        div { class: "flex-1 min-w-0",
            h1 { class: "text-lg font-semibold truncate", "{current_page.label()}" }
            p { class: "text-xs text-zinc-500 dark:text-zinc-400 truncate", "{current_page.sub()}" }
        }
        div { class: "hidden md:flex items-center gap-2 px-3 py-1.5 rounded-full bg-zinc-100 dark:bg-zinc-800 text-xs",
            span { class: "w-2 h-2 rounded-full bg-emerald-500", "" }
            span { class: "font-medium", "Healthy" }
        }
        button {
            class: "p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors",
            onclick: move |_| {
                // TODO(change/cleanux-master-completion): implement export report — opens a save dialog with the report text
            },
            span { class: "material-symbols-rounded", "file_download" }
        }
        button {
            class: "p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors",
            onclick: move |_| { let mut s = state; s.toggle_theme(); },
            span { class: "material-symbols-rounded", "dark_mode" }
        }
    }
}

#[component]
fn MobileBottomNav(state: AppState) -> Element {
    let current_page = state.page;

    rsx! {
        nav { class: "fixed bottom-0 inset-x-0 z-30 bg-white/90 dark:bg-zinc-900/90 backdrop-blur-xl border-t border-zinc-200 dark:border-zinc-800 lg:hidden",
            div { class: "grid grid-cols-5",
                for (page, nav_id, label, icon) in [
                    (Page::Dashboard,"dashboard","Dashboard","dashboard"),
                    (Page::Clean,   "clean",   "Clean",   "cleaning_services"),
                    (Page::Files,   "files",   "Files",   "folder"),
                    (Page::Power,   "power",   "Power",   "bolt"),
                    (Page::Settings,"settings","Settings","settings"),
                ] {
                    button {
                        class: format!(
                            "flex flex-col items-center gap-1 py-2.5 transition-colors {}",
                            if *current_page.read() == page {
                                "text-cyan-600 dark:text-cyan-400"
                            } else {
                                "text-zinc-500 dark:text-zinc-400"
                            }
                        ),
                        "data-mnav": nav_id,
                        onclick: move |_| state.navigate(page),
                        span { class: "material-symbols-rounded text-xl", "{icon}" }
                        span { class: "text-[10px] font-medium", "{label}" }
                    }
                }
            }
        }
    }
}
