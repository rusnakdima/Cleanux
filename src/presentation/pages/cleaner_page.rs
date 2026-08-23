//! Cleaner page for Cleanux

use crate::presentation::components::Card;
use crate::presentation::layouts::MainLayout;
use dioxus::prelude::*;

#[component]
pub fn Cleaner() -> Element {
    rsx! {
        MainLayout {
            title: "System Cleaner",
            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                CleanerCard {
                    title: "Cache Cleaner",
                    description: "Remove application cached files",
                    size: "1.2 GB"
                }
                CleanerCard {
                    title: "Trash",
                    description: "Empty the trash bin",
                    size: "256 MB"
                }
                CleanerCard {
                    title: "Logs",
                    description: "Clean old log files",
                    size: "512 MB"
                }
                CleanerCard {
                    title: "System Temp",
                    description: "Remove temporary files",
                    size: "128 MB"
                }
                CleanerCard {
                    title: "Browser Cache",
                    description: "Clean browser cache files",
                    size: "756 MB"
                }
                CleanerCard {
                    title: "Package Cache",
                    description: "Remove package manager cache",
                    size: "1.8 GB"
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct CleanerCardProps {
    title: String,
    description: String,
    size: String,
}

#[component]
fn CleanerCard(props: CleanerCardProps) -> Element {
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
                        class: "text-2xl font-bold text-gray-900 dark:text-white",
                        "{props.size}"
                    }
                    button {
                        class: "btn btn-primary btn-sm",
                        "Clean"
                    }
                }
            }
        }
    }
}
