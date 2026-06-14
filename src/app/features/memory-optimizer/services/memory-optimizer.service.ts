/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';

/* models */
import { MemoryInfo, SwapInfo, ProcessMemory } from '@models/memory.model';

@Injectable({
  providedIn: 'root',
})
export class MemoryOptimizerService {
  private api = inject(ApiService);
  private loggingService = getLoggingService();

  constructor() {
    this.loggingService.info('MemoryOptimizerService initialized');
  }

  async getMemoryInfo(): Promise<MemoryInfo> {
    this.loggingService.info('Getting memory info');
    try {
      const result = await this.api.invoke<MemoryInfo>('get_memory_info');
      this.loggingService.info('Memory info retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getSwapInfo(): Promise<SwapInfo> {
    this.loggingService.info('Getting swap info');
    try {
      const result = await this.api.invoke<SwapInfo>('get_swap_info');
      this.loggingService.info('Swap info retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getProcessMemory(): Promise<ProcessMemory[]> {
    this.loggingService.info('Getting process memory');
    try {
      const result = await this.api.invoke<ProcessMemory[]>('get_process_memory');
      this.loggingService.info('Process memory retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async optimizeMemory(): Promise<boolean> {
    this.loggingService.info('Optimizing memory');
    try {
      const result = await this.api.invoke<boolean>('optimize_memory');
      this.loggingService.info('Memory optimized');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }
}
