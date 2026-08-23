//! Automation page for Cleanux

use crate::presentation::components::Card;
use crate::presentation::layouts::MainLayout;
use dioxus::prelude::*;

#[component]
pub fn Automation() -> Element {
    rsx! {
        MainLayout {
            title: "Automation",
            div {
                class: "space-y-6",
                div {
                    class: "flex justify-between items-center",
                    h2 {
                        class: "text-lg font-semibold text-gray-900 dark:text-white",
                        "Automation Recipes"
                    }
                    button {
                        class: "btn btn-primary",
                        "New Recipe"
                    }
                }

                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    RecipeCard {
                        title: "Weekly Cleanup",
                        description: "Automatically clean cache and trash every Sunday",
                        enabled: true,
                    }
                    RecipeCard {
                        title: "Daily Memory Optimize",
                        description: "Optimize memory every day at 3 AM",
                        enabled: true,
                    }
                    RecipeCard {
                        title: "Log Rotation",
                        description: "Clean old logs every Monday",
                        enabled: false,
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct RecipeCardProps {
    title: String,
    description: String,
    enabled: bool,
}

#[component]
fn RecipeCard(props: RecipeCardProps) -> Element {
    rsx! {
        Card {
            title: Some(props.title.clone()),
            div {
                class: "space-y-4",
                p {
                    class: "text-gray-600 dark:text-gray-400 text-sm",
                    "{props.description}"
                }
                div {
                    class: "flex items-center justify-between",
                    span {
                        class: if props.enabled { "text-green-600 text-sm font-medium" } else { "text-gray-500 text-sm font-medium" },
                        if props.enabled { "Enabled" } else { "Disabled" }
                    }
                    div {
                        class: "flex gap-2",
                        button {
                            class: "btn btn-ghost btn-sm",
                            "Edit"
                        }
                    }
                }
            }
        }
    }
}
