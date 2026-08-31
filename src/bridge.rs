//! MCP bridge integration for Cleanux.

use dioxus_shared::mcp::bridge::{BridgeState, Response};
use serde_json::Value;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn invoke_app_commands() -> Vec<String> {
    Vec::new()
}

pub fn invoke_app_command(name: &str, _payload: &Value) -> Result<Value, String> {
    Err(format!("command not implemented: {}", name))
}

pub fn invoke_ui_action(action: &str, _params: &Value) -> Result<Value, String> {
    Err(format!("action not implemented: {}", action))
}

pub fn bridge_consumer_loop(state: Arc<BridgeState>) {
    loop {
        if state.is_shutdown() {
            break;
        }
        for (id, result) in state.dequeue_js_results() {
            state.set_response(
                id,
                Response {
                    result: Some(serde_json::json!(result)),
                    error: None,
                },
            );
        }
        for cmd in state.dequeue_all() {
            if matches!(cmd.method.as_str(), "evaluate_js" | "dom_snapshot") {
                state.enqueue_eval_request(dioxus_shared::mcp::bridge::state::EvalRequest {
                    id: cmd.id,
                    method: cmd.method,
                    params: cmd.params,
                });
                continue;
            }
            let response = match cmd.method.as_str() {
                "ping" => Response {
                    result: Some(serde_json::json!({ "pong": true })),
                    error: None,
                },
                "app_info" => Response {
                    result: Some(
                        serde_json::json!({ "name": env!("CARGO_PKG_NAME"), "version": env!("CARGO_PKG_VERSION"), "platform": "dioxus-desktop" }),
                    ),
                    error: None,
                },
                "initialize" => Response {
                    result: Some(
                        serde_json::json!({ "protocolVersion": "2024-11-05", "capabilities": { "tools": true }, "serverInfo": { "name": env!("CARGO_PKG_NAME"), "version": env!("CARGO_PKG_VERSION") } }),
                    ),
                    error: None,
                },
                "health" => Response {
                    result: Some(serde_json::json!({ "healthy": true })),
                    error: None,
                },
                "commands_list" => Response {
                    result: Some(serde_json::json!({ "commands": invoke_app_commands() })),
                    error: None,
                },
                "logs_read" => Response {
                    result: Some(serde_json::json!({ "entries": state.get_logs() })),
                    error: None,
                },
                "commands_invoke" => {
                    let name = cmd
                        .params
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let payload = cmd.params.get("payload").cloned().unwrap_or_default();
                    match invoke_app_command(name, &payload) {
                        Ok(value) => Response {
                            result: Some(value),
                            error: None,
                        },
                        Err(e) => Response {
                            result: None,
                            error: Some(e),
                        },
                    }
                }
                "ui_invoke_action" => {
                    let action = cmd
                        .params
                        .get("action")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    match invoke_ui_action(action, &cmd.params) {
                        Ok(value) => Response {
                            result: Some(value),
                            error: None,
                        },
                        Err(e) => Response {
                            result: None,
                            error: Some(e),
                        },
                    }
                }
                "tools/call" => {
                    let name = cmd
                        .params
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let arguments = cmd.params.get("arguments").cloned().unwrap_or_default();
                    match name {
                        "evaluate_js"
                        | "webview_dom_snapshot"
                        | "dom_snapshot"
                        | "webview_screenshot" => {
                            state.enqueue_eval_request(
                                dioxus_shared::mcp::bridge::state::EvalRequest {
                                    id: cmd.id,
                                    method: name.to_string(),
                                    params: arguments,
                                },
                            );
                            continue;
                        }
                        "app_info" => Response {
                            result: Some(
                                serde_json::json!({ "name": env!("CARGO_PKG_NAME"), "version": env!("CARGO_PKG_VERSION"), "platform": "dioxus-desktop" }),
                            ),
                            error: None,
                        },
                        "health" => Response {
                            result: Some(serde_json::json!({ "healthy": true })),
                            error: None,
                        },
                        "commands_list" => Response {
                            result: Some(serde_json::json!({ "commands": invoke_app_commands() })),
                            error: None,
                        },
                        "logs_read" => Response {
                            result: Some(serde_json::json!({ "entries": state.get_logs() })),
                            error: None,
                        },
                        "commands_invoke" => {
                            let name = arguments.get("name").and_then(|v| v.as_str()).unwrap_or("");
                            let payload = arguments.get("payload").cloned().unwrap_or_default();
                            match invoke_app_command(name, &payload) {
                                Ok(value) => Response {
                                    result: Some(value),
                                    error: None,
                                },
                                Err(e) => Response {
                                    result: None,
                                    error: Some(e),
                                },
                            }
                        }
                        _ => match invoke_app_command(name, &arguments) {
                            Ok(value) => Response {
                                result: Some(value),
                                error: None,
                            },
                            Err(e) => Response {
                                result: None,
                                error: Some(e),
                            },
                        },
                    }
                }
                other => Response {
                    result: None,
                    error: Some(format!("unsupported bridge method: {other}")),
                },
            };
            state.set_response(cmd.id, response);
        }
        thread::sleep(Duration::from_millis(25));
    }
}
