/* sys lib */
import { Injectable } from '@angular/core';

/* services */
import { BaseApiService } from './base-api.service';

/* models */
import { CleaningReport, SnapshotComparison } from '@models/report.model';

@Injectable({
  providedIn: 'root',
})
export class ReportService extends BaseApiService {
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
    return await this.call<{ id: number }>('generate_cleaning_report', {
      items_cleaned: itemsCleaned,
      space_reclaimed: spaceReclaimed,
      duration: duration,
      cache_items: cacheItems,
      trash_items: trashItems,
      log_items: logItems,
      large_file_items: largeFileItems,
      duplicate_items: duplicateItems,
    });
  }

  async getCleaningHistory(limit?: number): Promise<CleaningReport[]> {
    return await this.call<CleaningReport[]>('get_cleaning_history', { limit });
  }

  async exportToHtml(reportId: number): Promise<{ html: string }> {
    return await this.call<{ html: string }>('export_to_html', { report_id: reportId });
  }

  async compareSnapshots(beforeId: number, afterId: number): Promise<SnapshotComparison> {
    return await this.call<SnapshotComparison>('compare_snapshots', {
      before_id: beforeId,
      after_id: afterId,
    });
  }
}
