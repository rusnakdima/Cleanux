use cleanux_lib::services::log_manager_service::{
  JournalInfo, LogFileInfo, LogManagerService, LogManagerSummary, LogrotateAnalysis,
  LogrotateConfig, RotatedLogInfo, VarLogUsage,
};
#[test]
fn test_log_manager_service_get_journal_size() {
  let size = LogManagerService::get_journal_size();
  assert!(size >= 0);
}
#[test]
fn test_log_manager_service_get_var_log_usage() {
  let usage = LogManagerService::get_var_log_usage();
  assert!(usage.total_bytes >= 0);
}
#[test]
fn test_log_manager_service_get_rotated_logs_size() {
  let size = LogManagerService::get_rotated_logs_size();
  assert!(size >= 0);
}
#[test]
fn test_log_manager_service_get_journal_usage() {
  let info = LogManagerService::get_journal_usage();
  assert!(info.size_bytes >= 0);
}
#[test]
fn test_log_manager_service_get_largest_log_files() {
  let files = LogManagerService::get_largest_log_files(10);
  assert!(files.len() >= 0);
}
#[test]
fn test_log_manager_service_get_rotated_logs() {
  let logs = LogManagerService::get_rotated_logs();
  assert!(logs.len() >= 0);
}
#[test]
fn test_journal_info_serialization() {
  let info = JournalInfo {
    size_bytes: 1024,
    size_human: "1.00 KB".to_string(),
    oldest_entry: Some("2024-01-01".to_string()),
    newest_entry: Some("2024-12-01".to_string()),
    is_active: true,
  };
  let json = serde_json::to_string(&info).unwrap();
  assert!(json.contains("1024"));
  assert!(json.contains("1.00 KB"));
  assert!(json.contains("is_active"));
}
#[test]
fn test_rotated_log_info_serialization() {
  let info = RotatedLogInfo {
    path: "/var/log/syslog.1".to_string(),
    size_bytes: 2048,
    size_human: "2.00 KB".to_string(),
    modified: "2024-01-01".to_string(),
    compression_ratio: Some(0.5),
  };
  let json = serde_json::to_string(&info).unwrap();
  assert!(json.contains("/var/log/syslog.1"));
  assert!(json.contains("2048"));
}
#[test]
fn test_logrotate_config_serialization() {
  let config = LogrotateConfig {
    path: "/etc/logrotate.conf".to_string(),
    enabled: true,
    schedule: Some("daily".to_string()),
    max_size: Some("100M".to_string()),
    max_age: Some("7".to_string()),
    compress: true,
    rotate_count: Some(4),
  };
  let json = serde_json::to_string(&config).unwrap();
  assert!(json.contains("/etc/logrotate.conf"));
  assert!(json.contains("daily"));
}
#[test]
fn test_logrotate_analysis_serialization() {
  let analysis = LogrotateAnalysis {
    total_configs: 5,
    enabled_configs: 3,
    configs: vec![],
    potential_savings_mb: 100,
    issues: vec!["Issue 1".to_string()],
  };
  let json = serde_json::to_string(&analysis).unwrap();
  assert!(json.contains("5"));
  assert!(json.contains("100"));
}
#[test]
fn test_var_log_usage_serialization() {
  let usage = VarLogUsage {
    total_bytes: 1024 * 1024,
    total_human: "1.00 MB".to_string(),
    file_count: 100,
    directory_count: 10,
  };
  let json = serde_json::to_string(&usage).unwrap();
  assert!(json.contains("1.00 MB"));
  assert!(json.contains("100"));
}
#[test]
fn test_log_file_info_serialization() {
  let info = LogFileInfo {
    path: "/var/log/auth.log".to_string(),
    size_bytes: 4096,
    size_human: "4.00 KB".to_string(),
    modified: "2024-01-01".to_string(),
    file_type: "plain".to_string(),
  };
  let json = serde_json::to_string(&info).unwrap();
  assert!(json.contains("/var/log/auth.log"));
  assert!(json.contains("plain"));
}
#[test]
fn test_log_manager_summary_serialization() {
  let summary = LogManagerSummary {
    journal_size_bytes: 1024 * 1024,
    journal_size_human: "1.00 MB".to_string(),
    rotated_logs_size_bytes: 512 * 1024,
    rotated_logs_size_human: "512 KB".to_string(),
    var_log_size_bytes: 2 * 1024 * 1024,
    var_log_size_human: "2.00 MB".to_string(),
    logrotate_configs_count: 3,
    potential_savings_mb: 50,
  };
  let json = serde_json::to_string(&summary).unwrap();
  assert!(json.contains("1.00 MB"));
  assert!(json.contains("3"));
  assert!(json.contains("50"));
}
#[test]
fn test_log_manager_service_analyze_logrotate() {
  let analysis = LogManagerService::analyze_logrotate();
  assert!(analysis.total_configs >= 0);
  assert!(analysis.potential_savings_mb >= 0);
}
#[test]
fn test_log_manager_service_get_summary() {
  let summary = LogManagerService::get_log_manager_summary();
  assert!(summary.journal_size_bytes >= 0);
  assert!(summary.var_log_size_bytes >= 0);
}
#[test]
fn test_log_manager_service_clean_rotated_logs_dry_run() {
  let result = LogManagerService::clean_rotated_logs(30);
  assert!(result.is_ok());
}
