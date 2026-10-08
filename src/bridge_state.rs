//! Bridge state for MCP bridge integration.
//!
//! Thread-safe state shared between WebSocket thread and Dioxus app.

use parking_lot::{Mutex, RwLock};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU16, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// An eval request (evaluate_js, dom_snapshot) to be processed by the Dioxus main thread
#[derive(Debug, Clone)]
pub struct EvalRequest {
  pub id: String,
  pub method: String,
  pub params: serde_json::Value,
}

/// A command received from the MCP client (via WebSocket)
#[derive(Debug, Clone)]
pub struct Command {
  pub id: String,
  pub method: String,
  pub params: serde_json::Value,
}

/// A response to return to the MCP client
#[derive(Debug, Clone)]
pub struct Response {
  pub id: String,
  pub result: serde_json::Value,
  pub error: Option<String>,
}

/// Log entry for bridge logs
#[derive(Debug, Clone, serde::Serialize)]
pub struct LogEntry {
  pub timestamp: String,
  pub level: String,
  pub message: String,
}

/// Thread-safe bridge state shared between WebSocket thread and Dioxus app
#[derive(Debug)]
pub struct BridgeState {
  // Command queue (MCP client → Dioxus app)
  commands: Mutex<VecDeque<Command>>,
  // Response queue (Dioxus app → MCP client)
  responses: Mutex<VecDeque<Response>>,
  // Eval request queue (Dioxus app needs to evaluate JS in webview)
  eval_requests: Mutex<VecDeque<EvalRequest>>,
  // JS results from webview evaluations
  js_results: Mutex<VecDeque<(String, serde_json::Value)>>,
  // Pending responses indexed by ID
  pending_responses: Mutex<std::collections::HashMap<String, Response>>,
  // Port the bridge is listening on
  bound_port: AtomicU16,
  // Request counter for unique IDs
  request_counter: AtomicU64,
  // When the bridge started
  start_time: Instant,
  // Shutdown flag
  shutdown: Mutex<bool>,
  // In-memory log buffer
  logs: RwLock<VecDeque<LogEntry>>,
  // Current navigation route
  navigation: RwLock<String>,
  // Pending notifications
  notifications: RwLock<Vec<String>>,
  // Current theme
  theme: RwLock<String>,
}

impl BridgeState {
  pub fn new() -> Self {
    Self {
      commands: Mutex::new(VecDeque::new()),
      responses: Mutex::new(VecDeque::new()),
      eval_requests: Mutex::new(VecDeque::new()),
      js_results: Mutex::new(VecDeque::new()),
      pending_responses: Mutex::new(std::collections::HashMap::new()),
      bound_port: AtomicU16::new(0),
      request_counter: AtomicU64::new(0),
      start_time: Instant::now(),
      shutdown: Mutex::new(false),
      logs: RwLock::new(VecDeque::with_capacity(1000)),
      navigation: RwLock::new(String::new()),
      notifications: RwLock::new(Vec::new()),
      theme: RwLock::new(String::from("system")),
    }
  }

  /// Enqueue a command from the MCP client
  pub fn enqueue_command(&self, cmd: Command) {
    self.commands.lock().push_back(cmd);
  }

  /// Dequeue all pending commands
  pub fn dequeue_all(&self) -> Vec<Command> {
    let mut commands = self.commands.lock();
    commands.drain(..).collect()
  }

  /// Enqueue a response for the MCP client
  pub fn enqueue_response(&self, resp: impl Into<Response>) {
    self.responses.lock().push_back(resp.into());
  }

  /// Dequeue all pending responses
  pub fn dequeue_all_responses(&self) -> Vec<Response> {
    let mut responses = self.responses.lock();
    responses.drain(..).collect()
  }

  /// Enqueue an eval request (Dioxus app wants to evaluate JS)
  pub fn enqueue_eval_request(&self, req: EvalRequest) {
    self.eval_requests.lock().push_back(req);
  }

  /// Dequeue all pending eval requests
  pub fn dequeue_eval_requests(&self) -> Vec<EvalRequest> {
    let mut requests = self.eval_requests.lock();
    requests.drain(..).collect()
  }

  /// Dequeue all JS results
  pub fn dequeue_js_results(&self) -> Vec<(String, serde_json::Value)> {
    let mut results = self.js_results.lock();
    results.drain(..).collect()
  }

  /// Set a response for a given request ID
  pub fn set_response(&self, id: String, response: Response) {
    self.pending_responses.lock().insert(id, response);
  }

  /// Set the bound port
  pub fn set_bound_port(&self, port: u16) {
    self.bound_port.store(port, Ordering::SeqCst);
  }

  /// Get the bound port
  pub fn bound_port(&self) -> u16 {
    self.bound_port.load(Ordering::SeqCst)
  }

  /// Generate a unique request ID
  pub fn next_id(&self) -> String {
    let count = self.request_counter.fetch_add(1, Ordering::SeqCst);
    format!("req-{}", count)
  }

  /// Get uptime in seconds
  pub fn uptime_secs(&self) -> u64 {
    self.start_time.elapsed().as_secs()
  }

  /// Check if shutdown is requested
  pub fn is_shutdown(&self) -> bool {
    *self.shutdown.lock()
  }

  /// Request shutdown
  pub fn shutdown(&self) {
    *self.shutdown.lock() = true;
  }

  /// Get recent logs
  pub fn get_logs(&self) -> Vec<LogEntry> {
    let logs = self.logs.read();
    logs.iter().cloned().collect()
  }

  /// Add a log entry
  pub fn add_log(&self, level: &str, message: &str) {
    let entry = LogEntry {
      timestamp: chrono::Utc::now().to_rfc3339(),
      level: level.to_string(),
      message: message.to_string(),
    };
    let mut logs = self.logs.write();
    if logs.len() >= 1000 {
      logs.pop_front();
    }
    logs.push_back(entry);
  }

  /// Set the current navigation route
  pub fn set_navigation(&self, route: String) {
    *self.navigation.write() = route;
  }

  /// Push a notification message
  pub fn push_notification(&self, message: String) {
    self.notifications.write().push(message);
  }

  /// Set the current theme
  pub fn set_theme(&self, theme: String) {
    *self.theme.write() = theme;
  }
}

impl Default for BridgeState {
  fn default() -> Self {
    Self::new()
  }
}

/// Newtype wrapper for Arc<BridgeState> that implements PartialEq
/// Uses pointer equality since BridgeState is a singleton context
#[derive(Debug, Clone)]
pub struct BridgeStateHandle(pub Arc<BridgeState>);

impl PartialEq for BridgeStateHandle {
  fn eq(&self, other: &Self) -> bool {
    Arc::ptr_eq(&self.0, &other.0)
  }
}

impl BridgeStateHandle {
  pub fn new(state: Arc<BridgeState>) -> Self {
    Self(state)
  }

  pub fn inner(&self) -> Arc<BridgeState> {
    self.0.clone()
  }
}

impl Default for BridgeStateHandle {
  fn default() -> Self {
    Self::new(Arc::new(BridgeState::new()))
  }
}
