/* sys lib */
import { Injectable, signal, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';

/* models */
import { PackageCacheInfo } from '@models/package-manager.model';

@Injectable({
  providedIn: 'root',
})
export class PackageManagerService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  readonly cacheInfo = signal<PackageCacheInfo[]>([]);
  readonly loading = signal(false);

  constructor() {
    this.logger.logInfo(
      'service',
      'PackageManagerService',
      'init',
      'PackageManagerService initialized'
    );
  }

  async getPackageCacheInfo(): Promise<PackageCacheInfo[]> {
    this.logger.logInfo(
      'service',
      'PackageManagerService',
      'getPackageCacheInfo',
      'Getting package cache info'
    );
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{
        status: string;
        message: string;
        data: PackageCacheInfo[];
      }>('get_package_cache_info');
      const info = response.data || [];
      this.cacheInfo.set(info);
      this.logger.logInfo(
        'service',
        'PackageManagerService',
        'getPackageCacheInfo',
        'Package cache info retrieved',
        { count: info.length }
      );
      return info;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageManagerService',
        'getPackageCacheInfo',
        'Operation failed',
        error as Error
      );
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async cleanPackageCache(manager: string): Promise<string> {
    this.logger.logInfo(
      'service',
      'PackageManagerService',
      'cleanPackageCache',
      'Cleaning package cache',
      { manager }
    );
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
      this.logger.logInfo(
        'service',
        'PackageManagerService',
        'cleanPackageCache',
        'Package cache cleaned',
        { manager }
      );
      return response.message;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageManagerService',
        'cleanPackageCache',
        'Operation failed',
        error as Error,
        { manager }
      );
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async cleanAllPackageCaches(): Promise<string[]> {
    this.logger.logInfo(
      'service',
      'PackageManagerService',
      'cleanAllPackageCaches',
      'Cleaning all package caches'
    );
    const managers = this.cacheInfo().map((info) => info.name);
    const results: string[] = [];
    for (const manager of managers) {
      try {
        const result = await this.cleanPackageCache(manager);
        results.push(`${manager}: ${result}`);
      } catch (error) {
        results.push(`${manager}: Failed to clean`);
        this.logger.logError(
          'service',
          'PackageManagerService',
          'cleanAllPackageCaches',
          'Operation failed',
          error as Error,
          { manager }
        );
      }
    }
    this.logger.logInfo(
      'service',
      'PackageManagerService',
      'cleanAllPackageCaches',
      'All package caches cleaned',
      { count: results.length }
    );
    return results;
  }
}
