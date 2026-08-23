//! Main layout component for Cleanux
//!
//! Provides the primary application shell with navigation.

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct MainLayoutProps {
    pub title: String,
    pub children: Element,
}

#[component]
pub fn MainLayout(props: MainLayoutProps) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-50 dark:bg-gray-900",
            // Header
            div {
                class: "bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700",
                div {
                    class: "flex items-center justify-between px-6 py-4",
                    div {
                        class: "flex items-center gap-3",
                        span {
                            class: "text-2xl font-bold text-primary-600",
                            "Cleanux"
                        }
                    }
                    div {
                        class: "flex items-center gap-2",
                        button {
                            class: "btn btn-ghost btn-sm",
                            onclick: |_| {
                                // Toggle theme
                            },
                            "Theme"
                        }
                    }
                }
            }
            // Navigation tabs
            div {
                class: "bg-gray-100 dark:bg-gray-800 px-4 py-2",
                nav {
                    class: "flex gap-2",
                    NavButton { label: "Dashboard", route: "/" }
                    NavButton { label: "Cleaner", route: "/cleaner" }
                    NavButton { label: "Storage", route: "/storage" }
                    NavButton { label: "System", route: "/system" }
                    NavButton { label: "Automation", route: "/automation" }
                    NavButton { label: "Logs", route: "/logs" }
                    NavButton { label: "Settings", route: "/settings" }
                }
            }
            // Main content
            main {
                class: "p-6",
                div {
                    class: "max-w-7xl mx-auto",
                    h1 {
                        class: "text-2xl font-bold mb-6 text-gray-900 dark:text-white",
                        "{props.title}"
                    }
                    {props.children}
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct NavButtonProps {
    pub label: String,
    pub route: String,
}

#[component]
fn NavButton(props: NavButtonProps) -> Element {
    rsx! {
        Link {
            to: props.route,
            class: "px-4 py-2 rounded-md text-sm font-medium transition-colors \
                   bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-200 \
                   hover:bg-gray-50 dark:hover:bg-gray-600",
            "{props.label}"
        }
    }
}
