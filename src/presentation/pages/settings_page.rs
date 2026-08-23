//! Settings page for Cleanux

use crate::presentation::layouts::MainLayout;
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        MainLayout {
            title: "Settings",
            div {
                class: "space-y-6 max-w-2xl",
                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Appearance"
                    }
                    div {
                        class: "space-y-4",
                        div {
                            class: "flex items-center justify-between",
                            label {
                                class: "text-gray-700 dark:text-gray-300",
                                "Dark Mode"
                            }
                            input {
                                r#type: "checkbox",
                                class: "toggle",
                            }
                        }
                    }
                }

                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Notifications"
                    }
                    div {
                        class: "space-y-4",
                        div {
                            class: "flex items-center justify-between",
                            label {
                                class: "text-gray-700 dark:text-gray-300",
                                "Cleanup Complete"
                            }
                            input {
                                r#type: "checkbox",
                                class: "toggle",
                            }
                        }
                        div {
                            class: "flex items-center justify-between",
                            label {
                                class: "text-gray-700 dark:text-gray-300",
                                "Automation Alerts"
                            }
                            input {
                                r#type: "checkbox",
                                class: "toggle",
                            }
                        }
                    }
                }

                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
                    h2 {
                        class: "text-lg font-semibold mb-4 text-gray-900 dark:text-white",
                        "Data"
                    }
                    div {
                        class: "space-y-3",
                        button {
                            class: "btn btn-secondary w-full justify-center",
                            "Export Settings"
                        }
                        button {
                            class: "btn btn-secondary w-full justify-center",
                            "Import Settings"
                        }
                        button {
                            class: "btn btn-outline w-full justify-center text-red-600",
                            "Reset to Defaults"
                        }
                    }
                }
            }
        }
    }
}
