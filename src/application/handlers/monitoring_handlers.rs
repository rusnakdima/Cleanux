//! Monitoring KAS handlers for Cleanux
//!
//! Handles background system monitoring loop control.

use dioxus_shared::error::AppError;
use dioxus_shared::response::Response;

use crate::global_state::{
    get_monitoring_snapshot, monitoring_state, start_monitoring as gs_start_monitoring,
    stop_monitoring as gs_stop_monitoring,
};

/// Start the background system monitoring loop.
/// Polls CPU, memory, swap, and temperature every 5 seconds.
/// Idempotent — returns an error if already running.
pub async fn start_monitoring() -> Result<Response<bool>, AppError> {
    match gs_start_monitoring() {
        Some(_guard) => Ok(Response::success(true, Some("Monitoring started"))),
        None => Err(AppError::ValidationError(
            "Monitoring is already running".into(),
        )),
    }
}

/// Stop the background system monitoring loop.
pub async fn stop_monitoring() -> Result<Response<bool>, AppError> {
    gs_stop_monitoring();
    Ok(Response::success(true, Some("Monitoring stopped")))
}

/// Get the latest monitoring snapshot (CPU, memory, temps).
pub async fn get_monitoring_data() -> Result<Response<Option<crate::infrastructure::monitoring_service::MonitoringSnapshot>>, AppError> {
    let snap = get_monitoring_snapshot();
    Ok(Response::success(snap, Some("Monitoring data retrieved")))
}

/// Check whether the monitoring loop is currently active.
pub async fn is_monitoring_active() -> Result<Response<bool>, AppError> {
    let active = monitoring_state().is_running();
    Ok(Response::success(active, None))
}
