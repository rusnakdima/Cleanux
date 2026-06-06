import { Injectable } from '@angular/core';
import { BaseApiService } from '@services/base-api.service';
import { JunkCategorySummary, JunkItem } from '@models/junk-cleaner.model';

@Injectable({
  providedIn: 'root',
})
export class JunkCleanerService extends BaseApiService {
  async getJunkSummary(): Promise<JunkCategorySummary[]> {
    return this.call<JunkCategorySummary[]>('get_junk_summary');
  }

  async scanBrowserCaches(): Promise<JunkItem[]> {
    return this.call<JunkItem[]>('scan_browser_caches');
  }

  async scanThumbnailCaches(): Promise<JunkItem[]> {
    return this.call<JunkItem[]>('scan_thumbnail_caches');
  }

  async scanApplicationCaches(): Promise<JunkItem[]> {
    return this.call<JunkItem[]>('scan_application_caches');
  }

  async scanSystemTemp(): Promise<JunkItem[]> {
    return this.call<JunkItem[]>('scan_system_temp');
  }

  async scanLogRotations(): Promise<JunkItem[]> {
    return this.call<JunkItem[]>('scan_log_rotations');
  }

  async cleanJunkCategory(category: string): Promise<string> {
    return this.call<string>('clean_junk_category', { category });
  }
}