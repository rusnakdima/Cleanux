/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';

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
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo(
      'service',
      'MemoryOptimizerService',
      'init',
      'MemoryOptimizerService initialized'
    );
  }

  async getMemoryInfo(): Promise<MemoryInfo> {
    this.logger.logInfo(
      'service',
      'MemoryOptimizerService',
      'getMemoryInfo',
      'Getting memory info'
    );
    try {
      const result = await this.api.invoke<MemoryInfo>('get_memory_info');
      this.logger.logInfo(
        'service',
        'MemoryOptimizerService',
        'getMemoryInfo',
        'Memory info retrieved'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'MemoryOptimizerService',
        'getMemoryInfo',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getSwapInfo(): Promise<SwapInfo> {
    this.logger.logInfo('service', 'MemoryOptimizerService', 'getSwapInfo', 'Getting swap info');
    try {
      const result = await this.api.invoke<SwapInfo>('get_swap_info');
      this.logger.logInfo(
        'service',
        'MemoryOptimizerService',
        'getSwapInfo',
        'Swap info retrieved'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'MemoryOptimizerService',
        'getSwapInfo',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getProcessMemory(): Promise<ProcessMemory[]> {
    this.logger.logInfo(
      'service',
      'MemoryOptimizerService',
      'getProcessMemory',
      'Getting process memory'
    );
    try {
      const result = await this.api.invoke<ProcessMemory[]>('get_process_memory');
      this.logger.logInfo(
        'service',
        'MemoryOptimizerService',
        'getProcessMemory',
        'Process memory retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'MemoryOptimizerService',
        'getProcessMemory',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async optimizeMemory(): Promise<boolean> {
    this.logger.logInfo('service', 'MemoryOptimizerService', 'optimizeMemory', 'Optimizing memory');
    try {
      const result = await this.api.invoke<boolean>('optimize_memory');
      this.logger.logInfo(
        'service',
        'MemoryOptimizerService',
        'optimizeMemory',
        'Memory optimized'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'MemoryOptimizerService',
        'optimizeMemory',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }
}
