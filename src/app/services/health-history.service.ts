/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';

/* models */
import { HealthSnapshot, HealthTrend } from '@entities/health-history.model';

@Injectable({
  providedIn: 'root',
})
export class HealthHistoryService {
  private api = inject(ApiService);

  async saveHealthSnapshot(
    healthScore: number,
    cacheSize: number,
    trashSize: number,
    logSize: number,
    largeFilesCount: number
  ): Promise<{ id: number }> {
    const result = await this.api.invoke<{ id: number }>('save_health_snapshot', {
      health_score: healthScore,
      cache_size: cacheSize,
      trash_size: trashSize,
      log_size: logSize,
      large_files_count: largeFilesCount,
    });
    return result;
  }

  async getHealthHistory(days: number): Promise<HealthSnapshot[]> {
    const result = await this.api.invoke<HealthSnapshot[]>('get_health_history', { days });
    return result;
  }

  async getHealthTrends(days: number): Promise<HealthTrend> {
    const result = await this.api.invoke<HealthTrend>('get_health_trends', { days });
    return result;
  }
}
