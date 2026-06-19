/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';

/* models */
import { CleaningReport, SnapshotComparison } from '@entities/report.model';

@Injectable({
  providedIn: 'root',
})
export class ReportService {
  private api = inject(ApiService);

  constructor() {}

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
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getCleaningHistory(limit?: number): Promise<CleaningReport[]> {
    try {
      const result = await this.api.invoke<CleaningReport[]>('get_cleaning_history', { limit });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async exportToHtml(reportId: number): Promise<{ html: string }> {
    try {
      const result = await this.api.invoke<{ html: string }>('export_to_html', {
        report_id: reportId,
      });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async compareSnapshots(beforeId: number, afterId: number): Promise<SnapshotComparison> {
    try {
      const result = await this.api.invoke<SnapshotComparison>('compare_snapshots', {
        before_id: beforeId,
        after_id: afterId,
      });
      return result;
    } catch (error) {
      throw error;
    }
  }
}
