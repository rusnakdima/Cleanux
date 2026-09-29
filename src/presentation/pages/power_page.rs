//! Power page — mirrors the template `data-page="power"` section.

use crate::app::AppState;
use crate::bridge;
use crate::global_state::{self, PowerAction, ScheduleEntry};
use dioxus::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PowerProfileInfo {
    pub current: String,
    pub available: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ThermalInfo {
    pub zone: String,
    pub temp_c: f64,
    pub zone_type: String,
}

#[component]
pub fn PowerPage(state: AppState) -> Element {
    let power_actions = global_state::get_power_actions();
    let mut schedules = use_signal(|| global_state::get_schedules());
    let mut profiles = use_signal(|| Option::<PowerProfileInfo>::None);
    let thermals = use_signal(|| Vec::<ThermalInfo>::new());
    let mut profile_setting = use_signal(|| Option::<String>::None);

    // Load power profiles on mount
    use_effect(move || {
        let mut thermals_clone = thermals.clone();
        // bridge::invoke_app_command is synchronous (block_on inside)
        if let Ok(val) = bridge::invoke_app_command("power_profiles", &serde_json::json!({})) {
            if let Ok(info) = serde_json::from_value::<PowerProfileInfo>(val) {
                profiles.set(Some(info));
            }
        }
        if let Ok(val) = bridge::invoke_app_command("thermal_info", &serde_json::json!({})) {
            if let Ok(info) = serde_json::from_value::<Vec<ThermalInfo>>(val) {
                thermals_clone.set(info);
            }
        }
    });

    // Extract owned values before rsx block to avoid borrow issues
    let current_profile_display = profiles
        .read()
        .as_ref()
        .map(|p| p.current.clone())
        .unwrap_or_else(|| "loading...".to_string());
    let available_profiles: Vec<String> = profiles
        .read()
        .as_ref()
        .map(|p| p.available.clone())
        .unwrap_or_default();
    let active_profile = profiles
        .read()
        .as_ref()
        .map(|p| p.current.clone())
        .unwrap_or_default();

    rsx! {
        section { "data-page": "power",
            class: "space-y-6",

            // Power profile selector
            div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                div { class: "flex items-center justify-between mb-1",
                    h2 { class: "font-semibold", "Power Profile" }
                    span { class: "text-xs font-medium px-2 py-0.5 rounded-full bg-cyan-100 dark:bg-cyan-900/30 text-cyan-700 dark:text-cyan-400",
                        "{current_profile_display}"
                    }
                }
                p { class: "text-xs text-zinc-500 dark:text-zinc-400 mb-4", "Adjust CPU performance and power consumption." }
                div { class: "flex flex-wrap gap-2",
                    { available_profiles.iter().map(|profile| {
                        // MUST be fully owned String, not &String, for 'static closure
                        let profile_name: String = (*profile).to_string();
                        let is_active = active_profile == *profile;
                        let is_setting = profile_setting.read().as_ref().is_some_and(|s| s == profile);
                        let icon_name = match profile.as_str() {
                            "performance" => "bolt",
                            "power-saver" => "battery_saver",
                            _ => "balanced",
                        };
                        let active_class = if is_active {
                            "border-cyan-500 bg-cyan-50 dark:bg-cyan-900/30 text-cyan-700 dark:text-cyan-300"
                        } else {
                            "border-zinc-200 dark:border-zinc-700 hover:border-cyan-400 text-zinc-600 dark:text-zinc-300"
                        };
                        let btn_class = format!("flex-1 min-w-[100px] px-3 py-2 rounded-xl border text-sm font-medium transition-all {}", active_class);
                        rsx! {
                            button {
                                key: "{profile_name}",
                                class: "{btn_class}",
                                disabled: is_setting,
                                onclick: move |_| {
                                    let mut thermals_clone = thermals.clone();
                                    let mut profiles_clone = profiles.clone();
                                    let pn = profile_name.clone();
                                    profile_setting.set(Some(pn.clone()));
                                    async move {
                                        let _ = bridge::invoke_app_command("power_profile_set", &serde_json::json!({ "profile": &pn }));
                                        if let Ok(val) = bridge::invoke_app_command("power_profiles", &serde_json::json!({})) {
                                            if let Ok(info) = serde_json::from_value::<PowerProfileInfo>(val) {
                                                profiles_clone.set(Some(info));
                                            }
                                        }
                                        if let Ok(val) = bridge::invoke_app_command("thermal_info", &serde_json::json!({})) {
                                            if let Ok(info) = serde_json::from_value::<Vec<ThermalInfo>>(val) {
                                                thermals_clone.set(info);
                                            }
                                        }
                                        profile_setting.set(None);
                                    }
                                },
                                if is_setting {
                                    span { class: "material-symbols-rounded text-base animate-spin", "progress_activity" }
                                } else {
                                    span { class: "material-symbols-rounded text-base", "{icon_name}" }
                                }
                                span { class: "capitalize", "{profile}" }
                            }
                        }
                    })}
                }
            }

            // Thermal monitoring
            div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                div { class: "flex items-center justify-between mb-1",
                    h2 { class: "font-semibold", "Thermal Zones" }
                    button {
                        class: "text-xs text-zinc-400 hover:text-cyan-500 transition-colors flex items-center gap-1",
                        onclick: move |_| {
                            let mut thermals_clone = thermals.clone();
                            async move {
                                if let Ok(val) = bridge::invoke_app_command("thermal_info", &serde_json::json!({})) {
                                    if let Ok(info) = serde_json::from_value::<Vec<ThermalInfo>>(val) {
                                        thermals_clone.set(info);
                                    }
                                }
                            }
                        },
                        span { class: "material-symbols-rounded text-sm", "refresh" }
                        "Refresh"
                    }
                }
                p { class: "text-xs text-zinc-500 dark:text-zinc-400 mb-4", "Real-time temperature readings from system sensors." }
                div { class: "space-y-2",
                    if thermals.read().is_empty() {
                        div { class: "text-sm text-zinc-400 text-center py-4", "No thermal data available" }
                    } else {
                        for thermal in thermals.read().iter() {
                            ThermalRow { thermal: thermal.clone() }
                        }
                    }
                }
            }

            // Power actions grid
            div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                h2 { class: "font-semibold mb-1", "Power Actions" }
                p { class: "text-xs text-zinc-500 dark:text-zinc-400 mb-4", "Each action will ask for confirmation before executing." }
                div { class: "grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-3",
                    for action in power_actions {
                        PowerActionCard { action }
                    }
                }
            }

            // Scheduled cleanup
            div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                h2 { class: "font-semibold mb-3", "Scheduled Cleanup" }
                div { class: "space-y-2",
                    for schedule in schedules.read().iter().cloned() {
                        ScheduleRow {
                            schedule: schedule,
                            on_toggle: move |(id, enabled): (String, bool)| {
                                global_state::toggle_schedule(&id, enabled);
                                drop(schedules);
                            },
                            on_edit: move |s: ScheduleEntry| {
                                let _ = bridge::invoke_app_command("schedule_edit", &serde_json::json!({
                                    "id": s.id,
                                    "name": s.name,
                                    "frequency": s.frequency,
                                    "time": s.time,
                                    "categories": s.categories,
                                }));
                            },
                            on_delete: move |id: String| {
                                let _ = bridge::invoke_app_command("schedule_delete", &serde_json::json!({ "id": id }));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
struct ThermalRowProps {
    thermal: ThermalInfo,
}

#[component]
fn ThermalRow(props: ThermalRowProps) -> Element {
    let temp = props.thermal.temp_c;
    let (temp_class, icon) = if temp >= 80.0 {
        ("text-rose-500", "warning")
    } else if temp >= 60.0 {
        ("text-amber-500", "thermometer")
    } else {
        ("text-emerald-500", "thermostat")
    };

    rsx! {
        div { class: "flex items-center gap-3 p-3 rounded-xl border border-zinc-200 dark:border-zinc-800",
            div { class: "w-9 h-9 rounded-lg bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center flex-shrink-0",
                span { class: "material-symbols-rounded {temp_class} text-lg", "{icon}" }
            }
            div { class: "flex-1 min-w-0",
                div { class: "text-sm font-medium truncate", "{props.thermal.zone_type}" }
                div { class: "text-xs text-zinc-500 dark:text-zinc-400", "{props.thermal.zone}" }
            }
            div { class: "flex-shrink-0",
                span { class: "text-lg font-semibold {temp_class}", "{temp:.1}°C" }
            }
        }
    }
}

#[component]
fn PowerActionCard(action: PowerAction) -> Element {
    let mut show_confirm = use_signal(|| false);

    let color_map: std::collections::HashMap<&str, (&str, &str)> = [
        (
            "rose",
            (
                "bg-rose-100 dark:bg-rose-900/30",
                "text-rose-600 dark:text-rose-400",
            ),
        ),
        (
            "amber",
            (
                "bg-amber-100 dark:bg-amber-900/30",
                "text-amber-600 dark:text-amber-400",
            ),
        ),
        (
            "sky",
            (
                "bg-sky-100 dark:bg-sky-900/30",
                "text-sky-600 dark:text-sky-400",
            ),
        ),
        (
            "violet",
            (
                "bg-violet-100 dark:bg-violet-900/30",
                "text-violet-600 dark:text-violet-400",
            ),
        ),
        (
            "zinc",
            (
                "bg-zinc-200 dark:bg-zinc-700",
                "text-zinc-700 dark:text-zinc-300",
            ),
        ),
    ]
    .into_iter()
    .collect();

    let (bg_class, text_class) = color_map
        .get(action.color.as_str())
        .copied()
        .unwrap_or(("bg-zinc-100", "text-zinc-500"));
    let danger = matches!(action.id.as_str(), "shutdown" | "restart" | "logout");
    let ok_class = if danger {
        "bg-rose-600 hover:bg-rose-700"
    } else {
        "bg-cyan-600 hover:bg-cyan-700"
    };

    rsx! {
        if show_confirm() {
            div { class: "fixed inset-0 z-[70] bg-black/60 backdrop-blur-sm flex items-center justify-center",
                div { class: "max-w-sm mx-auto bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 shadow-2xl p-5 mx-4",
                    div { class: "w-12 h-12 rounded-full bg-rose-100 dark:bg-rose-900/30 flex items-center justify-center mb-3",
                        span { class: "material-symbols-rounded text-rose-600 dark:text-rose-400", "warning" }
                    }
                    h3 { class: "font-semibold text-lg", "{action.label}" }
                    p { class: "text-sm text-zinc-600 dark:text-zinc-400 mt-2", "{action.confirm}" }
                    div { class: "flex justify-end gap-2 mt-4",
                        button {
                            class: "px-4 py-2 rounded-lg text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800",
                            onclick: move |_| show_confirm.set(false),
                            "Cancel"
                        }
                        button {
                            class: "px-4 py-2 rounded-lg text-white text-sm font-medium {ok_class}",
                            onclick: move |_| {
                                show_confirm.set(false);
                                let cmd = match action.id.as_str() {
                                    "sleep" => "system_sleep",
                                    "restart" => "system_reboot",
                                    "shutdown" => "system_shutdown",
                                    "lock" => "system_lock",
                                    "logout" => "system_logout",
                                    _ => return,
                                };
                                let _ = bridge::invoke_app_command(cmd, &serde_json::json!({}));
                            },
                            "Confirm"
                        }
                    }
                }
            }
        }
        button {
            class: "flex flex-col items-center gap-2 p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 hover:border-cyan-500 transition-colors",
            onclick: move |_| show_confirm.set(true),
            div { class: "w-12 h-12 rounded-full {bg_class} flex items-center justify-center",
                span { class: "material-symbols-rounded text-2xl {text_class}", "{action.icon}" }
            }
            span { class: "text-sm font-medium", "{action.label}" }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
struct ScheduleRowProps {
    schedule: ScheduleEntry,
    on_toggle: EventHandler<(String, bool)>,
    on_edit: EventHandler<ScheduleEntry>,
    on_delete: EventHandler<String>,
}

#[component]
fn ScheduleRow(props: ScheduleRowProps) -> Element {
    let schedule_name = props.schedule.name.clone();
    let schedule_freq = props.schedule.frequency.clone();
    let schedule_time = props.schedule.time.clone();
    let schedule_cats_len = props.schedule.categories.len();
    let schedule_enabled = props.schedule.enabled;
    let id_for_delete = props.schedule.id.clone();
    let id_for_toggle = props.schedule.id.clone();
    rsx! {
        div { class: "flex items-center gap-3 p-3 rounded-xl border border-zinc-200 dark:border-zinc-800",
            div { class: "w-10 h-10 rounded-lg bg-cyan-100 dark:bg-cyan-900/30 flex items-center justify-center flex-shrink-0",
                span { class: "material-symbols-rounded text-cyan-600 dark:text-cyan-400", "schedule" }
            }
            div { class: "flex-1 min-w-0",
                div { class: "text-sm font-medium truncate", "{schedule_name}" }
                div { class: "text-xs text-zinc-500 dark:text-zinc-400", "{schedule_freq} at {schedule_time} · {schedule_cats_len} categories" }
            }
            div { class: "flex items-center gap-2",
                button {
                    class: "w-8 h-8 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 flex items-center justify-center transition-colors",
                    onclick: move |_| {
                        props.on_edit.call(props.schedule.clone());
                    },
                    span { class: "material-symbols-rounded text-zinc-400 hover:text-cyan-500 text-base", "edit" }
                }
                button {
                    class: "w-8 h-8 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 flex items-center justify-center transition-colors",
                    onclick: move |_| {
                        let id = id_for_delete.clone();
                        props.on_delete.call(id);
                    },
                    span { class: "material-symbols-rounded text-zinc-400 hover:text-rose-500 text-base", "delete" }
                }
                label { class: "relative inline-flex items-center cursor-pointer",
                    input {
                        r#type: "checkbox",
                        checked: schedule_enabled,
                        class: "sr-only peer",
                        onchange: move |e| {
                            let id = id_for_toggle.clone();
                            props.on_toggle.call((id, e.checked()));
                        },
                    }
                    div { class: "w-9 h-5 bg-zinc-200 dark:bg-zinc-700 peer-checked:bg-cyan-500 rounded-full transition-colors after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:w-4 after:h-4 after:bg-white after:rounded-full after:transition-transform peer-checked:after:translate-x-4", "" }
                }
            }
        }
    }
}
