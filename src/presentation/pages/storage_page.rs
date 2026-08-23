//! Storage page for Cleanux

use crate::presentation::layouts::MainLayout;
use dioxus::prelude::*;

#[component]
pub fn Storage() -> Element {
    rsx! {
        MainLayout {
            title: "Storage Management",
            div {
                class: "space-y-6",
                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Disk Usage"
                    }
                    div {
                        class: "w-full bg-gray-200 dark:bg-gray-700 rounded-full h-4",
                        div {
                            class: "bg-cyan-500 h-4 rounded-full",
                            style: "width: 65%",
                        }
                    }
                    div {
                        class: "flex justify-between mt-2 text-sm text-gray-600 dark:text-gray-400",
                        span { "Used: 332 GB" }
                        span { "Total: 512 GB" }
                    }
                }

                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                        h3 {
                            class: "font-semibold mb-3 text-gray-900 dark:text-white",
                            "Largest Directories"
                        }
                        div {
                            class: "space-y-2 text-sm",
                            div { class: "flex justify-between", span { "/home" }, span { "120 GB" } }
                            div { class: "flex justify-between", span { "/var" }, span { "45 GB" } }
                            div { class: "flex justify-between", span { "/opt" }, span { "32 GB" } }
                        }
                    }
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                        h3 {
                            class: "font-semibold mb-3 text-gray-900 dark:text-white",
                            "Duplicate Files"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-400 text-sm",
                            "Found 24 duplicate files (1.2 GB)"
                        }
                        button {
                            class: "btn btn-secondary btn-sm mt-3",
                            "Review Duplicates"
                        }
                    }
                }
            }
        }
    }
}
