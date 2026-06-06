/* sys lib */
import { Injectable } from '@angular/core';

/* services */
import { BaseApiService } from './base-api.service';

/* models */
import { LogSummary, LogEntry, LogCategorySummary } from '@models/log-analyzer.model';

@Injectable({
  providedIn: 'root',
})
export class LogAnalyzerService extends BaseApiService {
  async getLogSummary(): Promise<LogSummary> {
    return await this.call<LogSummary>('get_log_summary');
  }

  async getLogEntries(category: string): Promise<LogEntry[]> {
    return await this.call<LogEntry[]>('get_log_entries', { category });
  }

  async cleanOldLogs(days: number): Promise<string> {
    return await this.call<string>('clean_old_logs', { days });
  }
}
