import { Injectable, signal } from '@angular/core';
import { BaseApiService } from '@services/base-api.service';
import { OperationResult } from '@models/cleaner.models';
import { PackageManagerSummary, OrphanedPackage, DeepCleanResponse } from '@models/package.model';

@Injectable({
  providedIn: 'root',
})
export class PackageDeepCleanService extends BaseApiService {
  readonly summary = signal<PackageManagerSummary | null>(null);
  readonly orphanedPackages = signal<OrphanedPackage[]>([]);
  readonly partialDownloads = signal<string[]>([]);
  readonly loading = signal(false);

  async getPackageSummary(): Promise<PackageManagerSummary> {
    this.loading.set(true);
    try {
      const summary = await this.call<PackageManagerSummary>('get_package_summary');
      this.summary.set(summary);
      return summary;
    } finally {
      this.loading.set(false);
    }
  }

  async deepCleanAll(): Promise<DeepCleanResponse> {
    this.loading.set(true);
    try {
      const response = await this.call<DeepCleanResponse>('deep_clean_all');
      await this.getPackageSummary();
      return response;
    } finally {
      this.loading.set(false);
    }
  }

  async getAptCacheSize(): Promise<number> {
    return this.call<number>('get_apt_cache_size');
  }

  async aptClean(): Promise<{ spaceFreed: number; message: string }> {
    this.loading.set(true);
    try {
      const response = await this.call<{ spaceFreed: number; message: string }>('apt_clean');
      await this.getPackageSummary();
      return response;
    } finally {
      this.loading.set(false);
    }
  }

  async aptAutoremove(): Promise<string> {
    this.loading.set(true);
    try {
      const response = await this.call<string>('apt_autoremove');
      await this.getPackageSummary();
      return response;
    } finally {
      this.loading.set(false);
    }
  }

  async aptAutoclean(): Promise<{ spaceFreed: number; message: string }> {
    this.loading.set(true);
    try {
      const response = await this.call<{ spaceFreed: number; message: string }>('apt_autoclean');
      await this.getPackageSummary();
      return response;
    } finally {
      this.loading.set(false);
    }
  }

  async getOrphanedPackages(): Promise<OrphanedPackage[]> {
    const packages = await this.call<OrphanedPackage[]>('get_orphaned_packages');
    this.orphanedPackages.set(packages);
    return packages;
  }

  async removeOrphanedPackage(name: string): Promise<string> {
    const response = await this.call<string>('deep_clean_remove_orphaned_package', { name });
    await this.getOrphanedPackages();
    await this.getPackageSummary();
    return response;
  }

  async getPartialDownloads(): Promise<string[]> {
    const downloads = await this.call<string[]>('get_partial_downloads');
    this.partialDownloads.set(downloads);
    return downloads;
  }

  async getDnfCacheSize(): Promise<number> {
    return this.call<number>('get_dnf_cache_size');
  }

  async dnfCleanAll(): Promise<{ spaceFreed: number; message: string }> {
    this.loading.set(true);
    try {
      const response = await this.call<{ spaceFreed: number; message: string }>('dnf_clean_all');
      await this.getPackageSummary();
      return response;
    } finally {
      this.loading.set(false);
    }
  }

  async getPacmanCacheSize(): Promise<number> {
    return this.call<number>('get_pacman_cache_size');
  }

  async pacmanClean(keepRecent: number): Promise<{ spaceFreed: number; message: string }> {
    this.loading.set(true);
    try {
      const response = await this.call<{ spaceFreed: number; message: string }>('pacman_clean', {
        keepRecent,
      });
      await this.getPackageSummary();
      return response;
    } finally {
      this.loading.set(false);
    }
  }

  async pacmanFullClean(): Promise<{ spaceFreed: number; message: string }> {
    this.loading.set(true);
    try {
      const response = await this.call<{ spaceFreed: number; message: string }>(
        'pacman_full_clean'
      );
      await this.getPackageSummary();
      return response;
    } finally {
      this.loading.set(false);
    }
  }

  async getZypperCacheSize(): Promise<number> {
    return this.call<number>('get_zypper_cache_size');
  }

  async zypperClean(): Promise<{ spaceFreed: number; message: string }> {
    this.loading.set(true);
    try {
      const response = await this.call<{ spaceFreed: number; message: string }>('zypper_clean');
      await this.getPackageSummary();
      return response;
    } finally {
      this.loading.set(false);
    }
  }
}
