export interface JournalInfo {
  size_bytes: number;
  size_human: string;
  oldest_entry: string | null;
  newest_entry: string | null;
  is_active: boolean;
}

export interface RotatedLogInfo {
  path: string;
  size_bytes: number;
  size_human: string;
  modified: string;
  compression_ratio: number | null;
}

export interface LogrotateConfig {
  path: string;
  enabled: boolean;
  schedule: string | null;
  max_size: string | null;
  max_age: string | null;
  compress: boolean;
  rotate_count: number | null;
}

export interface LogrotateAnalysis {
  total_configs: number;
  enabled_configs: number;
  configs: LogrotateConfig[];
  potential_savings_mb: number;
  issues: string[];
}

export interface VarLogUsage {
  total_bytes: number;
  total_human: string;
  file_count: number;
  directory_count: number;
}

export interface LogFileInfo {
  path: string;
  size_bytes: number;
  size_human: string;
  modified: string;
  file_type: string;
}

export interface LogManagerSummary {
  journal_size_bytes: number;
  journal_size_human: string;
  rotated_logs_size_bytes: number;
  rotated_logs_size_human: string;
  var_log_size_bytes: number;
  var_log_size_human: string;
  logrotate_configs_count: number;
  potential_savings_mb: number;
}
