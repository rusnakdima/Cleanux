//! Scheduler service — CRUD operations for scheduled cleanup entries.
//!
//! All operations delegate to global_state schedule mutators so the UI sees
//! changes immediately.

use crate::global_state::{self, ScheduleEntry};
use dioxus_shared::error::AppError;
use dioxus_shared::response::Response;

/// Return all schedules.
pub async fn schedule_get_all() -> Result<Response<Vec<ScheduleEntry>>, AppError> {
    let entries = global_state::get_schedules();
    Ok(Response::success(entries, Some("Schedules retrieved")))
}

/// Add a new schedule entry.
pub async fn schedule_add(
    name: &str,
    frequency: &str,
    time: &str,
    categories: &[String],
) -> Result<Response<ScheduleEntry>, AppError> {
    let entry = ScheduleEntry {
        id: format!("sched-{}", uuid::Uuid::new_v4()),
        name: name.to_string(),
        frequency: frequency.to_string(),
        time: time.to_string(),
        enabled: true,
        categories: categories.to_vec(),
    };
    global_state::schedule_add(entry.clone());
    Ok(Response::success(entry, Some("Schedule added")))
}

/// Update an existing schedule entry.
pub async fn schedule_edit(
    id: &str,
    name: &str,
    frequency: &str,
    time: &str,
    categories: &[String],
) -> Result<Response<ScheduleEntry>, AppError> {
    let updated = ScheduleEntry {
        id: id.to_string(),
        name: name.to_string(),
        frequency: frequency.to_string(),
        time: time.to_string(),
        enabled: true,
        categories: categories.to_vec(),
    };
    global_state::schedule_edit(id, updated.clone());
    Ok(Response::success(updated, Some("Schedule updated")))
}

/// Delete a schedule entry by id.
pub async fn schedule_delete(id: &str) -> Result<Response<bool>, AppError> {
    global_state::schedule_delete(id);
    Ok(Response::success(true, Some("Schedule deleted")))
}
