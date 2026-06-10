import { inject } from '@angular/core';
import { ResolveFn } from '@angular/router';
import { ApiService } from '@services/api.service';
import { ScanSummary } from '@models/system.model';

interface DashboardData {
  cacheSummary: ScanSummary;
  trashSummary: ScanSummary;
  logSummary: ScanSummary;
  largeFilesSummary: ScanSummary;
}

export const dashboardDataResolver: ResolveFn<DashboardData> = async () => {
  const api = inject(ApiService);

  try {
    const [cacheSummary, trashSummary, logSummary, largeFilesSummary] = await Promise.all([
      api.invoke<ScanSummary>('getCacheSummary'),
      api.invoke<ScanSummary>('getTrashSummary'),
      api.invoke<ScanSummary>('getLogSummary'),
      api.invoke<ScanSummary>('getLargeFilesSummary'),
    ]);
    return { cacheSummary, trashSummary, logSummary, largeFilesSummary };
  } catch (error) {
    throw error;
  }
};
