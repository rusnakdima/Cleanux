//! Log Manager page for Cleanux

use crate::presentation::layouts::MainLayout;
use dioxus::prelude::*;

#[component]
pub fn LogManager() -> Element {
    rsx! {
        MainLayout {
            title: "Log Manager",
            div {
                class: "space-y-6",
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-4 border border-gray-200 dark:border-gray-700",
                        p {
                            class: "text-sm text-gray-600 dark:text-gray-400",
                            "Journal Size"
                        }
                        p {
                            class: "text-2xl font-bold text-gray-900 dark:text-white",
                            "1.2 GB"
                        }
                    }
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-4 border border-gray-200 dark:border-gray-700",
                        p {
                            class: "text-sm text-gray-600 dark:text-gray-400",
                            "Rotated Logs"
                        }
                        p {
                            class: "text-2xl font-bold text-gray-900 dark:text-white",
                            "856 MB"
                        }
                    }
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-4 border border-gray-200 dark:border-gray-700",
                        p {
                            class: "text-sm text-gray-600 dark:text-gray-400",
                            "Largest Log"
                        }
                        p {
                            class: "text-2xl font-bold text-gray-900 dark:text-white",
                            "syslog (234 MB)"
                        }
                    }
                }

                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Log Management"
                    }
                    div {
                        class: "space-y-4",
                        button {
                            class: "btn btn-secondary w-full justify-center",
                            "Vacuum Journal (keep last 7 days)"
                        }
                        button {
                            class: "btn btn-secondary w-full justify-center",
                            "Clean Rotated Logs"
                        }
                        button {
                            class: "btn btn-outline w-full justify-center",
                            "Analyze Logrotate Configs"
                        }
                    }
                }

                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Logrotate Configuration"
                    }
                    div {
                        class: "space-y-2 text-sm font-mono bg-gray-100 dark:bg-gray-900 p-4 rounded",
                        code { "etc/logrotate.conf" }
                        code { "etc/logrotate.d/" }
                    }
                }
            }
        }
    }
}
