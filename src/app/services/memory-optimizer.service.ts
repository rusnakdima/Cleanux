/* sys lib */
import { Injectable } from '@angular/core';

/* services */
import { BaseApiService } from '@services/base-api.service';

/* models */
import { MemoryInfo, SwapInfo, ProcessMemory } from '@models/memory.model';

@Injectable({
  providedIn: 'root',
})
export class MemoryOptimizerService extends BaseApiService {
  async getMemoryInfo(): Promise<MemoryInfo> {
    return this.call<MemoryInfo>('get_memory_info');
  }

  async getSwapInfo(): Promise<SwapInfo> {
    return this.call<SwapInfo>('get_swap_info');
  }

  async getProcessMemory(): Promise<ProcessMemory[]> {
    return this.call<ProcessMemory[]>('get_process_memory');
  }

  async optimizeMemory(): Promise<boolean> {
    return this.call<boolean>('optimize_memory');
  }
}
