/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';

/* models */
import { CleaningReport, SnapshotComparison } from '@models/report.model';

@Injectable({
  providedIn: 'root',
})
export class ReportService {
  private api = inject(ApiService);
  private loggingService = getLoggingService();

  constructor() {
    this.loggingService.info('ReportService initialized');
  }

  async generateCleaningReport(
    itemsCleaned: number,
    spaceReclaimed: number,
    duration: number,
    cacheItems: number,
    trashItems: number,
    logItems: number,
    largeFileItems: number,
    duplicateItems: number
  ): Promise<{ id: number }> {
    this.loggingService.info('Generating cleaning report', { itemsCleaned, spaceReclaimed });
    try {
      const result = await this.api.invoke<{ id: number }>('generate_cleaning_report', {
        items_cleaned: itemsCleaned,
        space_reclaimed: spaceReclaimed,
        duration: duration,
        cache_items: cacheItems,
        trash_items: trashItems,
        log_items: logItems,
        large_file_items: largeFileItems,
        duplicate_items: duplicateItems,
      });
      this.loggingService.info('Cleaning report generated', { id: result.id });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getCleaningHistory(limit?: number): Promise<CleaningReport[]> {
    this.loggingService.info('Getting cleaning history', { limit });
    try {
      const result = await this.api.invoke<CleaningReport[]>('get_cleaning_history', { limit });
      this.loggingService.info('Cleaning history retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async exportToHtml(reportId: number): Promise<{ html: string }> {
    this.loggingService.info('Exporting report to HTML', { reportId });
    try {
      const result = await this.api.invoke<{ html: string }>('export_to_html', {
        report_id: reportId,
      });
      this.loggingService.info('Report exported to HTML');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { reportId });
      throw error;
    }
  }

  async compareSnapshots(beforeId: number, afterId: number): Promise<SnapshotComparison> {
    this.loggingService.info('Comparing snapshots', { beforeId, afterId });
    try {
      const result = await this.api.invoke<SnapshotComparison>('compare_snapshots', {
        before_id: beforeId,
        after_id: afterId,
      });
      this.loggingService.info('Snapshots compared');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { beforeId, afterId });
      throw error;
    }
  }
}
