//! MCP Bridge server implementation for Cleanux.
//!
//! Provides WebSocket-based GUI inspection for Dioxus Desktop apps.

use crate::bridge_state::{BridgeState, EvalRequest};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JsonRpcRequest {
  jsonrpc: String,
  method: String,
  params: Option<serde_json::Value>,
  id: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JsonRpcResponse {
  jsonrpc: String,
  result: Option<serde_json::Value>,
  error: Option<JsonRpcError>,
  id: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JsonRpcError {
  code: i32,
  message: String,
}

/// MCP Bridge for Cleanux Desktop applications
pub struct McpBridge {
  port: u16,
}

impl McpBridge {
  pub fn new(port: u16) -> (Self, Arc<BridgeState>) {
    let state = Arc::new(BridgeState::new());
    (Self { port }, state)
  }

  /// Run the bridge server (blocking)
  pub fn run(&self, state: Arc<BridgeState>) {
    let addr = format!("127.0.0.1:{}", self.port);
    let listener = match TcpListener::bind(&addr) {
      Ok(l) => l,
      Err(_) => {
        // Try with port 0 to get an available port
        let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind");
        let local_addr = listener.local_addr().expect("failed to get local addr");
        state.set_bound_port(local_addr.port());
        listener.set_nonblocking(true).ok();

        // Run accept loop in blocking mode
        loop {
          if let Ok((mut stream, _)) = listener.accept() {
            let state = state.clone();
            thread::spawn(move || {
              handle_connection(&mut stream, state);
            });
          }
          thread::sleep(Duration::from_millis(10));
        }
      }
    };

    let local_addr = listener.local_addr().expect("failed to get local addr");
    state.set_bound_port(local_addr.port());

    listener.set_nonblocking(true).ok();

    for stream in listener.incoming() {
      match stream {
        Ok(mut stream) => {
          let state = state.clone();
          thread::spawn(move || {
            handle_connection(&mut stream, state);
          });
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
          thread::sleep(Duration::from_millis(10));
        }
        Err(e) => {
          tracing::error!("Connection error: {}", e);
        }
      }
    }
  }
}

fn handle_connection(stream: &mut std::net::TcpStream, state: Arc<BridgeState>) {
  use std::io::{Read, Write};

  let mut buffer = [0u8; 8192];

  loop {
    match stream.read(&mut buffer) {
      Ok(0) => break, // Connection closed
      Ok(n) => {
        let request_str = String::from_utf8_lossy(&buffer[..n]);

        // Parse JSON-RPC request
        if let Ok(request) = serde_json::from_str::<JsonRpcRequest>(&request_str) {
          let response = process_request(&request, &state);

          if let Ok(resp_str) = serde_json::to_string(&response) {
            let _ = stream.write_all(resp_str.as_bytes());
            let _ = stream.write_all(b"\n");
          }
        }
      }
      Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
        thread::sleep(Duration::from_millis(10));
      }
      Err(_) => break,
    }

    // Send any pending responses
    let responses = state.dequeue_all_responses();
    for resp in responses {
      let json_resp = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: Some(resp.result),
        error: None,
        id: Some(serde_json::Value::String(resp.id)),
      };
      if let Ok(resp_str) = serde_json::to_string(&json_resp) {
        let _ = stream.write_all(resp_str.as_bytes());
        let _ = stream.write_all(b"\n");
      }
    }
  }
}

fn process_request(request: &JsonRpcRequest, state: &Arc<BridgeState>) -> JsonRpcResponse {
  let method = &request.method;
  let id = request.id.clone();

  let result = match method.as_str() {
    "ping" => serde_json::json!({ "pong": true }),
    "uptime" => serde_json::json!({ "seconds": state.uptime_secs() }),
    "port" => serde_json::json!({ "port": state.bound_port() }),
    "evaluate_js" | "dom_snapshot" | "webview_screenshot" => {
      let params = request.params.clone().unwrap_or_default();
      let id = params
        .get("id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| state.next_id());

      state.enqueue_eval_request(EvalRequest {
        id,
        method: method.to_string(),
        params,
      });

      serde_json::json!({ "queued": true })
    }
    "list_commands" => {
      serde_json::json!([
        "ping",
        "uptime",
        "port",
        "evaluate_js",
        "dom_snapshot",
        "webview_screenshot"
      ])
    }
    _ => {
      return JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: None,
        error: Some(JsonRpcError {
          code: -32601,
          message: format!("Method not found: {}", method),
        }),
        id,
      };
    }
  };

  JsonRpcResponse {
    jsonrpc: "2.0".to_string(),
    result: Some(result),
    error: None,
    id,
  }
}

/// Start the MCP bridge server as a background thread.
pub fn start_mcp_bridge(port: u16) -> (McpBridge, Arc<BridgeState>) {
  let (bridge, state) = McpBridge::new(port);
  let state_clone = state.clone();
  let bridge_for_thread = McpBridge { port };

  thread::spawn(move || {
    bridge_for_thread.run(state_clone);
  });

  (bridge, state)
}

/// Legacy bridge consumer loop for Cleanux's custom DOM/eval handling.
pub fn bridge_consumer_loop(state: Arc<BridgeState>) {
  loop {
    let commands = state.dequeue_all();
    for cmd in commands {
      tracing::debug!("Received command: {} with id {}", cmd.method, cmd.id);
      // Commands are handled by the Dioxus app via the bridge invoke mechanism
    }

    thread::sleep(Duration::from_millis(50));
  }
}
