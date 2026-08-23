//! Reports page for Cleanux

use crate::presentation::layouts::MainLayout;
use dioxus::prelude::*;

#[component]
pub fn Reports() -> Element {
    rsx! {
        MainLayout {
            title: "Cleaning Reports",
            div {
                class: "space-y-6",
                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Recent Cleanups"
                    }
                    div {
                        class: "overflow-x-auto",
                        table {
                            class: "w-full text-sm text-left",
                            thead {
                                tr {
                                    th { class: "px-4 py-2 text-gray-600 dark:text-gray-400", "Date" }
                                    th { class: "px-4 py-2 text-gray-600 dark:text-gray-400", "Items" }
                                    th { class: "px-4 py-2 text-gray-600 dark:text-gray-400", "Space Reclaimed" }
                                    th { class: "px-4 py-2 text-gray-600 dark:text-gray-400", "Duration" }
                                }
                            }
                            tbody {
                                tr {
                                    td { class: "px-4 py-2 text-gray-900 dark:text-white", "Aug 12, 2026" }
                                    td { class: "px-4 py-2 text-gray-900 dark:text-white", "1,234" }
                                    td { class: "px-4 py-2 text-gray-900 dark:text-white", "2.3 GB" }
                                    td { class: "px-4 py-2 text-gray-900 dark:text-white", "45s" }
                                }
                                tr {
                                    td { class: "px-4 py-2 text-gray-900 dark:text-white", "Aug 10, 2026" }
                                    td { class: "px-4 py-2 text-gray-900 dark:text-white", "856" }
                                    td { class: "px-4 py-2 text-gray-900 dark:text-white", "1.8 GB" }
                                    td { class: "px-4 py-2 text-gray-900 dark:text-white", "32s" }
                                }
                                tr {
                                    td { class: "px-4 py-2 text-gray-900 dark:text-white", "Aug 8, 2026" }
                                    td { class: "px-4 py-2 text-gray-900 dark:text-white", "2,156" }
                                    td { class: "px-4 py-2 text-gray-900 dark:text-white", "4.1 GB" }
                                    td { class: "px-4 py-2 text-gray-900 dark:text-white", "78s" }
                                }
                            }
                        }
                    }
                }

                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Statistics"
                    }
                    div {
                        class: "grid grid-cols-3 gap-4 text-center",
                        div {
                            div {
                                class: "text-3xl font-bold text-primary-600",
                                "15.2 GB"
                            }
                            div {
                                class: "text-sm text-gray-600 dark:text-gray-400",
                                "Total Reclaimed"
                            }
                        }
                        div {
                            div {
                                class: "text-3xl font-bold text-primary-600",
                                "47"
                            }
                            div {
                                class: "text-sm text-gray-600 dark:text-gray-400",
                                "Total Cleanups"
                            }
                        }
                        div {
                            div {
                                class: "text-3xl font-bold text-primary-600",
                                "324 MB"
                            }
                            div {
                                class: "text-sm text-gray-600 dark:text-gray-400",
                                "Daily Average"
                            }
                        }
                    }
                }
            }
        }
    }
}
