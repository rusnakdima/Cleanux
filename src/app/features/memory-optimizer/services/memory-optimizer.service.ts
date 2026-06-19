/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';

/* models */
import { MemoryInfo, SwapInfo, ProcessMemory } from '@entities/memory.model';

@Injectable({
  providedIn: 'root',
})
export class MemoryOptimizerService {
  private api = inject(ApiService);

  constructor() {}

  async getMemoryInfo(): Promise<MemoryInfo> {
    try {
      const result = await this.api.invoke<MemoryInfo>('get_memory_info');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getSwapInfo(): Promise<SwapInfo> {
    try {
      const result = await this.api.invoke<SwapInfo>('get_swap_info');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getProcessMemory(): Promise<ProcessMemory[]> {
    try {
      const result = await this.api.invoke<ProcessMemory[]>('get_process_memory');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async optimizeMemory(): Promise<boolean> {
    try {
      const result = await this.api.invoke<boolean>('optimize_memory');
      return result;
    } catch (error) {
      throw error;
    }
  }
}
