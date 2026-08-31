//! Cleanux - Pure Dioxus Desktop Application
//!
//! A system cleanup application migrated to schema-driven UI (SDUI).

use std::sync::Arc;
use std::thread;

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use cleanux::bridge::bridge_consumer_loop;
use cleanux::infrastructure::json_storage::JsonStorage;
use cleanux::presentation::sdui::RootApp;
use dioxus_shared::env::data_dir;
use dioxus_shared::mcp::bridge::McpBridge;
use dioxus_shared::mcp::dynamic_port;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info,cleanux=debug")
        .init();

    tracing::info!("Starting Cleanux Dioxus application");

    let port = dynamic_port();
    let (bridge, bridge_state_raw): (McpBridge, _) = McpBridge::new(port);

    // Spawn bridge thread immediately - bridge is consumed here
    println!("MCP Bridge listening on ws://127.0.0.1:{}", port);
    thread::spawn(move || bridge.run());

    // Spawn bridge consumer loop
    thread::spawn(move || bridge_consumer_loop(bridge_state_raw));

    // Wire JsonStorage for application services
    let storage: Arc<JsonStorage> = Arc::new(JsonStorage::new(data_dir("cleanux")));
    provide_context(storage.clone());

    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("Cleanux")
                    .with_inner_size(dioxus_desktop::LogicalSize::new(1200.0, 800.0)),
            ),
        )
        .launch(RootApp)
}
