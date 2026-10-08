//! Cleanux - Pure Dioxus Desktop Application
//!
//! A system cleanup application with hardcoded UI.

use std::thread;

use dioxus_desktop::{Config, WindowBuilder};

use cleanux::app::App;
use cleanux::env::dynamic_port;
use cleanux::mcp_bridge::{bridge_consumer_loop, start_mcp_bridge};

fn main() {
  // Initialize tracing
  tracing_subscriber::fmt()
    .with_env_filter("info,cleanux=debug")
    .init();

  tracing::info!("Starting Cleanux Dioxus application");

  let port = dynamic_port();
  let (_bridge, bridge_state) = start_mcp_bridge(port);

  // Spawn bridge consumer loop
  println!("MCP Bridge listening on ws://127.0.0.1:{}", port);
  thread::spawn(move || bridge_consumer_loop(bridge_state));

  // DevTools URL — consumed by Conductor midscene.launch handler
  println!("DevTools listening on ws://127.0.0.1:9222");
  dioxus::LaunchBuilder::desktop()
    .with_cfg(
      Config::new().with_window(
        WindowBuilder::new()
          .with_title("Cleanux")
          .with_inner_size(dioxus_desktop::LogicalSize::new(1200.0, 800.0)),
      ),
    )
    .launch(App)
}
