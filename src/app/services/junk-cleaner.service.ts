/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';

/* models */
import { JunkCategorySummary, JunkItem } from '@models/junk-cleaner.model';

@Injectable({
  providedIn: 'root',
})
export class JunkCleanerService {
  private api = inject(ApiService);

  async getJunkSummary(): Promise<JunkCategorySummary[]> {
    return this.api.invoke<JunkCategorySummary[]>('get_junk_summary');
  }

  async scanBrowserCaches(): Promise<JunkItem[]> {
    return this.api.invoke<JunkItem[]>('scan_browser_caches');
  }

  async scanThumbnailCaches(): Promise<JunkItem[]> {
    return this.api.invoke<JunkItem[]>('scan_thumbnail_caches');
  }

  async scanApplicationCaches(): Promise<JunkItem[]> {
    return this.api.invoke<JunkItem[]>('scan_application_caches');
  }

  async scanSystemTemp(): Promise<JunkItem[]> {
    return this.api.invoke<JunkItem[]>('scan_system_temp');
  }

  async scanLogRotations(): Promise<JunkItem[]> {
    return this.api.invoke<JunkItem[]>('scan_log_rotations');
  }

  async cleanJunkCategory(category: string): Promise<string> {
    return this.api.invoke<string>('clean_junk_category', { category });
  }
}
