import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';
import {
  JournalInfo,
  LogManagerSummary,
  LogFileInfo,
  LogrotateAnalysis,
  LogrotateConfig,
  RotatedLogInfo,
  VarLogUsage,
} from '@models/log-manager.model';

export type {
  JournalInfo,
  LogManagerSummary,
  LogFileInfo,
  LogrotateAnalysis,
  LogrotateConfig,
  RotatedLogInfo,
  VarLogUsage,
} from '@models/log-manager.model';

@Injectable({
  providedIn: 'root',
})
export class LogManagerService {
  private api = inject(ApiService);
  private loggingService = getLoggingService();

  constructor() {
    this.loggingService.info('LogManagerService initialized');
  }

  async getJournalSize(): Promise<number> {
    this.loggingService.info('Getting journal size');
    try {
      const result = await this.api.invoke<number>('get_journal_size');
      this.loggingService.info('Journal size retrieved', { size: result });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getJournalUsage(): Promise<JournalInfo> {
    this.loggingService.info('Getting journal usage');
    try {
      const result = await this.api.invoke<JournalInfo>('get_journal_usage');
      this.loggingService.info('Journal usage retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async vacuumJournal(sizeMb: number): Promise<string> {
    this.loggingService.info('Vacuuming journal', { sizeMb });
    try {
      const result = await this.api.invoke<string>('vacuum_journal', { sizeMb });
      this.loggingService.info('Journal vacuumed');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { sizeMb });
      throw error;
    }
  }

  async vacuumJournalByDays(days: number): Promise<string> {
    this.loggingService.info('Vacuuming journal by days', { days });
    try {
      const result = await this.api.invoke<string>('vacuum_journal_by_days', { days });
      this.loggingService.info('Journal vacuumed by days');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { days });
      throw error;
    }
  }

  async getRotatedLogsSize(): Promise<number> {
    this.loggingService.info('Getting rotated logs size');
    try {
      const result = await this.api.invoke<number>('get_rotated_logs_size');
      this.loggingService.info('Rotated logs size retrieved', { size: result });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getRotatedLogs(): Promise<RotatedLogInfo[]> {
    this.loggingService.info('Getting rotated logs');
    try {
      const result = await this.api.invoke<RotatedLogInfo[]>('get_rotated_logs');
      this.loggingService.info('Rotated logs retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanRotatedLogs(days: number): Promise<string> {
    this.loggingService.info('Cleaning rotated logs', { days });
    try {
      const result = await this.api.invoke<string>('clean_rotated_logs', { days });
      this.loggingService.info('Rotated logs cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { days });
      throw error;
    }
  }

  async getLogrotateConfigs(): Promise<LogrotateConfig[]> {
    this.loggingService.info('Getting logrotate configs');
    try {
      const result = await this.api.invoke<LogrotateConfig[]>('get_logrotate_configs');
      this.loggingService.info('Logrotate configs retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async analyzeLogrotate(): Promise<LogrotateAnalysis> {
    this.loggingService.info('Analyzing logrotate');
    try {
      const result = await this.api.invoke<LogrotateAnalysis>('analyze_logrotate');
      this.loggingService.info('Logrotate analyzed');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getVarLogUsage(): Promise<VarLogUsage> {
    this.loggingService.info('Getting var log usage');
    try {
      const result = await this.api.invoke<VarLogUsage>('get_var_log_usage');
      this.loggingService.info('Var log usage retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getLargestLogFiles(limit: number): Promise<LogFileInfo[]> {
    this.loggingService.info('Getting largest log files', { limit });
    try {
      const result = await this.api.invoke<LogFileInfo[]>('get_largest_log_files', { limit });
      this.loggingService.info('Largest log files retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { limit });
      throw error;
    }
  }

  async getLogManagerSummary(): Promise<LogManagerSummary> {
    this.loggingService.info('Getting log manager summary');
    try {
      const result = await this.api.invoke<LogManagerSummary>('get_log_manager_summary');
      this.loggingService.info('Log manager summary retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }
}
