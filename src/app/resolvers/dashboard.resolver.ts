import { inject } from '@angular/core';
import { ResolveFn } from '@angular/router';
import { TauriApiService } from '@api/tauri-api.service';
import { ScanSummary } from '@models/system.model';

interface DashboardData {
  cacheSummary: ScanSummary;
  trashSummary: ScanSummary;
  logSummary: ScanSummary;
  largeFilesSummary: ScanSummary;
}

export const dashboardDataResolver: ResolveFn<DashboardData> = async () => {
  const api = inject(TauriApiService);

  try {
    const [cacheSummary, trashSummary, logSummary, largeFilesSummary] = await Promise.all([
      api.invoke<ScanSummary>('getCacheSummary'),
      api.invoke<ScanSummary>('getTrashSummary'),
      api.invoke<ScanSummary>('getLogSummary'),
      api.invoke<ScanSummary>('getLargeFilesSummary'),
    ]);
    return { cacheSummary, trashSummary, logSummary, largeFilesSummary };
  } catch (error) {
    console.error('Failed to resolve dashboard data:', error);
    return {
      cacheSummary: { totalSize: 0, fileCount: 0 },
      trashSummary: { totalSize: 0, fileCount: 0 },
      logSummary: { totalSize: 0, fileCount: 0 },
      largeFilesSummary: { totalSize: 0, fileCount: 0 },
    };
  }
};
