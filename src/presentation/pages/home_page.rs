//! Home page for Cleanux

use crate::presentation::layouts::MainLayout;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        MainLayout {
            title: "Welcome to Cleanux",
            div {
                class: "text-center py-20",
                h1 {
                    class: "text-4xl font-bold mb-4 text-gray-900 dark:text-white",
                    "Welcome to Cleanux"
                }
                p {
                    class: "text-gray-600 dark:text-gray-400 mb-8 max-w-md mx-auto",
                    "Your system cleanup and optimization tool"
                }

                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-6 max-w-4xl mx-auto mt-12",
                    FeatureCard {
                        title: "Dashboard",
                        description: "View system health and quick actions",
                        link_to: crate::Route::Dashboard {},
                    }
                    FeatureCard {
                        title: "Cleaner",
                        description: "Clean cache, trash, and system junk",
                        link_to: crate::Route::Cleaner {},
                    }
                    FeatureCard {
                        title: "Automation",
                        description: "Set up automated cleaning recipes",
                        link_to: crate::Route::Automation {},
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct FeatureCardProps {
    title: String,
    description: String,
    link_to: crate::Route,
}

#[component]
fn FeatureCard(props: FeatureCardProps) -> Element {
    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow border border-gray-200 dark:border-gray-700",
            h3 {
                class: "text-xl font-semibold mb-2 text-gray-900 dark:text-white",
                "{props.title}"
            }
            p {
                class: "text-gray-600 dark:text-gray-400 mb-4",
                "{props.description}"
            }
            Link {
                to: props.link_to,
                class: "text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 font-medium",
                "Go →"
            }
        }
    }
}
