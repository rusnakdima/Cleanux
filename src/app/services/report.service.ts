/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';

/* models */
import { CleaningReport, SnapshotComparison } from '@models/report.model';

@Injectable({
  providedIn: 'root',
})
export class ReportService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'ReportService', 'init', 'ReportService initialized');
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
    this.logger.logInfo(
      'service',
      'ReportService',
      'generateCleaningReport',
      'Generating cleaning report',
      { itemsCleaned, spaceReclaimed }
    );
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
      this.logger.logInfo(
        'service',
        'ReportService',
        'generateCleaningReport',
        'Cleaning report generated',
        { id: result.id }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'ReportService',
        'generateCleaningReport',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getCleaningHistory(limit?: number): Promise<CleaningReport[]> {
    this.logger.logInfo(
      'service',
      'ReportService',
      'getCleaningHistory',
      'Getting cleaning history',
      { limit }
    );
    try {
      const result = await this.api.invoke<CleaningReport[]>('get_cleaning_history', { limit });
      this.logger.logInfo(
        'service',
        'ReportService',
        'getCleaningHistory',
        'Cleaning history retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'ReportService',
        'getCleaningHistory',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async exportToHtml(reportId: number): Promise<{ html: string }> {
    this.logger.logInfo('service', 'ReportService', 'exportToHtml', 'Exporting report to HTML', {
      reportId,
    });
    try {
      const result = await this.api.invoke<{ html: string }>('export_to_html', {
        report_id: reportId,
      });
      this.logger.logInfo('service', 'ReportService', 'exportToHtml', 'Report exported to HTML');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'ReportService',
        'exportToHtml',
        'Operation failed',
        error as Error,
        { reportId }
      );
      throw error;
    }
  }

  async compareSnapshots(beforeId: number, afterId: number): Promise<SnapshotComparison> {
    this.logger.logInfo('service', 'ReportService', 'compareSnapshots', 'Comparing snapshots', {
      beforeId,
      afterId,
    });
    try {
      const result = await this.api.invoke<SnapshotComparison>('compare_snapshots', {
        before_id: beforeId,
        after_id: afterId,
      });
      this.logger.logInfo('service', 'ReportService', 'compareSnapshots', 'Snapshots compared');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'ReportService',
        'compareSnapshots',
        'Operation failed',
        error as Error,
        { beforeId, afterId }
      );
      throw error;
    }
  }
}
