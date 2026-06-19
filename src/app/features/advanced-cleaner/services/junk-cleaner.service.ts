/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';

/* models */
import { JunkCategorySummary, JunkItem } from '@entities/junk-cleaner.model';

@Injectable({
  providedIn: 'root',
})
export class JunkCleanerService {
  private api = inject(ApiService);

  constructor() {}

  async getJunkSummary(): Promise<JunkCategorySummary[]> {
    try {
      const result = await this.api.invoke<JunkCategorySummary[]>('get_junk_summary');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async scanBrowserCaches(): Promise<JunkItem[]> {
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_browser_caches');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async scanThumbnailCaches(): Promise<JunkItem[]> {
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_thumbnail_caches');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async scanApplicationCaches(): Promise<JunkItem[]> {
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_application_caches');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async scanSystemTemp(): Promise<JunkItem[]> {
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_system_temp');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async scanLogRotations(): Promise<JunkItem[]> {
    try {
      const result = await this.api.invoke<JunkItem[]>('scan_log_rotations');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanJunkCategory(category: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_junk_category', { category });
      return result;
    } catch (error) {
      throw error;
    }
  }
}
