/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';
import { LoggerService } from '@services/logger.service';

/* models */
import { JunkCategorySummary, JunkItem } from '@models/junk-cleaner.model';

@Injectable({
  providedIn: 'root',
})
export class JunkCleanerService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'JunkCleanerService', 'init', 'JunkCleanerService initialized');
  }

  async getJunkSummary(): Promise<JunkCategorySummary[]> {
    this.logger.logInfo('service', 'JunkCleanerService', 'getJunkSummary', 'Getting junk summary');
    try {
      const result = await this.api.invoke<JunkCategorySummary[]>('get_junk_summary');
      this.logger.logInfo(
        'service',
        'JunkCleanerService',
        'getJunkSummary',
        'Junk summary retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'JunkCleanerService',
        'getJunkSummary',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async scanBrowserCaches(): Promise<JunkItem[]> {
    this.logger.logInfo(
      'service',
      'JunkCleanerService',
      'scanBrowserCaches',
      'Scanning browser caches'
    );
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_browser_caches');
      this.logger.logInfo(
        'service',
        'JunkCleanerService',
        'scanBrowserCaches',
        'Browser caches scanned',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'JunkCleanerService',
        'scanBrowserCaches',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async scanThumbnailCaches(): Promise<JunkItem[]> {
    this.logger.logInfo(
      'service',
      'JunkCleanerService',
      'scanThumbnailCaches',
      'Scanning thumbnail caches'
    );
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_thumbnail_caches');
      this.logger.logInfo(
        'service',
        'JunkCleanerService',
        'scanThumbnailCaches',
        'Thumbnail caches scanned',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'JunkCleanerService',
        'scanThumbnailCaches',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async scanApplicationCaches(): Promise<JunkItem[]> {
    this.logger.logInfo(
      'service',
      'JunkCleanerService',
      'scanApplicationCaches',
      'Scanning application caches'
    );
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_application_caches');
      this.logger.logInfo(
        'service',
        'JunkCleanerService',
        'scanApplicationCaches',
        'Application caches scanned',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'JunkCleanerService',
        'scanApplicationCaches',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async scanSystemTemp(): Promise<JunkItem[]> {
    this.logger.logInfo('service', 'JunkCleanerService', 'scanSystemTemp', 'Scanning system temp');
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_system_temp');
      this.logger.logInfo(
        'service',
        'JunkCleanerService',
        'scanSystemTemp',
        'System temp scanned',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'JunkCleanerService',
        'scanSystemTemp',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async scanLogRotations(): Promise<JunkItem[]> {
    this.logger.logInfo(
      'service',
      'JunkCleanerService',
      'scanLogRotations',
      'Scanning log rotations'
    );
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_log_rotations');
      this.logger.logInfo(
        'service',
        'JunkCleanerService',
        'scanLogRotations',
        'Log rotations scanned',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'JunkCleanerService',
        'scanLogRotations',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanJunkCategory(category: string): Promise<string> {
    this.logger.logInfo(
      'service',
      'JunkCleanerService',
      'cleanJunkCategory',
      'Cleaning junk category',
      { category }
    );
    try {
      const result = await this.api.invoke<string>('clean_junk_category', { category });
      this.logger.logInfo(
        'service',
        'JunkCleanerService',
        'cleanJunkCategory',
        'Junk category cleaned',
        { category }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'JunkCleanerService',
        'cleanJunkCategory',
        'Operation failed',
        error as Error,
        { category }
      );
      throw error;
    }
  }
}
