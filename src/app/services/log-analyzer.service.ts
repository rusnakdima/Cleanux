/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';

/* models */
import { LogSummary, LogEntry, LogCategorySummary } from '@models/log-analyzer.model';

@Injectable({
  providedIn: 'root',
})
export class LogAnalyzerService {
  private api = inject(ApiService);

  async getLogSummary(): Promise<LogSummary> {
    return await this.api.invoke<LogSummary>('get_log_summary');
  }

  async getLogEntries(category: string): Promise<LogEntry[]> {
    return await this.api.invoke<LogEntry[]>('get_log_entries', { category });
  }

  async cleanOldLogs(days: number): Promise<string> {
    return await this.api.invoke<string>('clean_old_logs', { days });
  }
}
