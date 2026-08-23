//! StatCard component for displaying metrics

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StatCardProps {
    pub label: String,
    pub value: String,
    pub trend: Option<String>,
}

#[component]
pub fn StatCard(props: StatCardProps) -> Element {
    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-4 border border-gray-200 dark:border-gray-700",
            div {
                class: "flex items-center justify-between",
                div {
                    p {
                        class: "text-sm text-gray-500 dark:text-gray-400",
                        "{props.label}"
                    }
                    p {
                        class: "text-2xl font-bold text-gray-900 dark:text-white mt-1",
                        "{props.value}"
                    }
                }
                if let Some(trend) = props.trend {
                    div {
                        class: "text-sm font-medium text-green-600",
                        "{trend}"
                    }
                }
            }
        }
    }
}
