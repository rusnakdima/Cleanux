//! Profiles page for Cleanux

use crate::presentation::components::Card;
use crate::presentation::layouts::MainLayout;
use dioxus::prelude::*;

#[component]
pub fn Profiles() -> Element {
    rsx! {
        MainLayout {
            title: "Cleaning Profiles",
            div {
                class: "space-y-6",
                div {
                    class: "flex justify-between items-center",
                    p {
                        class: "text-gray-600 dark:text-gray-400",
                        "Create and manage cleaning profiles"
                    }
                    button {
                        class: "btn btn-primary",
                        "New Profile"
                    }
                }

                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    ProfileCard {
                        name: "Quick Clean",
                        description: "Basic cleanup for regular maintenance",
                        actions: "Cache, Temp files",
                    }
                    ProfileCard {
                        name: "Deep Clean",
                        description: "Thorough cleanup including logs",
                        actions: "Cache, Trash, Logs, Large files",
                    }
                    ProfileCard {
                        name: "Developer",
                        description: "Clean dev caches and build artifacts",
                        actions: "NPM, Cargo, .cache",
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct ProfileCardProps {
    name: String,
    description: String,
    actions: String,
}

#[component]
fn ProfileCard(props: ProfileCardProps) -> Element {
    rsx! {
        Card {
            title: Some(props.name.clone()),
            div {
                class: "space-y-3",
                p {
                    class: "text-gray-600 dark:text-gray-400 text-sm",
                    "{props.description}"
                }
                div {
                    class: "text-xs text-gray-500 dark:text-gray-500",
                    "Cleans: {props.actions}"
                }
                div {
                    class: "flex gap-2",
                    button {
                        class: "btn btn-primary btn-sm flex-1",
                        "Run"
                    }
                    button {
                        class: "btn btn-ghost btn-sm",
                        "Edit"
                    }
                }
            }
        }
    }
}
