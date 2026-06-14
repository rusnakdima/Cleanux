/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';

/* models */
import { JunkCategorySummary, JunkItem } from '@models/junk-cleaner.model';

@Injectable({
  providedIn: 'root',
})
export class JunkCleanerService {
  private api = inject(ApiService);
  private loggingService = getLoggingService();

  constructor() {
    this.loggingService.info('JunkCleanerService initialized');
  }

  async getJunkSummary(): Promise<JunkCategorySummary[]> {
    this.loggingService.info('Getting junk summary');
    try {
      const result = await this.api.invoke<JunkCategorySummary[]>('get_junk_summary');
      this.loggingService.info('Junk summary retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async scanBrowserCaches(): Promise<JunkItem[]> {
    this.loggingService.info('Scanning browser caches');
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_browser_caches');
      this.loggingService.info('Browser caches scanned', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async scanThumbnailCaches(): Promise<JunkItem[]> {
    this.loggingService.info('Scanning thumbnail caches');
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_thumbnail_caches');
      this.loggingService.info('Thumbnail caches scanned', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async scanApplicationCaches(): Promise<JunkItem[]> {
    this.loggingService.info('Scanning application caches');
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_application_caches');
      this.loggingService.info('Application caches scanned', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async scanSystemTemp(): Promise<JunkItem[]> {
    this.loggingService.info('Scanning system temp');
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_system_temp');
      this.loggingService.info('System temp scanned', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async scanLogRotations(): Promise<JunkItem[]> {
    this.loggingService.info('Scanning log rotations');
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_log_rotations');
      this.loggingService.info('Log rotations scanned', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanJunkCategory(category: string): Promise<string> {
    this.loggingService.info('Cleaning junk category', { category });
    try {
      const result = await this.api.invoke<string>('clean_junk_category', { category });
      this.loggingService.info('Junk category cleaned', { category });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { category });
      throw error;
    }
  }
}
