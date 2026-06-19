import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import {
  JournalInfo,
  LogManagerSummary,
  LogFileInfo,
  LogrotateAnalysis,
  LogrotateConfig,
  RotatedLogInfo,
  VarLogUsage,
} from '@entities/log-manager.model';

export type {
  JournalInfo,
  LogManagerSummary,
  LogFileInfo,
  LogrotateAnalysis,
  LogrotateConfig,
  RotatedLogInfo,
  VarLogUsage,
} from '@entities/log-manager.model';

@Injectable({
  providedIn: 'root',
})
export class LogManagerService {
  private api = inject(ApiService);

  constructor() {}

  async getJournalSize(): Promise<number> {
    try {
      const result = await this.api.invoke<number>('get_journal_size');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getJournalUsage(): Promise<JournalInfo> {
    try {
      const result = await this.api.invoke<JournalInfo>('get_journal_usage');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async vacuumJournal(sizeMb: number): Promise<string> {
    try {
      const result = await this.api.invoke<string>('vacuum_journal', { sizeMb });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async vacuumJournalByDays(days: number): Promise<string> {
    try {
      const result = await this.api.invoke<string>('vacuum_journal_by_days', { days });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getRotatedLogsSize(): Promise<number> {
    try {
      const result = await this.api.invoke<number>('get_rotated_logs_size');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getRotatedLogs(): Promise<RotatedLogInfo[]> {
    try {
      const result = await this.api.invoke<RotatedLogInfo[]>('get_rotated_logs');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanRotatedLogs(days: number): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_rotated_logs', { days });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getLogrotateConfigs(): Promise<LogrotateConfig[]> {
    try {
      const result = await this.api.invoke<LogrotateConfig[]>('get_logrotate_configs');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async analyzeLogrotate(): Promise<LogrotateAnalysis> {
    try {
      const result = await this.api.invoke<LogrotateAnalysis>('analyze_logrotate');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getVarLogUsage(): Promise<VarLogUsage> {
    try {
      const result = await this.api.invoke<VarLogUsage>('get_var_log_usage');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getLargestLogFiles(limit: number): Promise<LogFileInfo[]> {
    try {
      const result = await this.api.invoke<LogFileInfo[]>('get_largest_log_files', { limit });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getLogManagerSummary(): Promise<LogManagerSummary> {
    try {
      const result = await this.api.invoke<LogManagerSummary>('get_log_manager_summary');
      return result;
    } catch (error) {
      throw error;
    }
  }
}
