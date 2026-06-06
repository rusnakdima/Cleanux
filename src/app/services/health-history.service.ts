/* sys lib */
import { Injectable } from '@angular/core';

/* services */
import { BaseApiService } from './base-api.service';

/* models */
import { HealthSnapshot, HealthTrend } from '@models/health-history.model';

@Injectable({
  providedIn: 'root',
})
export class HealthHistoryService extends BaseApiService {
  async saveHealthSnapshot(
    healthScore: number,
    cacheSize: number,
    trashSize: number,
    logSize: number,
    largeFilesCount: number
  ): Promise<{ id: number }> {
    return await this.call<{ id: number }>('save_health_snapshot', {
      health_score: healthScore,
      cache_size: cacheSize,
      trash_size: trashSize,
      log_size: logSize,
      large_files_count: largeFilesCount,
    });
  }

  async getHealthHistory(days: number): Promise<HealthSnapshot[]> {
    return await this.call<HealthSnapshot[]>('get_health_history', { days });
  }

  async getHealthTrends(days: number): Promise<HealthTrend> {
    return await this.call<HealthTrend>('get_health_trends', { days });
  }
}
