//! Files page — mirrors the template `data-page="files"` section.

use crate::app::AppState;
use crate::global_state::LargeFile;
use crate::global_state;
use dioxus::prelude::*;

#[component]
pub fn FilesPage(state: AppState) -> Element {
    let files = use_signal(|| global_state::get_large_files());
    let mut search_query = use_signal(|| String::new());

    let filtered: Vec<_> = {
        let q = search_query.read().to_lowercase();
        files.read().iter()
            .filter(|f| q.is_empty()
                || f.name.to_lowercase().contains(&q)
                || f.path.to_lowercase().contains(&q)
                || f.file_type.contains(&q))
            .cloned()
            .collect()
    };

    rsx! {
        section { "data-page": "files",
            class: "space-y-6",
            div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                div { class: "flex flex-col sm:flex-row sm:items-center gap-3 mb-4",
                    div { class: "flex-1",
                        h2 { class: "font-semibold", "Large Files" }
                        p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Drag to reorder priority · Click to inspect" }
                    }
                    div { class: "relative",
                        span { class: "material-symbols-rounded absolute left-3 top-1/2 -translate-y-1/2 text-zinc-400 text-lg", "search" }
                        input {
                            r#type: "text",
                            placeholder: "Search files…",
                            class: "pl-9 pr-3 py-2 rounded-lg bg-zinc-100 dark:bg-zinc-800 text-sm w-full sm:w-64 focus:outline-none focus:ring-2 focus:ring-cyan-500",
                            oninput: move |e| search_query.set(e.value().to_string()),
                        }
                    }
                }
                div { class: "space-y-2",
                    if filtered.is_empty() {
                        div { class: "p-8 text-center text-sm text-zinc-500", "No files match" }
                    } else {
                        for file in filtered.iter() {
                            LargeFileRow { file: file.clone() }
                        }
                    }
                }
                div { class: "mt-3 flex items-center gap-2 text-xs text-zinc-500 dark:text-zinc-400 bg-zinc-50 dark:bg-zinc-800/50 rounded-lg p-3",
                    span { class: "material-symbols-rounded text-base text-cyan-500", "drag_indicator" }
                    "Drag-drop hint: grab any file row and drop higher to flag it for removal. Reorder won't delete anything until you confirm."
                }
            }
        }
    }
}

#[component]
fn LargeFileRow(file: LargeFile) -> Element {
    let icon_map: std::collections::HashMap<&str, &str> = [
        ("archive", "folder_zip"),
        ("video", "movie"),
        ("installer", "download"),
        ("design", "palette"),
        ("data", "database"),
    ].into_iter().collect();
    let icon = icon_map.get(file.file_type.as_str()).copied().unwrap_or("description");

    rsx! {
        div {
            class: "flex items-center gap-3 p-3 rounded-xl border border-zinc-200 dark:border-zinc-800 hover:border-cyan-500 transition-colors cursor-grab active:cursor-grabbing",
            draggable: true,
            div { class: "w-9 h-9 rounded-lg bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center flex-shrink-0",
                span { class: "material-symbols-rounded text-zinc-600 dark:text-zinc-400", "{icon}" }
            }
            div { class: "flex-1 min-w-0",
                div { class: "text-sm font-medium truncate", "{file.name}" }
                div { class: "text-xs text-zinc-500 dark:text-zinc-400 truncate", "{file.path} · {file.age} old" }
            }
            div { class: "text-right flex-shrink-0",
                div { class: "text-sm font-semibold", "{file.size}" }
                button {
                    class: "text-xs text-rose-600 dark:text-rose-400 font-medium hover:underline",
                    onclick: move |_| { /* flag for removal */ },
                    "Remove"
                }
            }
        }
    }
}
