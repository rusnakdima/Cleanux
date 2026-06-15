/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';
import { LoggerService } from './logger.service';

/* models */
import {
  CacheFileItem,
  TrashFileItem,
  LogFileItem,
  LargeFileItem,
  ScanSummary,
  PaginatedData,
} from '@models/system.model';

export type {
  CacheFileItem,
  TrashFileItem,
  LogFileItem,
  LargeFileItem,
  ScanSummary,
  PaginatedData,
} from '@models/system.model';

export interface FilePreviewResult {
  name: string;
  path: string;
  type: string;
  content?: string;
  imageUrl?: string;
}

@Injectable({
  providedIn: 'root',
})
export class FileService {
  private api = inject(ApiService);
  private loggingService = new LoggerService();

  private inFlightRequests = new Map<string, Promise<unknown>>();
  private abortController: AbortController | null = null;

  constructor() {
    this.loggingService.info('FileService initialized');
  }

  private getCacheKey(cmd: string, params?: Record<string, unknown>): string {
    return `${cmd}:${JSON.stringify(params ?? {})}`;
  }

  private cancelPreviousRequest(): void {
    if (this.abortController) {
      this.abortController.abort();
      this.abortController = null;
    }
  }

  async getCacheFiles(limit?: number, offset?: number): Promise<PaginatedData<CacheFileItem>> {
    this.loggingService.info('Getting cache files', { limit, offset });
    const cacheKey = this.getCacheKey('getCacheFiles', { limit, offset });

    if (this.inFlightRequests.has(cacheKey)) {
      return this.inFlightRequests.get(cacheKey) as Promise<PaginatedData<CacheFileItem>>;
    }

    this.cancelPreviousRequest();
    this.abortController = new AbortController();
    const controller = this.abortController;

    const promise = (async () => {
      try {
        const response = await this.api.invoke<PaginatedData<CacheFileItem>>('getCacheFiles', {
          limit: limit ?? null,
          offset: offset ?? null,
          signal: controller.signal,
        });
        this.loggingService.info('Cache files retrieved', { total: response.total });
        return {
          data: response.data,
          has_more: response.has_more,
          total: response.total,
        };
      } catch (error) {
        this.loggingService.error('Operation failed', error as Error);
        throw error;
      } finally {
        this.inFlightRequests.delete(cacheKey);
      }
    })();

    this.inFlightRequests.set(cacheKey, promise);
    return promise;
  }

