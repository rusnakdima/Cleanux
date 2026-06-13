/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';

/* models */
import { LogSummary, LogEntry, LogCategorySummary } from '@models/log-analyzer.model';

@Injectable({
  providedIn: 'root',
})
export class LogAnalyzerService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'LogAnalyzerService', 'init', 'LogAnalyzerService initialized');
  }

  async getLogSummary(): Promise<LogSummary> {
    this.logger.logInfo('service', 'LogAnalyzerService', 'getLogSummary', 'Getting log summary');
    try {
      const result = await this.api.invoke<LogSummary>('get_log_summary');
      this.logger.logInfo(
        'service',
        'LogAnalyzerService',
        'getLogSummary',
        'Log summary retrieved'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogAnalyzerService',
        'getLogSummary',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getLogEntries(category: string): Promise<LogEntry[]> {
    this.logger.logInfo('service', 'LogAnalyzerService', 'getLogEntries', 'Getting log entries', {
      category,
    });
    try {
      const result = await this.api.invoke<LogEntry[]>('get_log_entries', { category });
      this.logger.logInfo(
        'service',
        'LogAnalyzerService',
        'getLogEntries',
        'Log entries retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogAnalyzerService',
        'getLogEntries',
        'Operation failed',
        error as Error,
        { category }
      );
      throw error;
    }
  }

  async cleanOldLogs(days: number): Promise<string> {
    this.logger.logInfo('service', 'LogAnalyzerService', 'cleanOldLogs', 'Cleaning old logs', {
      days,
    });
    try {
      const result = await this.api.invoke<string>('clean_old_logs', { days });
      this.logger.logInfo('service', 'LogAnalyzerService', 'cleanOldLogs', 'Old logs cleaned');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogAnalyzerService',
        'cleanOldLogs',
        'Operation failed',
        error as Error,
        { days }
      );
      throw error;
    }
  }
}
