//! System monitoring service for Cleanux
//!
//! Background task that periodically collects CPU, memory, and temperature data.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::interval;

/// Snapshot of system metrics at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSnapshot {
    pub cpu_percent: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub memory_percent: f32,
    pub swap_used_mb: u64,
    pub swap_total_mb: u64,
    pub cpu_temp_c: Option<f64>,
    pub gpu_temp_c: Option<f64>,
    pub timestamp_secs: i64,
}

/// Shared monitoring state — written by the background task, read by the UI.
#[derive(Debug)]
pub struct MonitoringState {
    /// Latest snapshot, or None if monitoring has never run.
    pub latest: RwLock<Option<MonitoringSnapshot>>,
    /// Whether the background task should keep running.
    pub running: std::sync::atomic::AtomicBool,
    /// Polling interval in seconds.
    pub interval_secs: u64,
}

impl MonitoringState {
    pub fn new(interval_secs: u64) -> Self {
        Self {
            latest: RwLock::new(None),
            running: std::sync::atomic::AtomicBool::new(false),
            interval_secs,
        }
    }

    /// Returns a copy of the latest snapshot, or None.
    pub fn get_latest(&self) -> Option<MonitoringSnapshot> {
        self.latest.read().clone()
    }

    /// Returns true if the monitoring loop is currently active.
    pub fn is_running(&self) -> bool {
        self.running
            .load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Collect a single snapshot of all monitored metrics.
async fn collect_snapshot() -> MonitoringSnapshot {
    let sys = sysinfo::System::new_all();
    let cpu_percent = sys.global_cpu_usage();

    let total_mem = sys.total_memory() / (1024 * 1024);
    let used_mem = sys.used_memory() / (1024 * 1024);
    let mem_pct = if total_mem > 0 {
        (used_mem as f32 / total_mem as f32) * 100.0
    } else {
        0.0
    };

    let total_swap = sys.total_swap() / (1024 * 1024);
    let used_swap = sys.used_swap() / (1024 * 1024);

    let cpu_temp = crate::infrastructure::sys_utils::get_cpu_temp()
        .await
        .ok();

    let gpu_temp = crate::infrastructure::sys_utils::get_gpu_temp()
        .await
        .ok();

    MonitoringSnapshot {
        cpu_percent,
        memory_used_mb: used_mem,
        memory_total_mb: total_mem,
        memory_percent: mem_pct,
        swap_used_mb: used_swap,
        swap_total_mb: total_swap,
        cpu_temp_c: cpu_temp,
        gpu_temp_c: gpu_temp,
        timestamp_secs: chrono::Utc::now().timestamp(),
    }
}

/// Start the background monitoring loop.
///
/// Call `start()` on a tokio task and pass the `state` Arc in.  The task
/// will update `state.latest` every `interval_secs` seconds until
/// `state.running` is set to false.
pub async fn monitoring_loop(
    state: Arc<MonitoringState>,
    mut stop_rx: mpsc::Receiver<()>,
) {
    let mut ticker = interval(Duration::from_secs(state.interval_secs));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                if !state.running.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
                let snap = collect_snapshot().await;
                *state.latest.write() = Some(snap);
            }
            _ = stop_rx.recv() => {
                break;
            }
        }
    }
}

/// Start monitoring in the background and return a `StopGuard` that, when
/// dropped, signals the loop to stop.
pub fn start_monitoring(
    state: Arc<MonitoringState>,
) -> MonitoringStopGuard {
    state
        .running
        .store(true, std::sync::atomic::Ordering::SeqCst);

    let (tx, rx) = mpsc::channel(1);
    let state_clone = state.clone();

    tokio::spawn(async move {
        monitoring_loop(state_clone, rx).await;
    });

    MonitoringStopGuard { tx }
}

/// RAII guard that stops the monitoring loop on drop.
#[must_use]
pub struct MonitoringStopGuard {
    tx: mpsc::Sender<()>,
}

impl Drop for MonitoringStopGuard {
    fn drop(&mut self) {
        // Signal the loop to stop; ignore error if already stopped.
        let _ = self.tx.try_send(());
    }
}
