import { Injectable } from '@angular/core';
import { BaseApiService } from './base-api.service';
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
export class LogManagerService extends BaseApiService {
  async getJournalSize(): Promise<number> {
    return await this.call<number>('get_journal_size');
  }

  async getJournalUsage(): Promise<JournalInfo> {
    return await this.call<JournalInfo>('get_journal_usage');
  }

  async vacuumJournal(sizeMb: number): Promise<string> {
    return await this.call<string>('vacuum_journal', { sizeMb });
  }

  async vacuumJournalByDays(days: number): Promise<string> {
    return await this.call<string>('vacuum_journal_by_days', { days });
  }

  async getRotatedLogsSize(): Promise<number> {
    return await this.call<number>('get_rotated_logs_size');
  }

  async getRotatedLogs(): Promise<RotatedLogInfo[]> {
    return await this.call<RotatedLogInfo[]>('get_rotated_logs');
  }

  async cleanRotatedLogs(days: number): Promise<string> {
    return await this.call<string>('clean_rotated_logs', { days });
  }

  async getLogrotateConfigs(): Promise<LogrotateConfig[]> {
    return await this.call<LogrotateConfig[]>('get_logrotate_configs');
  }

  async analyzeLogrotate(): Promise<LogrotateAnalysis> {
    return await this.call<LogrotateAnalysis>('analyze_logrotate');
  }

  async getVarLogUsage(): Promise<VarLogUsage> {
    return await this.call<VarLogUsage>('get_var_log_usage');
  }

  async getLargestLogFiles(limit: number): Promise<LogFileInfo[]> {
    return await this.call<LogFileInfo[]>('get_largest_log_files', { limit });
  }

  async getLogManagerSummary(): Promise<LogManagerSummary> {
    return await this.call<LogManagerSummary>('get_log_manager_summary');
  }
}
