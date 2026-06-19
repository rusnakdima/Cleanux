import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';

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

  constructor() {}

  async getCurrentKernel(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('get_current_kernel');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getInstalledKernels(): Promise<KernelInfo[]> {
    try {
      const result = await this.api.invoke<KernelInfo[]>('get_installed_kernels');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getOldKernels(): Promise<KernelInfo[]> {
    try {
      const result = await this.api.invoke<KernelInfo[]>('get_old_kernels');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getOldKernelsSize(): Promise<number> {
    try {
      const result = await this.api.invoke<number>('get_old_kernels_size');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async removeKernel(version: string): Promise<RemoveResult> {
    try {
      const result = await this.api.invoke<RemoveResult>('remove_kernel', { version });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getOldInitramfs(): Promise<InitramfsInfo[]> {
    try {
      const result = await this.api.invoke<InitramfsInfo[]>('get_old_initramfs');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async removeInitramfs(version: string): Promise<RemoveResult> {
    try {
      const result = await this.api.invoke<RemoveResult>('remove_initramfs', { version });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getBootSpaceInfo(): Promise<BootSpaceInfo> {
    try {
      const result = await this.api.invoke<BootSpaceInfo>('get_boot_space_info');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async updateGrub(): Promise<boolean> {
    try {
      const result = await this.api.invoke<boolean>('update_grub');
      return result;
    } catch (error) {
      throw error;
    }
  }
}
