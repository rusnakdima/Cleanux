//! Card component for Cleanux

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    pub title: Option<String>,
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700",
            if let Some(title) = props.title {
                div {
                    class: "px-4 py-3 border-b border-gray-200 dark:border-gray-700",
                    h3 {
                        class: "text-lg font-semibold text-gray-900 dark:text-white",
                        "{title}"
                    }
                }
            }
            div {
                class: "p-4",
                {props.children}
            }
        }
    }
}
