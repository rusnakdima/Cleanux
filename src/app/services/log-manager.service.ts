import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';
import {
  JournalInfo,
  LogManagerSummary,
  LogFileInfo,
  LogrotateAnalysis,
  LogrotateConfig,
  RotatedLogInfo,
  VarLogUsage,
} from '@models/log-manager.model';

export {
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

  async getJournalSize(): Promise<number> {
    return await this.api.invoke<number>('get_journal_size');
  }

  async getJournalUsage(): Promise<JournalInfo> {
    return await this.api.invoke<JournalInfo>('get_journal_usage');
  }

  async vacuumJournal(sizeMb: number): Promise<string> {
    return await this.api.invoke<string>('vacuum_journal', { sizeMb });
  }

  async vacuumJournalByDays(days: number): Promise<string> {
    return await this.api.invoke<string>('vacuum_journal_by_days', { days });
  }

  async getRotatedLogsSize(): Promise<number> {
    return await this.api.invoke<number>('get_rotated_logs_size');
  }

  async getRotatedLogs(): Promise<RotatedLogInfo[]> {
    return await this.api.invoke<RotatedLogInfo[]>('get_rotated_logs');
  }

  async cleanRotatedLogs(days: number): Promise<string> {
    return await this.api.invoke<string>('clean_rotated_logs', { days });
  }

  async getLogrotateConfigs(): Promise<LogrotateConfig[]> {
    return await this.api.invoke<LogrotateConfig[]>('get_logrotate_configs');
  }

  async analyzeLogrotate(): Promise<LogrotateAnalysis> {
    return await this.api.invoke<LogrotateAnalysis>('analyze_logrotate');
  }

  async getVarLogUsage(): Promise<VarLogUsage> {
    return await this.api.invoke<VarLogUsage>('get_var_log_usage');
  }

  async getLargestLogFiles(limit: number): Promise<LogFileInfo[]> {
    return await this.api.invoke<LogFileInfo[]>('get_largest_log_files', { limit });
  }

  async getLogManagerSummary(): Promise<LogManagerSummary> {
    return await this.api.invoke<LogManagerSummary>('get_log_manager_summary');
  }
}
