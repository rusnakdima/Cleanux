//! Cleanux - Pure Dioxus Desktop Application
//!
//! A system cleanup application migrated from Tauri to Dioxus with KAS handlers.

use std::sync::{Arc, OnceLock};
use std::thread;

use dioxus::prelude::*;
use dioxus_desktop::{launch::launch_virtual_dom_blocking, Config};
use tokio::runtime::Builder;

use cleanux::infrastructure::mcp::DioxusMcpServer;
use cleanux::Route;

/// Global window handle for MCP server
static WINDOW_HANDLE: OnceLock<Arc<tao::window::Window>> = OnceLock::new();

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info,cleanux=debug")
        .init();

    tracing::info!("Starting Cleanux Dioxus application");

    // Create a single-threaded Tokio runtime for the webview
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    // Create config with window
    let desktop_config = Config::new().with_window(
        tao::window::WindowBuilder::new()
            .with_title("Cleanux")
            .with_inner_size(tao::dpi::LogicalSize::new(1200, 800)),
    );

    // Capture window and spawn MCP server on separate thread
    let desktop_config = desktop_config.with_on_window(move |window, _| {
        if WINDOW_HANDLE.get().is_none() {
            let _ = WINDOW_HANDLE.set(window.clone());

            tracing::info!("Window captured! Starting MCP server on separate thread...");

            thread::spawn(move || {
                let rt =
                    tokio::runtime::Runtime::new().expect("Failed to create MCP Tokio runtime");

                rt.block_on(async {
                    let server = DioxusMcpServer::new(window);
                    match server.bind().await {
                        Ok(port) => {
                            let _ = std::fs::write("/tmp/cleanux-mcp.port", port.to_string());
                            tracing::info!("MCP server listening on port {}", port);
                            if let Err(e) = server.serve().await {
                                tracing::error!("MCP server error: {}", e);
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to bind MCP server: {}", e);
                        }
                    }
                });
            });
        }
    });

    // Build virtual dom and launch - this BLOCKS the main thread
    let dom = VirtualDom::new(App);

    // Enter the runtime and run the event loop
    rt.block_on(async move {
        launch_virtual_dom_blocking(dom, desktop_config);
    });
}
