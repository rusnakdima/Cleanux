//! NavButton component for navigation

use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NavButtonProps {
    pub label: String,
    pub route: String,
}

#[component]
pub fn NavButton(props: NavButtonProps) -> Element {
    rsx! {
        Link {
            to: props.route,
            class: "px-4 py-2 rounded-md text-sm font-medium transition-colors \
                   bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-200 \
                   hover:bg-gray-50 dark:hover:bg-gray-600 border border-gray-200 dark:border-gray-600",
            "{props.label}"
        }
    }
}
