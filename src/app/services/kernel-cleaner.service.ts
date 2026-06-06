import { Injectable } from '@angular/core';
import { BaseApiService } from '@services/base-api.service';
import { OperationResult } from '@models/cleaner.models';
import { KernelInfo, InitramfsInfo, BootSpaceInfo } from '@models/kernel.model';

@Injectable({
  providedIn: 'root',
})
export class KernelCleanerService extends BaseApiService {
  async getCurrentKernel(): Promise<string> {
    return this.call<string>('get_current_kernel');
  }

  async getInstalledKernels(): Promise<KernelInfo[]> {
    return this.call<KernelInfo[]>('get_installed_kernels');
  }

  async getOldKernels(): Promise<KernelInfo[]> {
    return this.call<KernelInfo[]>('get_old_kernels');
  }

  async getOldKernelsSize(): Promise<number> {
    return this.call<number>('get_old_kernels_size');
  }

  async removeKernel(version: string): Promise<OperationResult<string[]>> {
    return this.call<OperationResult<string[]>>('remove_kernel', { version });
  }

  async getOldInitramfs(): Promise<InitramfsInfo[]> {
    return this.call<InitramfsInfo[]>('get_old_initramfs');
  }

  async removeInitramfs(version: string): Promise<OperationResult<string[]>> {
    return this.call<OperationResult<string[]>>('remove_initramfs', { version });
  }

  async getBootSpaceInfo(): Promise<BootSpaceInfo> {
    return this.call<BootSpaceInfo>('get_boot_space_info');
  }

  async updateGrub(): Promise<boolean> {
    return this.call<boolean>('update_grub');
  }
}
