//! Header component for Cleanux

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct HeaderProps {
    pub title: String,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    rsx! {
        header {
            class: "bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700",
            div {
                class: "flex items-center justify-between px-6 py-4",
                div {
                    class: "flex items-center gap-3",
                    span {
                        class: "text-2xl font-bold text-primary-600",
                        "{props.title}"
                    }
                }
                div {
                    class: "flex items-center gap-2",
                    button {
                        class: "btn btn-ghost btn-sm",
                        "Theme"
                    }
                }
            }
        }
    }
}
