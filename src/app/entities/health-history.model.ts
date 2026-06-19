export interface HealthSnapshot {
  id?: number;
  timestamp: string;
  health_score: number;
  cache_size: number;
  trash_size: number;
  log_size: number;
  large_files_count: number;
}

export interface HealthTrend {
  trend: string;
  change_percent: number;
  days_analyzed: number;
}
