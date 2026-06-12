import { Injectable, inject, signal } from '@angular/core';
import { ApiService } from '@services/api.service';
import { LoggerService } from '@services/logger.service';

export interface PackageManagerSummary {
  aptAvailable: boolean;
  aptCacheSize: number;
  aptAutoremoveSize: number;
  aptOrphanedCount: number;
  aptPartialDownloads: number;
  dnfAvailable: boolean;
  dnfCacheSize: number;
  pacmanAvailable: boolean;
  pacmanCacheSize: number;
  zypperAvailable: boolean;
  zypperCacheSize: number;
}

export interface OrphanedPackage {
  name: string;
  version: string;
  description: string;
}

export interface CleanResult {
  command: string;
  spaceFreed: number;
  message: string;
}

export interface DeepCleanResponse {
  totalSpaceFreed: number;
  results: CleanResult[];
}

@Injectable({
  providedIn: 'root',
})
export class PackageDeepCleanService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  readonly summary = signal<PackageManagerSummary | null>(null);
  readonly orphanedPackages = signal<OrphanedPackage[]>([]);
  readonly partialDownloads = signal<string[]>([]);
  readonly loading = signal(false);

  constructor() {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'init',
      'PackageDeepCleanService initialized'
    );
  }

  async getPackageSummary(): Promise<PackageManagerSummary> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'getPackageSummary',
      'Getting package summary'
    );
    this.loading.set(true);
    try {
      const summary = await this.api.invoke<PackageManagerSummary>('get_package_summary');
      this.summary.set(summary);
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'getPackageSummary',
        'Package summary retrieved'
      );
      return summary;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'getPackageSummary',
        'Operation failed',
        error as Error
      );
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async deepCleanAll(): Promise<DeepCleanResponse> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'deepCleanAll',
      'Running deep clean all'
    );
    this.loading.set(true);
    try {
      const response = await this.api.invoke<DeepCleanResponse>('deep_clean_all');
      await this.getPackageSummary();
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'deepCleanAll',
        'Deep clean completed',
        { totalSpaceFreed: response.totalSpaceFreed }
      );
      return response;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'deepCleanAll',
        'Operation failed',
        error as Error
      );
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async getAptCacheSize(): Promise<number> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'getAptCacheSize',
      'Getting apt cache size'
    );
    try {
      const result = await this.api.invoke<number>('get_apt_cache_size');
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'getAptCacheSize',
        'APT cache size retrieved',
        { size: result }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'getAptCacheSize',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async aptClean(): Promise<{ spaceFreed: number; message: string }> {
    this.logger.logInfo('service', 'PackageDeepCleanService', 'aptClean', 'Running apt clean');
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>('apt_clean');
      await this.getPackageSummary();
      this.logger.logInfo('service', 'PackageDeepCleanService', 'aptClean', 'APT cleaned', {
        spaceFreed: response.spaceFreed,
      });
      return response;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'aptClean',
        'Operation failed',
        error as Error
      );
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async aptAutoremove(): Promise<string> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'aptAutoremove',
      'Running apt autoremove'
    );
    this.loading.set(true);
    try {
      const response = await this.api.invoke<string>('apt_autoremove');
      await this.getPackageSummary();
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'aptAutoremove',
        'APT autoremove completed'
      );
      return response;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'aptAutoremove',
        'Operation failed',
        error as Error
      );
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async aptAutoclean(): Promise<{ spaceFreed: number; message: string }> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'aptAutoclean',
      'Running apt autoclean'
    );
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'apt_autoclean'
      );
      await this.getPackageSummary();
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'aptAutoclean',
        'APT autoclean completed',
        { spaceFreed: response.spaceFreed }
      );
      return response;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'aptAutoclean',
        'Operation failed',
        error as Error
      );
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async getOrphanedPackages(): Promise<OrphanedPackage[]> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'getOrphanedPackages',
      'Getting orphaned packages'
    );
    try {
      const packages = await this.api.invoke<OrphanedPackage[]>('get_orphaned_packages');
      this.orphanedPackages.set(packages);
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'getOrphanedPackages',
        'Orphaned packages retrieved',
        { count: packages.length }
      );
      return packages;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'getOrphanedPackages',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async removeOrphanedPackage(name: string): Promise<string> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'removeOrphanedPackage',
      'Removing orphaned package',
      { name }
    );
    try {
      const response = await this.api.invoke<string>('deep_clean_remove_orphaned_package', {
        name,
      });
      await this.getOrphanedPackages();
      await this.getPackageSummary();
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'removeOrphanedPackage',
        'Orphaned package removed'
      );
      return response;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'removeOrphanedPackage',
        'Operation failed',
        error as Error,
        { name }
      );
      throw error;
    }
  }

  async getPartialDownloads(): Promise<string[]> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'getPartialDownloads',
      'Getting partial downloads'
    );
    try {
      const downloads = await this.api.invoke<string[]>('get_partial_downloads');
      this.partialDownloads.set(downloads);
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'getPartialDownloads',
        'Partial downloads retrieved',
        { count: downloads.length }
      );
      return downloads;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'getPartialDownloads',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getDnfCacheSize(): Promise<number> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'getDnfCacheSize',
      'Getting dnf cache size'
    );
    try {
      const result = await this.api.invoke<number>('get_dnf_cache_size');
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'getDnfCacheSize',
        'DNF cache size retrieved',
        { size: result }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'getDnfCacheSize',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async dnfCleanAll(): Promise<{ spaceFreed: number; message: string }> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'dnfCleanAll',
      'Running dnf clean all'
    );
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'dnf_clean_all'
      );
      await this.getPackageSummary();
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'dnfCleanAll',
        'DNF clean completed',
        { spaceFreed: response.spaceFreed }
      );
      return response;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'dnfCleanAll',
        'Operation failed',
        error as Error
      );
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async getPacmanCacheSize(): Promise<number> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'getPacmanCacheSize',
      'Getting pacman cache size'
    );
    try {
      const result = await this.api.invoke<number>('get_pacman_cache_size');
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'getPacmanCacheSize',
        'Pacman cache size retrieved',
        { size: result }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'getPacmanCacheSize',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async pacmanClean(keepRecent: number): Promise<{ spaceFreed: number; message: string }> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'pacmanClean',
      'Running pacman clean',
      { keepRecent }
    );
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'pacman_clean',
        { keepRecent }
      );
      await this.getPackageSummary();
      this.logger.logInfo('service', 'PackageDeepCleanService', 'pacmanClean', 'Pacman cleaned', {
        spaceFreed: response.spaceFreed,
      });
      return response;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'pacmanClean',
        'Operation failed',
        error as Error,
        { keepRecent }
      );
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async pacmanFullClean(): Promise<{ spaceFreed: number; message: string }> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'pacmanFullClean',
      'Running pacman full clean'
    );
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'pacman_full_clean'
      );
      await this.getPackageSummary();
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'pacmanFullClean',
        'Pacman full clean completed',
        { spaceFreed: response.spaceFreed }
      );
      return response;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'pacmanFullClean',
        'Operation failed',
        error as Error
      );
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async getZypperCacheSize(): Promise<number> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'getZypperCacheSize',
      'Getting zypper cache size'
    );
    try {
      const result = await this.api.invoke<number>('get_zypper_cache_size');
      this.logger.logInfo(
        'service',
        'PackageDeepCleanService',
        'getZypperCacheSize',
        'Zypper cache size retrieved',
        { size: result }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'getZypperCacheSize',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async zypperClean(): Promise<{ spaceFreed: number; message: string }> {
    this.logger.logInfo(
      'service',
      'PackageDeepCleanService',
      'zypperClean',
      'Running zypper clean'
    );
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'zypper_clean'
      );
      await this.getPackageSummary();
      this.logger.logInfo('service', 'PackageDeepCleanService', 'zypperClean', 'Zypper cleaned', {
        spaceFreed: response.spaceFreed,
      });
      return response;
    } catch (error) {
      this.logger.logError(
        'service',
        'PackageDeepCleanService',
        'zypperClean',
        'Operation failed',
        error as Error
      );
      throw error;
    } finally {
      this.loading.set(false);
    }
  }
}
