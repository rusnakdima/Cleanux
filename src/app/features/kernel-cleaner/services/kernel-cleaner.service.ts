import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
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
  private loggingService = new LoggerService();

  constructor() {
    this.loggingService.info('KernelCleanerService initialized');
  }

  async getCurrentKernel(): Promise<string> {
    this.loggingService.info('Getting current kernel');
    try {
      const result = await this.api.invoke<string>('get_current_kernel');
      this.loggingService.info('Current kernel retrieved', { version: result });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getInstalledKernels(): Promise<KernelInfo[]> {
    this.loggingService.info('Getting installed kernels');
    try {
      const result = await this.api.invoke<KernelInfo[]>('get_installed_kernels');
      this.loggingService.info('Installed kernels retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getOldKernels(): Promise<KernelInfo[]> {
    this.loggingService.info('Getting old kernels');
    try {
      const result = await this.api.invoke<KernelInfo[]>('get_old_kernels');
      this.loggingService.info('Old kernels retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getOldKernelsSize(): Promise<number> {
    this.loggingService.info('Getting old kernels size');
    try {
      const result = await this.api.invoke<number>('get_old_kernels_size');
      this.loggingService.info('Old kernels size retrieved', { size: result });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async removeKernel(version: string): Promise<RemoveResult> {
    this.loggingService.info('Removing kernel', { version });
    try {
      const result = await this.api.invoke<RemoveResult>('remove_kernel', { version });
      this.loggingService.info('Kernel removed', {
        removed: result.removed.length,
        failed: result.failed.length,
      });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { version });
      throw error;
    }
  }

  async getOldInitramfs(): Promise<InitramfsInfo[]> {
    this.loggingService.info('Getting old initramfs');
    try {
      const result = await this.api.invoke<InitramfsInfo[]>('get_old_initramfs');
      this.loggingService.info('Old initramfs retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async removeInitramfs(version: string): Promise<RemoveResult> {
    this.loggingService.info('Removing initramfs', { version });
    try {
      const result = await this.api.invoke<RemoveResult>('remove_initramfs', { version });
      this.loggingService.info('Initramfs removed', {
        removed: result.removed.length,
        failed: result.failed.length,
      });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { version });
      throw error;
    }
  }

  async getBootSpaceInfo(): Promise<BootSpaceInfo> {
    this.loggingService.info('Getting boot space info');
    try {
      const result = await this.api.invoke<BootSpaceInfo>('get_boot_space_info');
      this.loggingService.info('Boot space info retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async updateGrub(): Promise<boolean> {
    this.loggingService.info('Updating grub');
    try {
      const result = await this.api.invoke<boolean>('update_grub');
      this.loggingService.info('Grub updated');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }
}
