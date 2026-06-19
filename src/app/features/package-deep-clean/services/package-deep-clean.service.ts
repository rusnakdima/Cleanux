import { Injectable, inject, signal } from '@angular/core';
import { ApiService } from '@services/api.service';

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

  readonly summary = signal<PackageManagerSummary | null>(null);
  readonly orphanedPackages = signal<OrphanedPackage[]>([]);
  readonly partialDownloads = signal<string[]>([]);
  readonly loading = signal(false);

  constructor() {}

  async getPackageSummary(): Promise<PackageManagerSummary> {
    this.loading.set(true);
    try {
      const summary = await this.api.invoke<PackageManagerSummary>('get_package_summary');
      this.summary.set(summary);
      return summary;
    } catch (error) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async deepCleanAll(): Promise<DeepCleanResponse> {
    this.loading.set(true);
    try {
      const response = await this.api.invoke<DeepCleanResponse>('deep_clean_all');
      await this.getPackageSummary();
      return response;
    } catch (error) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async getAptCacheSize(): Promise<number> {
    try {
      const result = await this.api.invoke<number>('get_apt_cache_size');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async aptClean(): Promise<{ spaceFreed: number; message: string }> {
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>('apt_clean');
      await this.getPackageSummary();
      return response;
    } catch (error) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async aptAutoremove(): Promise<string> {
    this.loading.set(true);
    try {
      const response = await this.api.invoke<string>('apt_autoremove');
      await this.getPackageSummary();
      return response;
    } catch (error) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async aptAutoclean(): Promise<{ spaceFreed: number; message: string }> {
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'apt_autoclean'
      );
      await this.getPackageSummary();
      return response;
    } catch (error) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async getOrphanedPackages(): Promise<OrphanedPackage[]> {
    try {
      const packages = await this.api.invoke<OrphanedPackage[]>('get_orphaned_packages');
      this.orphanedPackages.set(packages);
      return packages;
    } catch (error) {
      throw error;
    }
  }

  async removeOrphanedPackage(name: string): Promise<string> {
    try {
      const response = await this.api.invoke<string>('deep_clean_remove_orphaned_package', {
        name,
      });
      await this.getOrphanedPackages();
      await this.getPackageSummary();
      return response;
    } catch (error) {
      throw error;
    }
  }

  async getPartialDownloads(): Promise<string[]> {
    try {
      const downloads = await this.api.invoke<string[]>('get_partial_downloads');
      this.partialDownloads.set(downloads);
      return downloads;
    } catch (error) {
      throw error;
    }
  }

  async getDnfCacheSize(): Promise<number> {
    try {
      const result = await this.api.invoke<number>('get_dnf_cache_size');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async dnfCleanAll(): Promise<{ spaceFreed: number; message: string }> {
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'dnf_clean_all'
      );
      await this.getPackageSummary();
      return response;
    } catch (error) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async getPacmanCacheSize(): Promise<number> {
    try {
      const result = await this.api.invoke<number>('get_pacman_cache_size');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async pacmanClean(keepRecent: number): Promise<{ spaceFreed: number; message: string }> {
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'pacman_clean',
        {
          keepRecent,
        }
      );
      await this.getPackageSummary();
      return response;
    } catch (error) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async pacmanFullClean(): Promise<{ spaceFreed: number; message: string }> {
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'pacman_full_clean'
      );
      await this.getPackageSummary();
      return response;
    } catch (error) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async getZypperCacheSize(): Promise<number> {
    try {
      const result = await this.api.invoke<number>('get_zypper_cache_size');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async zypperClean(): Promise<{ spaceFreed: number; message: string }> {
    this.loading.set(true);
    try {
      const response = await this.api.invoke<{ spaceFreed: number; message: string }>(
        'zypper_clean'
      );
      await this.getPackageSummary();
      return response;
    } catch (error) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }
}