  async getTrashFiles(limit?: number, offset?: number): Promise<TrashFileItem[]> {
    this.loggingService.info('Getting trash files', { limit, offset });
    try {
      const result = await this.api.invoke<TrashFileItem[]>('getTrashFiles', {
        limit: limit ?? null,
        offset: offset ?? null,
      });
      this.loggingService.info('Trash files retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getSystemLogs(limit?: number, offset?: number): Promise<LogFileItem[]> {
    this.loggingService.info('Getting system logs', { limit, offset });
    try {
      const result = await this.api.invoke<LogFileItem[]>('getSystemLogs', {
        limit: limit ?? null,
        offset: offset ?? null,
      });
      this.loggingService.info('System logs retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getLargeFiles(limit?: number, offset?: number): Promise<PaginatedData<LargeFileItem>> {
    this.loggingService.info('Getting large files', { limit, offset });
    const cacheKey = this.getCacheKey('getLargeFiles', { limit, offset });

    if (this.inFlightRequests.has(cacheKey)) {
      return this.inFlightRequests.get(cacheKey) as Promise<PaginatedData<LargeFileItem>>;
    }

    this.cancelPreviousRequest();
    this.abortController = new AbortController();
    const controller = this.abortController;

    const promise = (async () => {
      try {
        const response = await this.api.invoke<PaginatedData<LargeFileItem>>('getLargeFiles', {
          limit: limit ?? null,
          offset: offset ?? null,
          signal: controller.signal,
        });
        this.loggingService.info('Large files retrieved', { total: response.total });
        return {
          data: response.data,
          has_more: response.has_more,
          total: response.total,
        };
      } catch (error) {
        this.loggingService.error('Operation failed', error as Error);
        throw error;
      } finally {
        this.inFlightRequests.delete(cacheKey);
      }
    })();

    this.inFlightRequests.set(cacheKey, promise);
    return promise;
  }

  async getCacheSummary(): Promise<ScanSummary> {
    this.loggingService.info('Getting cache summary');
    try {
      const result = await this.api.invoke<ScanSummary>('getCacheSummary');
      this.loggingService.info('Cache summary retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getTrashSummary(): Promise<ScanSummary> {
    this.loggingService.info('Getting trash summary');
    try {
      const result = await this.api.invoke<ScanSummary>('getTrashSummary');
      this.loggingService.info('Trash summary retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getLogSummary(): Promise<ScanSummary> {
    this.loggingService.info('Getting log summary');
    try {
      const result = await this.api.invoke<ScanSummary>('getLogSummary');
      this.loggingService.info('Log summary retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getLargeFilesSummary(): Promise<ScanSummary> {
    this.loggingService.info('Getting large files summary');
    try {
      const result = await this.api.invoke<ScanSummary>('getLargeFilesSummary');
      this.loggingService.info('Large files summary retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async clearSelectedCacheFiles(paths: string[]): Promise<string> {
    this.loggingService.info('Clearing selected cache files', { count: paths.length });
    try {
      const result = await this.api.invoke<string>('clearSelectedCacheFiles', { paths });
      this.loggingService.info('Selected cache files cleared');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { paths });
      throw error;
    }
  }

  async clearSelectedTrashFiles(paths: string[]): Promise<string> {
    this.loggingService.info('Clearing selected trash files', { count: paths.length });
    try {
      const result = await this.api.invoke<string>('clearSelectedTrashFiles', { paths });
      this.loggingService.info('Selected trash files cleared');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { paths });
      throw error;
    }
  }

  async clearSelectedLogFiles(paths: string[]): Promise<string> {
    this.loggingService.info('Clearing selected log files', { count: paths.length });
    try {
      const result = await this.api.invoke<string>('clearSelectedLogFiles', { paths });
      this.loggingService.info('Selected log files cleared');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { paths });
      throw error;
    }
  }

  async clearSelectedLargeFiles(paths: string[]): Promise<string> {
    this.loggingService.info('Clearing selected large files', { count: paths.length });
    try {
      const result = await this.api.invoke<string>('clearSelectedLargeFiles', { paths });
      this.loggingService.info('Selected large files cleared');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { paths });
      throw error;
    }
  }

  async clearTrash(): Promise<string> {
    this.loggingService.info('Clearing trash');
    try {
      const result = await this.api.invoke<string>('clearTrash');
      this.loggingService.info('Trash cleared');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async clearCache(): Promise<string> {
    this.loggingService.info('Clearing cache');
    try {
      const result = await this.api.invoke<string>('clearCache');
      this.loggingService.info('Cache cleared');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async clearAllLogs(): Promise<string> {
    this.loggingService.info('Clearing all logs');
    try {
      const result = await this.api.invoke<string>('clearAllLogs');
      this.loggingService.info('All logs cleared');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async clearAllLargeFiles(): Promise<string> {
    this.loggingService.info('Clearing all large files');
    try {
      const result = await this.api.invoke<string>('clearAllLargeFiles');
      this.loggingService.info('All large files cleared');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async previewFile(path: string): Promise<FilePreviewResult> {
    this.loggingService.info('Previewing file', { path });
    try {
      const result = await this.api.invoke<FilePreviewResult>('previewFile', { path });
      this.loggingService.info('File previewed', { type: result.type });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { path });
      throw error;
    }
  }

  async openFile(path: string, command?: string): Promise<void> {
    this.loggingService.info('Opening file', { path, command });
    try {
      await this.api.invoke<void>('openFile', { path, command: command || null });
      this.loggingService.info('File opened');
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { path, command });
      throw error;
    }
  }

  async deleteFiles(paths: string[]): Promise<string> {
    this.loggingService.info('Deleting files', { count: paths.length });
    try {
      const result = await this.api.invoke<string>('deleteFiles', { paths });
      this.loggingService.info('Files deleted');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { paths });
      throw error;
    }
  }
}
