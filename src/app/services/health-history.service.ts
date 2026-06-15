/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';
import { LoggerService } from './logger.service';

/* models */
import { HealthSnapshot, HealthTrend } from '@models/health-history.model';

@Injectable({
  providedIn: 'root',
})
export class HealthHistoryService {
  private api = inject(ApiService);
  private loggingService = new LoggerService();

  constructor() {
    this.loggingService.info('HealthHistoryService initialized');
  }

  async saveHealthSnapshot(
    healthScore: number,
    cacheSize: number,
    trashSize: number,
    logSize: number,
    largeFilesCount: number
  ): Promise<{ id: number }> {
    this.loggingService.info('Saving health snapshot', { healthScore });
    try {
      const result = await this.api.invoke<{ id: number }>('save_health_snapshot', {
        health_score: healthScore,
        cache_size: cacheSize,
        trash_size: trashSize,
        log_size: logSize,
        large_files_count: largeFilesCount,
      });
      this.loggingService.info('Health snapshot saved', { id: result.id });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getHealthHistory(days: number): Promise<HealthSnapshot[]> {
    this.loggingService.info('Getting health history', { days });
    try {
      const result = await this.api.invoke<HealthSnapshot[]>('get_health_history', { days });
      this.loggingService.info('Health history retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getHealthTrends(days: number): Promise<HealthTrend> {
    this.loggingService.info('Getting health trends', { days });
    try {
      const result = await this.api.invoke<HealthTrend>('get_health_trends', { days });
      this.loggingService.info('Health trends retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }
}
