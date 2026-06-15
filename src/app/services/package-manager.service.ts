/* sys lib */
import { Injectable, signal, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';
import { LoggerService } from './logger.service';

/* models */
import { PackageCacheInfo } from '@models/package-manager.model';

@Injectable({
  providedIn: 'root',
})
export class PackageManagerService {
  private api = inject(ApiService);
  private loggingService = new LoggerService();

  readonly cacheInfo = signal<PackageCacheInfo[]>([]);
  readonly loading = signal(false);

  constructor() {
    this.loggingService.info('PackageManagerService initialized');
  }

  async getPackageCacheInfo(): Promise<PackageCacheInfo[]> {
    this.loggingService.info('Getting package cache info');
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{
        status: string;
        message: string;
        data: PackageCacheInfo[];
      }>('get_package_cache_info');
      const info = response.data || [];
      this.cacheInfo.set(info);
      this.loggingService.info('Package cache info retrieved', { count: info.length });
      return info;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async cleanPackageCache(manager: string): Promise<string> {
    this.loggingService.info('Cleaning package cache', { manager });
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ status: string; message: string }>(
        'clean_package_cache',
        {
          manager,
        }
      );
      if (response.status === 'success') {
        await this.getPackageCacheInfo();
      }
      this.loggingService.info('Package cache cleaned', { manager });
      return response.message;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { manager });
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async cleanAllPackageCaches(): Promise<string[]> {
    this.loggingService.info('Cleaning all package caches');
    const managers = this.cacheInfo().map((info) => info.name);
    const results: string[] = [];
    for (const manager of managers) {
      try {
        const result = await this.cleanPackageCache(manager);
        results.push(`${manager}: ${result}`);
      } catch (error) {
        results.push(`${manager}: Failed to clean`);
        this.loggingService.error('Operation failed', error as Error, { manager });
      }
    }
    this.loggingService.info('All package caches cleaned', { count: results.length });
    return results;
  }
}
