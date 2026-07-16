/* angular */
import { Injectable, inject } from '@angular/core';

/* library */
import { InvokeWrapperService } from '@tauri-front/shared';

export interface JunkCategory {
  category: string;
  size: number;
  count: number;
}

export interface JunkFileItem {
  path: string;
  size: number;
  modified: string;
}

@Injectable({ providedIn: 'root' })
export class JunkCleanerService {
  private api = inject(InvokeWrapperService);

  async getJunkSummary(): Promise<JunkCategory[]> {
    return this.api.invoke<JunkCategory[]>('get_junk_summary');
  }

  async scanBrowserCaches(): Promise<JunkFileItem[]> {
    return this.api.invoke<JunkFileItem[]>('scan_browser_caches');
  }

  async scanThumbnailCaches(): Promise<JunkFileItem[]> {
    return this.api.invoke<JunkFileItem[]>('scan_thumbnail_caches');
  }

  async scanApplicationCaches(): Promise<JunkFileItem[]> {
    return this.api.invoke<JunkFileItem[]>('scan_application_caches');
  }

  async scanSystemTemp(): Promise<JunkFileItem[]> {
    return this.api.invoke<JunkFileItem[]>('scan_system_temp');
  }

  async scanLogRotations(): Promise<JunkFileItem[]> {
    return this.api.invoke<JunkFileItem[]>('scan_log_rotations');
  }

  async cleanJunkCategory(category: string): Promise<string> {
    return this.api.invoke<string>('clean_junk_category', { category });
  }
}
