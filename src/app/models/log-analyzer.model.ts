export type LogSeverity = 'error' | 'warning' | 'info';

export interface LogEntry {
  path: string;
  severity: LogSeverity;
  count: number;
  lastModified: string;
  size: number;
}

export interface LogCategorySummary {
  category: string;
  count: number;
  size: number;
  entries: LogEntry[];
}

export interface LogSummary {
  system: LogCategorySummary;
  application: LogCategorySummary;
  security: LogCategorySummary;
  hardware: LogCategorySummary;
}
