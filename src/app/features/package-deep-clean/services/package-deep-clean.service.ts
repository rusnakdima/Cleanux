import { Injectable, inject, signal } from '@angular/core';
import { ApiService } from '@services/api.service';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';

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
  private loggingService = getLoggingService();

  readonly summary = signal<PackageManagerSummary | null>(null);
  readonly orphanedPackages = signal<OrphanedPackage[]>([]);
  readonly partialDownloads = signal<string[]>([]);
  readonly loading = signal(false);

  constructor() {
    this.loggingService.info('PackageDeepCleanService initialized');
  }

  async getPackageSummary(): Promise<PackageManagerSummary> {
    this.loggingService.info('Getting package summary');
    this.loading.set(true);
    try {
      const summary = await this.api.invoke<PackageManagerSummary>('get_package_summary');
      this.summary.set(summary);
      this.loggingService.info('Package summary retrieved');
      return summary;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async deepCleanAll(): Promise<DeepCleanResponse> {
    this.loggingService.info('Running deep clean all');
    this.loading.set(true);
    try {
      const response = await this.api.invoke<DeepCleanResponse>('deep_clean_all');
      await this.getPackageSummary();
      this.loggingService.info('Deep clean completed', { totalSpaceFreed: response.totalSpaceFreed });
      return response;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async getAptCacheSize(): Promise<number> {
    this.loggingService.info('Getting apt cache size');
    try {
      const result = await this.api.invoke<number>('get_apt_cache_size');
      this.loggingService.info('APT cache size retrieved', { size: result });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async aptClean(): Promise<{ spaceFreed: number; message: string }> {
    this.loggingService.info('Running apt clean');
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>('apt_clean');
      await this.getPackageSummary();
      this.loggingService.info('APT cleaned', { spaceFreed: response.spaceFreed });
      return response;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async aptAutoremove(): Promise<string> {
    this.loggingService.info('Running apt autoremove');
    this.loading.set(true);
    try {
      const response = await this.api.invoke<string>('apt_autoremove');
      await this.getPackageSummary();
      this.loggingService.info('APT autoremove completed');
      return response;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async aptAutoclean(): Promise<{ spaceFreed: number; message: string }> {
    this.loggingService.info('Running apt autoclean');
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'apt_autoclean'
      );
      await this.getPackageSummary();
      this.loggingService.info('APT autoclean completed', { spaceFreed: response.spaceFreed });
      return response;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async getOrphanedPackages(): Promise<OrphanedPackage[]> {
    this.loggingService.info('Getting orphaned packages');
    try {
      const packages = await this.api.invoke<OrphanedPackage[]>('get_orphaned_packages');
      this.orphanedPackages.set(packages);
      this.loggingService.info('Orphaned packages retrieved', { count: packages.length });
      return packages;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async removeOrphanedPackage(name: string): Promise<string> {
    this.loggingService.info('Removing orphaned package', { name });
    try {
      const response = await this.api.invoke<string>('deep_clean_remove_orphaned_package', {
        name,
      });
      await this.getOrphanedPackages();
      await this.getPackageSummary();
      this.loggingService.info('Orphaned package removed');
      return response;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { name });
      throw error;
    }
  }

  async getPartialDownloads(): Promise<string[]> {
    this.loggingService.info('Getting partial downloads');
    try {
      const downloads = await this.api.invoke<string[]>('get_partial_downloads');
      this.partialDownloads.set(downloads);
      this.loggingService.info('Partial downloads retrieved', { count: downloads.length });
      return downloads;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getDnfCacheSize(): Promise<number> {
    this.loggingService.info('Getting dnf cache size');
    try {
      const result = await this.api.invoke<number>('get_dnf_cache_size');
      this.loggingService.info('DNF cache size retrieved', { size: result });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async dnfCleanAll(): Promise<{ spaceFreed: number; message: string }> {
    this.loggingService.info('Running dnf clean all');
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'dnf_clean_all'
      );
      await this.getPackageSummary();
      this.loggingService.info('DNF clean completed', { spaceFreed: response.spaceFreed });
      return response;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async getPacmanCacheSize(): Promise<number> {
    this.loggingService.info('Getting pacman cache size');
    try {
      const result = await this.api.invoke<number>('get_pacman_cache_size');
      this.loggingService.info('Pacman cache size retrieved', { size: result });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async pacmanClean(keepRecent: number): Promise<{ spaceFreed: number; message: string }> {
    this.loggingService.info('Running pacman clean', { keepRecent });
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'pacman_clean',
        {
          keepRecent,
        }
      );
      await this.getPackageSummary();
      this.loggingService.info('Pacman cleaned', { spaceFreed: response.spaceFreed });
      return response;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { keepRecent });
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async pacmanFullClean(): Promise<{ spaceFreed: number; message: string }> {
    this.loggingService.info('Running pacman full clean');
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'pacman_full_clean'
      );
      await this.getPackageSummary();
      this.loggingService.info('Pacman full clean completed', { spaceFreed: response.spaceFreed });
      return response;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async getZypperCacheSize(): Promise<number> {
    this.loggingService.info('Getting zypper cache size');
    try {
      const result = await this.api.invoke<number>('get_zypper_cache_size');
      this.loggingService.info('Zypper cache size retrieved', { size: result });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async zypperClean(): Promise<{ spaceFreed: number; message: string }> {
    this.loggingService.info('Running zypper clean');
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'zypper_clean'
      );
      await this.getPackageSummary();
      this.loggingService.info('Zypper cleaned', { spaceFreed: response.spaceFreed });
      return response;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    } finally {
      this.loading.set(false);
    }
  }
}
