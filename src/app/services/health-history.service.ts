/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';

/* models */
import { HealthSnapshot, HealthTrend } from '@models/health-history.model';

@Injectable({
  providedIn: 'root',
})
export class HealthHistoryService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo(
      'service',
      'HealthHistoryService',
      'init',
      'HealthHistoryService initialized'
    );
  }

  async saveHealthSnapshot(
    healthScore: number,
    cacheSize: number,
    trashSize: number,
    logSize: number,
    largeFilesCount: number
  ): Promise<{ id: number }> {
    this.logger.logInfo(
      'service',
      'HealthHistoryService',
      'saveHealthSnapshot',
      'Saving health snapshot',
      { healthScore }
    );
    try {
      const result = await this.api.invoke<{ id: number }>('save_health_snapshot', {
        health_score: healthScore,
        cache_size: cacheSize,
        trash_size: trashSize,
        log_size: logSize,
        large_files_count: largeFilesCount,
      });
      this.logger.logInfo(
        'service',
        'HealthHistoryService',
        'saveHealthSnapshot',
        'Health snapshot saved',
        { id: result.id }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'HealthHistoryService',
        'saveHealthSnapshot',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getHealthHistory(days: number): Promise<HealthSnapshot[]> {
    this.logger.logInfo(
      'service',
      'HealthHistoryService',
      'getHealthHistory',
      'Getting health history',
      { days }
    );
    try {
      const result = await this.api.invoke<HealthSnapshot[]>('get_health_history', { days });
      this.logger.logInfo(
        'service',
        'HealthHistoryService',
        'getHealthHistory',
        'Health history retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'HealthHistoryService',
        'getHealthHistory',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getHealthTrends(days: number): Promise<HealthTrend> {
    this.logger.logInfo(
      'service',
      'HealthHistoryService',
      'getHealthTrends',
      'Getting health trends',
      { days }
    );
    try {
      const result = await this.api.invoke<HealthTrend>('get_health_trends', { days });
      this.logger.logInfo(
        'service',
        'HealthHistoryService',
        'getHealthTrends',
        'Health trends retrieved'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'HealthHistoryService',
        'getHealthTrends',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }
}
