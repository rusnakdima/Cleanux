//! SDUI strangler runtime: schema loader, root component, and action processor.
//!
//! Cutover from hardcoded RSX pages to pure DynamicPage-driven UI.

use dioxus::prelude::*;
use dioxus_shared::get_theme_css;
use dioxus_shared::mcp::bridge::BridgeStateHandle;
use dioxus_shared::schema::{load_schema_with_status, AppSchema};
use dioxus_shared::themes::{ThemeMode, ThemeVariant};
use dioxus_shared::ui::components::{ActionBus, DynamicPage, ThemeProvider};

use dioxus_shared::load_theme_pref;

/// Root app component — loads schema directly inside Dioxus runtime.
#[component]
pub fn RootApp() -> Element {
    let (schema, schema_found) = load_schema_with_status("cleanux");
    let initial_dark = load_theme_pref("cleanux");

    rsx! {
        style { {get_theme_css()} {include_str!("../../assets/app.css")} }
        ThemeProvider {
            initial_mode: if initial_dark { ThemeMode::Dark } else { ThemeMode::Light },
            initial_variant: ThemeVariant::MaterialDesign3,
            SduiSurface {
                schema: schema,
                app_id: "cleanux".to_string(),
                schema_not_found: !schema_found,
            }
        }
    }
}

/// Root app component with bridge_state prop.
#[component]
pub fn RootAppWithBridge(bridge_state: BridgeStateHandle) -> Element {
    provide_context(bridge_state);
    let (schema, schema_found) = load_schema_with_status("cleanux");
    let initial_dark = load_theme_pref("cleanux");

    rsx! {
        style { {get_theme_css()} {include_str!("../../assets/app.css")} }
        ThemeProvider {
            initial_mode: if initial_dark { ThemeMode::Dark } else { ThemeMode::Light },
            initial_variant: ThemeVariant::MaterialDesign3,
            SduiSurface {
                schema: schema,
                app_id: "cleanux".to_string(),
                schema_not_found: !schema_found,
            }
        }
    }
}

#[component]
pub fn SduiRoot(schema: AppSchema) -> Element {
    rsx! {
        ThemeProvider {
            initial_mode: ThemeMode::System,
            initial_variant: ThemeVariant::MaterialDesign3,
            SduiSurface {
                schema: schema,
                app_id: "cleanux".to_string(),
                schema_not_found: false,
            }
        }
    }
}

/// Everything that needs the ThemeState context lives here.
#[component]
fn SduiSurface(schema: AppSchema, app_id: String, schema_not_found: bool) -> Element {
    let bus = ActionBus::new("/");
    provide_context(bus.clone());

    rsx! {
        div {
            class: "min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors",
            DynamicPage {
                schema: schema.clone(),
                app_id: app_id.clone(),
                initial_route: String::from("/"),
                bus: bus.clone(),
                registry: None,
                schema_not_found: schema_not_found,
            }
        }
    }
}
