//! Reports page — mirrors the template `data-page="reports"` section.
//!
//! Manages cleaning reports, history, export, and snapshot comparison.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CleaningReport {
  pub id: String,
  pub date: String,
  pub size_cleaned: u64,
  pub duration_secs: u64,
  pub categories: Vec<String>,
  pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HealthSnapshot {
  pub id: String,
  pub date: String,
  pub health_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
  Date,
  Size,
  Duration,
}

impl SortColumn {
  fn label(&self) -> &'static str {
    match self {
      SortColumn::Date => "Date",
      SortColumn::Size => "Size",
      SortColumn::Duration => "Duration",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SortDir {
  Asc,
  Desc,
}

impl SortDir {
  fn next(&self) -> Self {
    match self {
      SortDir::Asc => SortDir::Desc,
      SortDir::Desc => SortDir::Asc,
    }
  }
}

const PAGE_SIZE: usize = 10;

fn format_bytes(bytes: u64) -> String {
  const KB: u64 = 1024;
  const MB: u64 = KB * 1024;
  const GB: u64 = MB * 1024;
  if bytes >= GB {
    format!("{:.1} GB", bytes as f64 / GB as f64)
  } else if bytes >= MB {
    format!("{:.1} MB", bytes as f64 / MB as f64)
  } else if bytes >= KB {
    format!("{:.1} KB", bytes as f64 / KB as f64)
  } else {
    format!("{} B", bytes)
  }
}

fn format_duration(secs: u64) -> String {
  if secs >= 3600 {
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    format!("{}h {}m", hours, mins)
  } else if secs >= 60 {
    let mins = secs / 60;
    let secs = secs % 60;
    format!("{}m {}s", mins, secs)
  } else {
    format!("{}s", secs)
  }
}

fn format_date(date: &str) -> String {
  if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date) {
    dt.format("%b %d, %Y · %H:%M").to_string()
  } else {
    date.to_string()
  }
}

fn format_pct(value: f64) -> String {
  format!("{:.1}%", value)
}

#[derive(Clone, Props, PartialEq)]
struct ReportRowProps {
  report: CleaningReport,
  on_view: Callback<CleaningReport>,
}

#[component]
fn ReportRow(props: ReportRowProps) -> Element {
  let report = props.report.clone();
  let categories_str = report.categories.join(", ");
  rsx! {
      tr {
          class: "border-b border-zinc-100 dark:border-zinc-800 hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors",
          td { class: "px-4 py-3",
              div { class: "flex items-center gap-3",
                  div { class: "w-8 h-8 rounded-lg bg-green-100 dark:bg-green-900/30 flex items-center justify-center text-sm",
                      "📊"
                  }
                  div {
                      p { class: "font-medium text-sm", "{format_date(&report.date)}" }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400", "ID: {report.id}" }
                  }
              }
          }
          td { class: "px-4 py-3 text-right",
              p { class: "font-medium text-sm", "{format_bytes(report.size_cleaned)}" }
          }
          td { class: "px-4 py-3 text-right",
              p { class: "font-medium text-sm", "{format_duration(report.duration_secs)}" }
          }
          td { class: "px-4 py-3",
              p { class: "text-xs text-zinc-600 dark:text-zinc-400 truncate max-w-32", title: "{categories_str}",
                  "{categories_str}"
              }
          }
          td { class: "px-4 py-3",
              span {
                  class: format!(
                      "px-2 py-1 rounded-full text-xs font-medium {}",
                      if report.status == "completed" {
                          "bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300"
                      } else if report.status == "partial" {
                          "bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-300"
                      } else {
                          "bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300"
                      }
                  ),
                  "{report.status}"
              }
          }
          td { class: "px-4 py-3",
              div { class: "flex justify-end",
                  button {
                      class: "px-3 py-1 rounded-lg border border-zinc-200 dark:border-zinc-700 hover:bg-blue-50 dark:hover:bg-blue-900/20 text-xs font-medium transition-colors text-blue-600 dark:text-blue-400",
                      onclick: move |_| props.on_view.call(report.clone()),
                      "View Details"
                  }
              }
          }
      }
  }
}

#[derive(Clone, Props, PartialEq)]
struct ReportDetailModalProps {
  report: CleaningReport,
  on_close: Callback<()>,
}

#[component]
fn ReportDetailModal(props: ReportDetailModalProps) -> Element {
  let report = props.report.clone();
  rsx! {
      div {
          class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm",
          onclick: move |_| {
              props.on_close.call(());
          },
          div {
              class: "bg-white dark:bg-zinc-900 rounded-2xl shadow-2xl max-w-lg w-full max-h-[80vh] overflow-y-auto",
              onclick: move |e| {
                  e.stop_propagation();
              },
              div { class: "p-6 border-b border-zinc-100 dark:border-zinc-800",
                  div { class: "flex items-center justify-between",
                      h2 { class: "text-xl font-bold", "Report Details" }
                      button {
                          class: "w-8 h-8 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 flex items-center justify-center text-zinc-500",
                          onclick: move |_| props.on_close.call(()),
                          "✕"
                      }
                  }
              }
              div { class: "p-6 space-y-4",
                  div { class: "grid grid-cols-2 gap-4",
                      div { class: "bg-zinc-50 dark:bg-zinc-800/50 rounded-xl p-4",
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400 mb-1", "Date" }
                          p { class: "font-semibold", "{format_date(&report.date)}" }
                      }
                      div { class: "bg-zinc-50 dark:bg-zinc-800/50 rounded-xl p-4",
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400 mb-1", "Report ID" }
                          p { class: "font-semibold text-sm", "{report.id}" }
                      }
                      div { class: "bg-zinc-50 dark:bg-zinc-800/50 rounded-xl p-4",
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400 mb-1", "Size Cleaned" }
                          p { class: "font-semibold text-green-600 dark:text-green-400", "{format_bytes(report.size_cleaned)}" }
                      }
                      div { class: "bg-zinc-50 dark:bg-zinc-800/50 rounded-xl p-4",
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400 mb-1", "Duration" }
                          p { class: "font-semibold", "{format_duration(report.duration_secs)}" }
                      }
                  }
                  div { class: "bg-zinc-50 dark:bg-zinc-800/50 rounded-xl p-4",
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400 mb-2", "Categories" }
                      div { class: "flex flex-wrap gap-2",
                          for cat in report.categories.iter() {
                              span { class: "px-2 py-1 bg-blue-100 dark:bg-blue-900/30 rounded-lg text-xs text-blue-700 dark:text-blue-300", "{cat}" }
                          }
                      }
                  }
                  div { class: "bg-zinc-50 dark:bg-zinc-800/50 rounded-xl p-4",
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400 mb-1", "Status" }
                      span {
                          class: format!(
                              "px-3 py-1 rounded-full text-sm font-medium {}",
                              if report.status == "completed" {
                                  "bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300"
                              } else if report.status == "partial" {
                                  "bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-300"
                              } else {
                                  "bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300"
                              }
                          ),
                          "{report.status}"
                      }
                  }
              }
          }
      }
  }
}

#[derive(Clone, Props, PartialEq)]
struct SnapshotCompareModalProps {
  snapshots: Vec<HealthSnapshot>,
  on_close: Callback<()>,
  on_compare: Callback<(String, String)>,
}

#[component]
fn SnapshotCompareModal(props: SnapshotCompareModalProps) -> Element {
  let mut snapshot1_id = use_signal(|| String::new());
  let mut snapshot2_id = use_signal(|| String::new());

  rsx! {
      div {
          class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm",
          onclick: move |_| {
              props.on_close.call(());
          },
          div {
              class: "bg-white dark:bg-zinc-900 rounded-2xl shadow-2xl max-w-md w-full",
              onclick: move |e| {
                  e.stop_propagation();
              },
              div { class: "p-6 border-b border-zinc-100 dark:border-zinc-800",
                  div { class: "flex items-center justify-between",
                      h2 { class: "text-xl font-bold", "Compare Snapshots" }
                      button {
                          class: "w-8 h-8 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 flex items-center justify-center text-zinc-500",
                          onclick: move |_| props.on_close.call(()),
                          "✕"
                      }
                  }
              }
              div { class: "p-6 space-y-4",
                  p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                      "Select two health snapshots to compare their differences."
                  }
                  div { class: "space-y-3",
                      label { class: "block text-sm font-medium", "Snapshot 1" }
                      select {
                          class: "w-full px-4 py-2.5 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-900 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500",
                          value: "{snapshot1_id}",
                          onchange: move |evt| snapshot1_id.set(evt.value().clone()),
                          option { value: "", "Select snapshot..." }
                          for snap in props.snapshots.iter() {
                              option { value: "{snap.id}", "{snap.date} (Score: {snap.health_score:.1})" }
                          }
                      }
                      label { class: "block text-sm font-medium mt-4", "Snapshot 2" }
                      select {
                          class: "w-full px-4 py-2.5 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-900 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500",
                          value: "{snapshot2_id}",
                          onchange: move |evt| snapshot2_id.set(evt.value().clone()),
                          option { value: "", "Select snapshot..." }
                          for snap in props.snapshots.iter() {
                              option { value: "{snap.id}", "{snap.date} (Score: {snap.health_score:.1})" }
                          }
                      }
                  }
                  div { class: "flex gap-3 mt-6",
                      button {
                          class: "flex-1 px-4 py-2.5 rounded-xl border border-zinc-200 dark:border-zinc-700 hover:bg-zinc-50 dark:hover:bg-zinc-800 text-sm font-medium transition-colors",
                          onclick: move |_| props.on_close.call(()),
                          "Cancel"
                      }
                      button {
                          class: "flex-1 px-4 py-2.5 rounded-xl bg-blue-500 hover:bg-blue-600 text-white font-semibold text-sm transition-colors shadow-sm disabled:opacity-50 disabled:cursor-not-allowed",
                          disabled: snapshot1_id.read().is_empty() || snapshot2_id.read().is_empty(),
                          onclick: move |_| {
                              let id1 = snapshot1_id.read().clone();
                              let id2 = snapshot2_id.read().clone();
                              if !id1.is_empty() && !id2.is_empty() {
                                  props.on_compare.call((id1, id2));
                              }
                          },
                          "Compare"
                      }
                  }
              }
          }
      }
  }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct CompareResult {
  space_reclaimed: u64,
  items_cleaned: i64,
  health_improvement: f64,
}

#[component]
pub fn ReportsPage(state: AppState) -> Element {
  let mut reports = use_signal(|| Vec::<CleaningReport>::new());
  let mut snapshots = use_signal(|| Vec::<HealthSnapshot>::new());
  let mut sort_col = use_signal(|| SortColumn::Date);
  let mut sort_dir = use_signal(|| SortDir::Desc);
  let mut current_page = use_signal(|| 0usize);
  let mut selected_report_id = use_signal(|| Option::<String>::None);
  let mut is_loading = use_signal(|| false);
  let mut status_msg = use_signal(|| String::new());
  let mut status_type = use_signal(|| "info".to_string());
  let mut show_compare_modal = use_signal(|| false);
  let mut compare_result = use_signal(|| Option::<CompareResult>::None);

  // Load reports and snapshots on mount
  use_effect(move || {
    // Load reports
    let result = bridge::invoke_app_command("crud_find_cleaning_reports", &serde_json::json!({}));
    if let Ok(val) = result {
      if let Some(data) = val.get("data") {
        if let Ok(list) = serde_json::from_value::<Vec<CleaningReport>>(data.clone()) {
          reports.set(list);
        }
      }
    }
    // Load snapshots for comparison
    let result = bridge::invoke_app_command("crud_find_health_snapshots", &serde_json::json!({}));
    if let Ok(val) = result {
      if let Some(data) = val.get("data") {
        if let Ok(list) = serde_json::from_value::<Vec<HealthSnapshot>>(data.clone()) {
          snapshots.set(list);
        }
      }
    }
  });

  // Sort reports
  let sorted_reports = {
    let all = reports.read().clone();
    let col = *sort_col.read();
    let dir = *sort_dir.read();
    let mut sorted = match col {
      SortColumn::Date => {
        let mut s = all;
        s.sort_by(|a, b| {
          let cmp = a.date.cmp(&b.date);
          if dir == SortDir::Asc {
            cmp
          } else {
            cmp.reverse()
          }
        });
        s
      }
      SortColumn::Size => {
        let mut s = all;
        s.sort_by(|a, b| {
          let cmp = a.size_cleaned.cmp(&b.size_cleaned);
          if dir == SortDir::Asc {
            cmp
          } else {
            cmp.reverse()
          }
        });
        s
      }
      SortColumn::Duration => {
        let mut s = all;
        s.sort_by(|a, b| {
          let cmp = a.duration_secs.cmp(&b.duration_secs);
          if dir == SortDir::Asc {
            cmp
          } else {
            cmp.reverse()
          }
        });
        s
      }
    };
    sorted
  };

  let total_pages = (sorted_reports.len() + PAGE_SIZE - 1) / PAGE_SIZE;
  let page_num = *current_page.read();
  let paginated_reports: Vec<_> = sorted_reports
    .iter()
    .skip(page_num * PAGE_SIZE)
    .take(PAGE_SIZE)
    .cloned()
    .collect();

  let total_cleaned: u64 = sorted_reports.iter().map(|r| r.size_cleaned).sum();

  // Pre-compute sort indicators
  let sort_date_asc = *sort_col.read() == SortColumn::Date && *sort_dir.read() == SortDir::Asc;
  let sort_date_desc = *sort_col.read() == SortColumn::Date && *sort_dir.read() == SortDir::Desc;
  let sort_size_asc = *sort_col.read() == SortColumn::Size && *sort_dir.read() == SortDir::Asc;
  let sort_size_desc = *sort_col.read() == SortColumn::Size && *sort_dir.read() == SortDir::Desc;
  let sort_dur_asc = *sort_col.read() == SortColumn::Duration && *sort_dir.read() == SortDir::Asc;
  let sort_dur_desc = *sort_col.read() == SortColumn::Duration && *sort_dir.read() == SortDir::Desc;

  let current_page_val = *current_page.read();
  let has_prev = current_page_val > 0;
  let has_next = current_page_val < total_pages.saturating_sub(1);
  let is_loading_val = *is_loading.read();
  let status_type_val = status_type.read().clone();
  let status_msg_val = status_msg.read().clone();
  let show_modal = *show_compare_modal.read();
  let snapshots_val = snapshots.read().clone();
  let report_count = sorted_reports.len();
  let compare_result_val = compare_result.read().clone();
  let selected_report_val = {
    if let Some(id) = selected_report_id() {
      reports.read().iter().find(|r| r.id == id).cloned()
    } else {
      None
    }
  };

  rsx! {
      section { "data-page": "reports",
          class: "space-y-6",

          // Header
          div { class: "flex items-center justify-between",
              div {
                  h1 { class: "text-2xl font-bold", "Cleaning Reports" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400 mt-1",
                      "View history and export cleaning reports"
                  }
              }
              div { class: "flex items-center gap-3",
                  div { class: "text-right",
                      p { class: "text-sm font-medium", "{report_count} reports" }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400",
                          "{format_bytes(total_cleaned)} total cleaned"
                      }
                  }
                  button {
                      class: "px-4 py-2 rounded-xl bg-blue-500 hover:bg-blue-600 text-white font-semibold text-sm transition-colors shadow-sm disabled:opacity-50",
                      disabled: is_loading_val,
                      onclick: move |_| {
                          is_loading.set(true);
                          let result = bridge::invoke_app_command("crud_generate_cleaning_report", &serde_json::json!({}));
                          is_loading.set(false);
                          if result.is_ok() {
                              status_msg.set("Report generated successfully".to_string());
                              status_type.set("success".to_string());
                              let result = bridge::invoke_app_command("crud_find_cleaning_reports", &serde_json::json!({}));
                              if let Ok(val) = result {
                                  if let Some(data) = val.get("data") {
                                      if let Ok(list) = serde_json::from_value::<Vec<CleaningReport>>(data.clone()) {
                                          reports.set(list);
                                      }
                                  }
                              }
                          } else {
                              status_msg.set("Failed to generate report".to_string());
                              status_type.set("error".to_string());
                          }
                      },
                      if is_loading_val { "Generating..." } else { "Generate Report" }
                  }
              }
          }

          // Status message
          if !status_msg_val.is_empty() {
              div {
                  class: format!(
                      "rounded-xl p-4 text-sm {}",
                      if status_type_val == "success" {
                          "bg-green-50 dark:bg-green-950/30 border border-green-200 dark:border-green-800 text-green-700 dark:text-green-300"
                      } else {
                          "bg-red-50 dark:bg-red-950/30 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-300"
                      }
                  ),
                  "{status_msg_val}"
              }
          }

          // Action bar
          div { class: "flex flex-wrap items-center gap-3",
              button {
                  class: "px-4 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 hover:bg-purple-50 dark:hover:bg-purple-900/20 text-sm font-medium transition-colors text-purple-600 dark:text-purple-400",
                  onclick: move |_| show_compare_modal.set(true),
                  "Compare Snapshots"
              }
              div { class: "flex-1" }
              div { class: "flex gap-2",
                  button {
                      class: "px-3 py-2 rounded-lg border border-zinc-200 dark:border-zinc-700 hover:bg-zinc-50 dark:hover:bg-zinc-800 text-xs font-medium transition-colors",
                      onclick: {
                          let reports_ref = sorted_reports.clone();
                          move |_| {
                              let mut csv = String::from("ID,Date,Size Cleaned (bytes),Duration (secs),Categories,Status\n");
                              for r in &reports_ref {
                                  csv.push_str(&format!(
                                      "{},{},{},{},\"{}\",{}\n",
                                      r.id, r.date, r.size_cleaned, r.duration_secs,
                                      r.categories.join("; "), r.status
                                  ));
                              }
                              download_file("cleaning_reports.csv", &csv, "text/csv");
                              status_msg.set("Exported to CSV".to_string());
                              status_type.set("success".to_string());
                          }
                      },
                      "Export CSV"
                  }
                  button {
                      class: "px-3 py-2 rounded-lg border border-zinc-200 dark:border-zinc-700 hover:bg-zinc-50 dark:hover:bg-zinc-800 text-xs font-medium transition-colors",
                      onclick: {
                          let reports_ref = sorted_reports.clone();
                          move |_| {
                              let json = serde_json::to_string_pretty(&*reports_ref).unwrap_or_default();
                              download_file("cleaning_reports.json", &json, "application/json");
                              status_msg.set("Exported to JSON".to_string());
                              status_type.set("success".to_string());
                          }
                      },
                      "Export JSON"
                  }
              }
          }

          // Compare result
          if let Some(result) = compare_result_val.as_ref() {
              div { class: "bg-purple-50 dark:bg-purple-950/30 border border-purple-200 dark:border-purple-800 rounded-xl p-4",
                  div { class: "flex items-center justify-between mb-3",
                      h3 { class: "font-semibold text-purple-700 dark:text-purple-300", "Snapshot Comparison" }
                      button {
                          class: "text-purple-500 hover:text-purple-700 text-sm",
                          onclick: move |_| compare_result.set(None),
                          "Dismiss"
                      }
                  }
                  div { class: "grid grid-cols-3 gap-4",
                      div {
                          p { class: "text-xs text-purple-600 dark:text-purple-400", "Space Reclaimed" }
                          p { class: "font-bold text-lg", "{format_bytes(result.space_reclaimed)}" }
                      }
                      div {
                          p { class: "text-xs text-purple-600 dark:text-purple-400", "Items Cleaned" }
                          p { class: "font-bold text-lg", "{result.items_cleaned}" }
                      }
                      div {
                          p { class: "text-xs text-purple-600 dark:text-purple-400", "Health Improvement" }
                          p { class: "font-bold text-lg", "{format_pct(result.health_improvement)}" }
                      }
                  }
              }
          }

          // Reports table
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 overflow-hidden",
              if paginated_reports.is_empty() {
                  div { class: "flex flex-col items-center justify-center py-16 text-center",
                      div { class: "text-4xl mb-3", "📊" }
                      p { class: "font-medium", "No reports yet" }
                      p { class: "text-sm text-zinc-500 dark:text-zinc-400 mt-1",
                          "Generate a report to see your cleaning history"
                      }
                  }
              } else {
                  table { class: "w-full",
                      thead {
                          tr { class: "border-b border-zinc-100 dark:border-zinc-800",
                              th { class: "px-4 py-3 text-left",
                                  button {
                                      class: "flex items-center gap-1 text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide hover:text-zinc-700 dark:hover:text-zinc-200",
                                      onclick: move |_| {
                                          if *sort_col.read() == SortColumn::Date {
                                              let current_dir = *sort_dir.read();
                                              sort_dir.set(current_dir.next());
                                          } else {
                                              sort_col.set(SortColumn::Date);
                                              sort_dir.set(SortDir::Desc);
                                          }
                                          current_page.set(0);
                                      },
                                      "Date"
                                      if sort_date_asc { span { class: "text-blue-500", "↑" } }
                                      if sort_date_desc { span { class: "text-blue-500", "↓" } }
                                  }
                              }
                              th { class: "px-4 py-3 text-right",
                                  button {
                                      class: "flex items-center gap-1 text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide hover:text-zinc-700 dark:hover:text-zinc-200 ml-auto",
                                      onclick: move |_| {
                                          if *sort_col.read() == SortColumn::Size {
                                              let current_dir = *sort_dir.read();
                                              sort_dir.set(current_dir.next());
                                          } else {
                                              sort_col.set(SortColumn::Size);
                                              sort_dir.set(SortDir::Desc);
                                          }
                                          current_page.set(0);
                                      },
                                      "Size"
                                      if sort_size_asc { span { class: "text-blue-500", "↑" } }
                                      if sort_size_desc { span { class: "text-blue-500", "↓" } }
                                  }
                              }
                              th { class: "px-4 py-3 text-right",
                                  button {
                                      class: "flex items-center gap-1 text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide hover:text-zinc-700 dark:hover:text-zinc-200 ml-auto",
                                      onclick: move |_| {
                                          if *sort_col.read() == SortColumn::Duration {
                                              let current_dir = *sort_dir.read();
                                              sort_dir.set(current_dir.next());
                                          } else {
                                              sort_col.set(SortColumn::Duration);
                                              sort_dir.set(SortDir::Desc);
                                          }
                                          current_page.set(0);
                                      },
                                      "Duration"
                                      if sort_dur_asc { span { class: "text-blue-500", "↑" } }
                                      if sort_dur_desc { span { class: "text-blue-500", "↓" } }
                                  }
                              }
                              th { class: "px-4 py-3 text-left text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Categories" }
                              th { class: "px-4 py-3 text-left text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Status" }
                              th { class: "px-4 py-3 text-right text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Actions" }
                          }
                      }
                      tbody {
                          for report in paginated_reports.iter() {
                              ReportRow {
                                  report: report.clone(),
                                  on_view: move |r: CleaningReport| {
                                      selected_report_id.set(Some(r.id));
                                  },
                              }
                          }
                      }
                  }
              }
          }

          // Pagination
          if total_pages > 1 {
              div { class: "flex items-center justify-center gap-2",
                  button {
                      class: "px-3 py-1.5 rounded-lg border border-zinc-200 dark:border-zinc-700 hover:bg-zinc-50 dark:hover:bg-zinc-800 text-sm disabled:opacity-50",
                      disabled: !has_prev,
                      onclick: move |_| current_page.set(current_page_val.saturating_sub(1)),
                      "Previous"
                  }
                  div { class: "flex items-center gap-1",
                      for i in 0..total_pages {
                          button {
                              class: format!(
                                  "w-8 h-8 rounded-lg text-sm font-medium transition-colors {}",
                                  if current_page_val == i {
                                      "bg-blue-500 text-white"
                                  } else {
                                      "hover:bg-zinc-100 dark:hover:bg-zinc-800"
                                  }
                              ),
                              onclick: move |_| current_page.set(i),
                              "{i + 1}"
                          }
                      }
                  }
                  button {
                      class: "px-3 py-1.5 rounded-lg border border-zinc-200 dark:border-zinc-700 hover:bg-zinc-50 dark:hover:bg-zinc-800 text-sm disabled:opacity-50",
                      disabled: !has_next,
                      onclick: move |_| current_page.set(current_page_val + 1),
                      "Next"
                  }
              }
          }

          // Detail modal
          if let Some(report) = selected_report_val {
              ReportDetailModal {
                  report: report.clone(),
                  on_close: move |_| selected_report_id.set(None),
              }
          }

          // Compare snapshots modal
          if show_modal {
              SnapshotCompareModal {
                  snapshots: snapshots_val.clone(),
                  on_close: move |_| show_compare_modal.set(false),
                  on_compare: move |(id1, id2)| {
                      is_loading.set(true);
                      let result = bridge::invoke_app_command(
                          "crud_compare_snapshots",
                          &serde_json::json!({ "id1": id1, "id2": id2 }),
                      );
                      is_loading.set(false);
                      show_compare_modal.set(false);
                      if let Ok(val) = result {
                          if let Some(data) = val.get("data") {
                              if let Ok(res) = serde_json::from_value::<CompareResult>(data.clone()) {
                                  compare_result.set(Some(res));
                              }
                          }
                      } else {
                          status_msg.set("Failed to compare snapshots".to_string());
                          status_type.set("error".to_string());
                      }
                  },
              }
          }
      }
  }
}

/// Helper to trigger file download - placeholder until web_sys is available
#[allow(unused)]
fn download_file(_filename: &str, _content: &str, _mime_type: &str) {
  // TODO: Implement file download via backend command
  // For now, export is handled by the backend via automation_export_report command
  tracing::info!("Download requested - implementation pending");
}
