import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';
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
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'LogManagerService', 'init', 'LogManagerService initialized');
  }

  async getJournalSize(): Promise<number> {
    this.logger.logInfo('service', 'LogManagerService', 'getJournalSize', 'Getting journal size');
    try {
      const result = await this.api.invoke<number>('get_journal_size');
      this.logger.logInfo(
        'service',
        'LogManagerService',
        'getJournalSize',
        'Journal size retrieved',
        { size: result }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogManagerService',
        'getJournalSize',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getJournalUsage(): Promise<JournalInfo> {
    this.logger.logInfo('service', 'LogManagerService', 'getJournalUsage', 'Getting journal usage');
    try {
      const result = await this.api.invoke<JournalInfo>('get_journal_usage');
      this.logger.logInfo(
        'service',
        'LogManagerService',
        'getJournalUsage',
        'Journal usage retrieved'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogManagerService',
        'getJournalUsage',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async vacuumJournal(sizeMb: number): Promise<string> {
    this.logger.logInfo('service', 'LogManagerService', 'vacuumJournal', 'Vacuuming journal', {
      sizeMb,
    });
    try {
      const result = await this.api.invoke<string>('vacuum_journal', { sizeMb });
      this.logger.logInfo('service', 'LogManagerService', 'vacuumJournal', 'Journal vacuumed');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogManagerService',
        'vacuumJournal',
        'Operation failed',
        error as Error,
        { sizeMb }
      );
      throw error;
    }
  }

  async vacuumJournalByDays(days: number): Promise<string> {
    this.logger.logInfo(
      'service',
      'LogManagerService',
      'vacuumJournalByDays',
      'Vacuuming journal by days',
      { days }
    );
    try {
      const result = await this.api.invoke<string>('vacuum_journal_by_days', { days });
      this.logger.logInfo(
        'service',
        'LogManagerService',
        'vacuumJournalByDays',
        'Journal vacuumed by days'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogManagerService',
        'vacuumJournalByDays',
        'Operation failed',
        error as Error,
        { days }
      );
      throw error;
    }
  }

  async getRotatedLogsSize(): Promise<number> {
    this.logger.logInfo(
      'service',
      'LogManagerService',
      'getRotatedLogsSize',
      'Getting rotated logs size'
    );
    try {
      const result = await this.api.invoke<number>('get_rotated_logs_size');
      this.logger.logInfo(
        'service',
        'LogManagerService',
        'getRotatedLogsSize',
        'Rotated logs size retrieved',
        { size: result }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogManagerService',
        'getRotatedLogsSize',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getRotatedLogs(): Promise<RotatedLogInfo[]> {
    this.logger.logInfo('service', 'LogManagerService', 'getRotatedLogs', 'Getting rotated logs');
    try {
      const result = await this.api.invoke<RotatedLogInfo[]>('get_rotated_logs');
      this.logger.logInfo(
        'service',
        'LogManagerService',
        'getRotatedLogs',
        'Rotated logs retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogManagerService',
        'getRotatedLogs',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanRotatedLogs(days: number): Promise<string> {
    this.logger.logInfo(
      'service',
      'LogManagerService',
      'cleanRotatedLogs',
      'Cleaning rotated logs',
      { days }
    );
    try {
      const result = await this.api.invoke<string>('clean_rotated_logs', { days });
      this.logger.logInfo(
        'service',
        'LogManagerService',
        'cleanRotatedLogs',
        'Rotated logs cleaned'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogManagerService',
        'cleanRotatedLogs',
        'Operation failed',
        error as Error,
        { days }
      );
      throw error;
    }
  }

  async getLogrotateConfigs(): Promise<LogrotateConfig[]> {
    this.logger.logInfo(
      'service',
      'LogManagerService',
      'getLogrotateConfigs',
      'Getting logrotate configs'
    );
    try {
      const result = await this.api.invoke<LogrotateConfig[]>('get_logrotate_configs');
      this.logger.logInfo(
        'service',
        'LogManagerService',
        'getLogrotateConfigs',
        'Logrotate configs retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogManagerService',
        'getLogrotateConfigs',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async analyzeLogrotate(): Promise<LogrotateAnalysis> {
    this.logger.logInfo('service', 'LogManagerService', 'analyzeLogrotate', 'Analyzing logrotate');
    try {
      const result = await this.api.invoke<LogrotateAnalysis>('analyze_logrotate');
      this.logger.logInfo('service', 'LogManagerService', 'analyzeLogrotate', 'Logrotate analyzed');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogManagerService',
        'analyzeLogrotate',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getVarLogUsage(): Promise<VarLogUsage> {
    this.logger.logInfo('service', 'LogManagerService', 'getVarLogUsage', 'Getting var log usage');
    try {
      const result = await this.api.invoke<VarLogUsage>('get_var_log_usage');
      this.logger.logInfo(
        'service',
        'LogManagerService',
        'getVarLogUsage',
        'Var log usage retrieved'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogManagerService',
        'getVarLogUsage',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getLargestLogFiles(limit: number): Promise<LogFileInfo[]> {
    this.logger.logInfo(
      'service',
      'LogManagerService',
      'getLargestLogFiles',
      'Getting largest log files',
      { limit }
    );
    try {
      const result = await this.api.invoke<LogFileInfo[]>('get_largest_log_files', { limit });
      this.logger.logInfo(
        'service',
        'LogManagerService',
        'getLargestLogFiles',
        'Largest log files retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogManagerService',
        'getLargestLogFiles',
        'Operation failed',
        error as Error,
        { limit }
      );
      throw error;
    }
  }

  async getLogManagerSummary(): Promise<LogManagerSummary> {
    this.logger.logInfo(
      'service',
      'LogManagerService',
      'getLogManagerSummary',
      'Getting log manager summary'
    );
    try {
      const result = await this.api.invoke<LogManagerSummary>('get_log_manager_summary');
      this.logger.logInfo(
        'service',
        'LogManagerService',
        'getLogManagerSummary',
        'Log manager summary retrieved'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'LogManagerService',
        'getLogManagerSummary',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }
}
