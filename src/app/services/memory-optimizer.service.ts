/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';

/* models */
export interface MemoryInfo {
  total: number;
  used: number;
  available: number;
  cached: number;
  buffers: number;
}

export interface SwapInfo {
  total: number;
  used: number;
}

export interface ProcessMemory {
  pid: number;
  name: string;
  memory_mb: number;
  cpu_percent: number;
}

@Injectable({
  providedIn: 'root',
})
export class MemoryOptimizerService {
  private api = inject(ApiService);

  async getMemoryInfo(): Promise<MemoryInfo> {
    return await this.api.invoke<MemoryInfo>('get_memory_info');
  }

  async getSwapInfo(): Promise<SwapInfo> {
    return await this.api.invoke<SwapInfo>('get_swap_info');
  }

  async getProcessMemory(): Promise<ProcessMemory[]> {
    return await this.api.invoke<ProcessMemory[]>('get_process_memory');
  }

  async optimizeMemory(): Promise<boolean> {
    return await this.api.invoke<boolean>('optimize_memory');
  }
}
