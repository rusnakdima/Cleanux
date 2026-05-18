/* sys lib */
import { Injectable, inject, signal } from '@angular/core';

/* services */
import { ApiService } from './api.service';

/* models */
import { PackageCacheInfo } from '@models/package-manager.model';

@Injectable({
  providedIn: 'root',
})
export class PackageManagerService {
  private api = inject(ApiService);

  readonly cacheInfo = signal<PackageCacheInfo[]>([]);
  readonly loading = signal(false);

  async getPackageCacheInfo(): Promise<PackageCacheInfo[]> {
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{
        status: string;
        message: string;
        data: PackageCacheInfo[];
      }>('get_package_cache_info');
      const info = response.data || [];
      this.cacheInfo.set(info);
      return info;
    } finally {
      this.loading.set(false);
    }
  }

  async cleanPackageCache(manager: string): Promise<string> {
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ status: string; message: string }>(
        'clean_package_cache',
        { manager }
      );
      if (response.status === 'success') {
        await this.getPackageCacheInfo();
      }
      return response.message;
    } finally {
      this.loading.set(false);
    }
  }

  async cleanAllPackageCaches(): Promise<string[]> {
    const managers = this.cacheInfo().map((info) => info.name);
    const results: string[] = [];
    for (const manager of managers) {
      try {
        const result = await this.cleanPackageCache(manager);
        results.push(`${manager}: ${result}`);
      } catch (error) {
        results.push(`${manager}: Failed to clean`);
      }
    }
    return results;
  }
}
