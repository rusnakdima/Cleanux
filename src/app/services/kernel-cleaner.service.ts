import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';

export interface KernelInfo {
  version: string;
  path: string;
  size: number;
  is_current: boolean;
}

export interface InitramfsInfo {
  version: string;
  path: string;
  size: number;
}

export interface BootSpaceInfo {
  total: number;
  used: number;
  available: number;
  usage_percent: number;
}

export interface RemoveResult {
  removed: string[];
  failed: string[];
}

@Injectable({
  providedIn: 'root',
})
export class KernelCleanerService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo(
      'service',
      'KernelCleanerService',
      'init',
      'KernelCleanerService initialized'
    );
  }

  async getCurrentKernel(): Promise<string> {
    this.logger.logInfo(
      'service',
      'KernelCleanerService',
      'getCurrentKernel',
      'Getting current kernel'
    );
    try {
      const result = await this.api.invoke<string>('get_current_kernel');
      this.logger.logInfo(
        'service',
        'KernelCleanerService',
        'getCurrentKernel',
        'Current kernel retrieved',
        { version: result }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'KernelCleanerService',
        'getCurrentKernel',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getInstalledKernels(): Promise<KernelInfo[]> {
    this.logger.logInfo(
      'service',
      'KernelCleanerService',
      'getInstalledKernels',
      'Getting installed kernels'
    );
    try {
      const result = await this.api.invoke<KernelInfo[]>('get_installed_kernels');
      this.logger.logInfo(
        'service',
        'KernelCleanerService',
        'getInstalledKernels',
        'Installed kernels retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'KernelCleanerService',
        'getInstalledKernels',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getOldKernels(): Promise<KernelInfo[]> {
    this.logger.logInfo('service', 'KernelCleanerService', 'getOldKernels', 'Getting old kernels');
    try {
      const result = await this.api.invoke<KernelInfo[]>('get_old_kernels');
      this.logger.logInfo(
        'service',
        'KernelCleanerService',
        'getOldKernels',
        'Old kernels retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'KernelCleanerService',
        'getOldKernels',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getOldKernelsSize(): Promise<number> {
    this.logger.logInfo(
      'service',
      'KernelCleanerService',
      'getOldKernelsSize',
      'Getting old kernels size'
    );
    try {
      const result = await this.api.invoke<number>('get_old_kernels_size');
      this.logger.logInfo(
        'service',
        'KernelCleanerService',
        'getOldKernelsSize',
        'Old kernels size retrieved',
        { size: result }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'KernelCleanerService',
        'getOldKernelsSize',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async removeKernel(version: string): Promise<RemoveResult> {
    this.logger.logInfo('service', 'KernelCleanerService', 'removeKernel', 'Removing kernel', {
      version,
    });
    try {
      const result = await this.api.invoke<RemoveResult>('remove_kernel', { version });
      this.logger.logInfo('service', 'KernelCleanerService', 'removeKernel', 'Kernel removed', {
        removed: result.removed.length,
        failed: result.failed.length,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'KernelCleanerService',
        'removeKernel',
        'Operation failed',
        error as Error,
        { version }
      );
      throw error;
    }
  }

  async getOldInitramfs(): Promise<InitramfsInfo[]> {
    this.logger.logInfo(
      'service',
      'KernelCleanerService',
      'getOldInitramfs',
      'Getting old initramfs'
    );
    try {
      const result = await this.api.invoke<InitramfsInfo[]>('get_old_initramfs');
      this.logger.logInfo(
        'service',
        'KernelCleanerService',
        'getOldInitramfs',
        'Old initramfs retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'KernelCleanerService',
        'getOldInitramfs',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async removeInitramfs(version: string): Promise<RemoveResult> {
    this.logger.logInfo(
      'service',
      'KernelCleanerService',
      'removeInitramfs',
      'Removing initramfs',
      { version }
    );
    try {
      const result = await this.api.invoke<RemoveResult>('remove_initramfs', { version });
      this.logger.logInfo(
        'service',
        'KernelCleanerService',
        'removeInitramfs',
        'Initramfs removed',
        { removed: result.removed.length, failed: result.failed.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'KernelCleanerService',
        'removeInitramfs',
        'Operation failed',
        error as Error,
        { version }
      );
      throw error;
    }
  }

  async getBootSpaceInfo(): Promise<BootSpaceInfo> {
    this.logger.logInfo(
      'service',
      'KernelCleanerService',
      'getBootSpaceInfo',
      'Getting boot space info'
    );
    try {
      const result = await this.api.invoke<BootSpaceInfo>('get_boot_space_info');
      this.logger.logInfo(
        'service',
        'KernelCleanerService',
        'getBootSpaceInfo',
        'Boot space info retrieved'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'KernelCleanerService',
        'getBootSpaceInfo',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async updateGrub(): Promise<boolean> {
    this.logger.logInfo('service', 'KernelCleanerService', 'updateGrub', 'Updating grub');
    try {
      const result = await this.api.invoke<boolean>('update_grub');
      this.logger.logInfo('service', 'KernelCleanerService', 'updateGrub', 'Grub updated');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'KernelCleanerService',
        'updateGrub',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }
}
