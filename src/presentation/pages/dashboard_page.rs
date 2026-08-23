//! Dashboard page for Cleanux

use crate::presentation::components::StatCard;
use crate::presentation::layouts::MainLayout;
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        MainLayout {
            title: "Dashboard",
            div {
                class: "space-y-6",
                // Stats grid
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4",
                    StatCard {
                        label: "Memory Used",
                        value: "4.2 / 16 GB",
                        trend: Some("+0.5 GB".to_string())
                    }
                    StatCard {
                        label: "CPU Usage",
                        value: "23%",
                        trend: Some("-2%".to_string())
                    }
                    StatCard {
                        label: "Storage",
                        value: "256 / 512 GB",
                        trend: Some("50%".to_string())
                    }
                    StatCard {
                        label: "Temperature",
                        value: "45°C",
                        trend: Some("Normal".to_string())
                    }
                }

                // Quick actions
                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Quick Actions"
                    }
                    div {
                        class: "flex flex-wrap gap-3",
                        button {
                            class: "btn btn-primary",
                            onclick: |_| {},
                            "Optimize Memory"
                        }
                        button {
                            class: "btn btn-secondary",
                            onclick: |_| {},
                            "Scan System"
                        }
                        button {
                            class: "btn btn-outline",
                            onclick: |_| {},
                            "Refresh Stats"
                        }
                    }
                }

                // System status
                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "System Status"
                    }
                    div {
                        class: "space-y-3",
                        div {
                            class: "flex items-center justify-between",
                            span { class: "text-gray-600 dark:text-gray-400", "Last Cleanup" }
                            span { class: "font-medium text-gray-900 dark:text-white", "2 days ago" }
                        }
                        div {
                            class: "flex items-center justify-between",
                            span { class: "text-gray-600 dark:text-gray-400", "Active Recipes" }
                            span { class: "font-medium text-gray-900 dark:text-white", "3" }
                        }
                        div {
                            class: "flex items-center justify-between",
                            span { class: "text-gray-600 dark:text-gray-400", "Health Score" }
                            span { class: "font-medium text-green-600", "Good (85%)" }
                        }
                    }
                }
            }
        }
    }
}
