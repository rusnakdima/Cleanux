//! System page for Cleanux

use crate::presentation::layouts::MainLayout;
use dioxus::prelude::*;

#[component]
pub fn System() -> Element {
    rsx! {
        MainLayout {
            title: "System Information",
            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Kernel"
                    }
                    div {
                        class: "space-y-3 text-sm",
                        div {
                            class: "flex justify-between",
                            span { class: "text-gray-600 dark:text-gray-400", "Current Kernel" }
                            span { class: "font-medium text-gray-900 dark:text-white", "6.8.0-45-generic" }
                        }
                        button {
                            class: "btn btn-secondary btn-sm mt-2",
                            "Clean Old Kernels"
                        }
                    }
                }

                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Services"
                    }
                    div {
                        class: "space-y-3 text-sm",
                        div {
                            class: "flex justify-between items-center",
                            span { class: "text-gray-600 dark:text-gray-400", "Running Services" }
                            span { class: "font-medium text-green-600", "42" }
                        }
                        div {
                            class: "flex justify-between items-center",
                            span { class: "text-gray-600 dark:text-gray-400", "Stopped Services" }
                            span { class: "font-medium text-gray-900 dark:text-white", "12" }
                        }
                        button {
                            class: "btn btn-secondary btn-sm mt-2",
                            "Manage Services"
                        }
                    }
                }

                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Processes"
                    }
                    div {
                        class: "space-y-3 text-sm",
                        div {
                            class: "flex justify-between",
                            span { class: "text-gray-600 dark:text-gray-400", "Total Processes" }
                            span { class: "font-medium text-gray-900 dark:text-white", "247" }
                        }
                        div {
                            class: "flex justify-between",
                            span { class: "text-gray-600 dark:text-gray-400", "Running" }
                            span { class: "font-medium text-green-600", "183" }
                        }
                        button {
                            class: "btn btn-secondary btn-sm mt-2",
                            "View Processes"
                        }
                    }
                }

                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Power"
                    }
                    div {
                        class: "space-y-3 text-sm",
                        div {
                            class: "flex justify-between",
                            span { class: "text-gray-600 dark:text-gray-400", "Power Profile" }
                            span { class: "font-medium text-gray-900 dark:text-white", "Balanced" }
                        }
                        div {
                            class: "flex justify-between",
                            span { class: "text-gray-600 dark:text-gray-400", "Battery" }
                            span { class: "font-medium text-gray-900 dark:text-white", "87%" }
                        }
                        button {
                            class: "btn btn-secondary btn-sm mt-2",
                            "Power Settings"
                        }
                    }
                }
            }
        }
    }
}
