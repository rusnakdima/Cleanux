import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';

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

  async getCurrentKernel(): Promise<string> {
    return await this.api.invoke<string>('get_current_kernel');
  }

  async getInstalledKernels(): Promise<KernelInfo[]> {
    return await this.api.invoke<KernelInfo[]>('get_installed_kernels');
  }

  async getOldKernels(): Promise<KernelInfo[]> {
    return await this.api.invoke<KernelInfo[]>('get_old_kernels');
  }

  async getOldKernelsSize(): Promise<number> {
    return await this.api.invoke<number>('get_old_kernels_size');
  }

  async removeKernel(version: string): Promise<RemoveResult> {
    return await this.api.invoke<RemoveResult>('remove_kernel', { version });
  }

  async getOldInitramfs(): Promise<InitramfsInfo[]> {
    return await this.api.invoke<InitramfsInfo[]>('get_old_initramfs');
  }

  async removeInitramfs(version: string): Promise<RemoveResult> {
    return await this.api.invoke<RemoveResult>('remove_initramfs', { version });
  }

  async getBootSpaceInfo(): Promise<BootSpaceInfo> {
    return await this.api.invoke<BootSpaceInfo>('get_boot_space_info');
  }

  async updateGrub(): Promise<boolean> {
    return await this.api.invoke<boolean>('update_grub');
  }
}
