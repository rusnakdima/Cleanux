/* angular */
import { Injectable, inject } from '@angular/core';

/* library */
import { InvokeWrapperService } from '@tauri-front/shared';

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

export interface ProcessMemoryItem {
  pid: number;
  name: string;
  memory_mb: number;
  cpu_percent: number;
}

@Injectable({ providedIn: 'root' })
export class MemoryOptimizerService {
  private api = inject(InvokeWrapperService);

  async getMemoryInfo(): Promise<MemoryInfo> {
    return this.api.invoke<MemoryInfo>('get_memory_info');
  }

  async getSwapInfo(): Promise<SwapInfo> {
    return this.api.invoke<SwapInfo>('get_swap_info');
  }

  async getProcessMemory(): Promise<ProcessMemoryItem[]> {
    return this.api.invoke<ProcessMemoryItem[]>('get_process_memory');
  }

  async optimizeMemory(): Promise<boolean> {
    return this.api.invoke<boolean>('optimize_memory');
  }
}
