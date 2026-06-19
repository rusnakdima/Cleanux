/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';

/* models */
import {
  CacheFileItem,
  TrashFileItem,
  LogFileItem,
  LargeFileItem,
  ScanSummary,
  PaginatedData,
} from '@entities/system.model';

export type {
  CacheFileItem,
  TrashFileItem,
  LogFileItem,
  LargeFileItem,
  ScanSummary,
  PaginatedData,
} from '@entities/system.model';

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

  private inFlightRequests = new Map<string, Promise<unknown>>();
  private abortController: AbortController | null = null;

  constructor() {}

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
        return {
          data: response.data,
          has_more: response.has_more,
          total: response.total,
        };
      } catch (error) {
        throw error;
      } finally {
        this.inFlightRequests.delete(cacheKey);
      }
    })();

    this.inFlightRequests.set(cacheKey, promise);
    return promise;
  }

  async getTrashFiles(limit?: number, offset?: number): Promise<TrashFileItem[]> {
    try {
      const result = await this.api.invoke<TrashFileItem[]>('getTrashFiles', {
        limit: limit ?? null,
        offset: offset ?? null,
      });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getSystemLogs(limit?: number, offset?: number): Promise<LogFileItem[]> {
    try {
      const result = await this.api.invoke<LogFileItem[]>('getSystemLogs', {
        limit: limit ?? null,
        offset: offset ?? null,
      });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getLargeFiles(limit?: number, offset?: number): Promise<PaginatedData<LargeFileItem>> {
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
        return {
          data: response.data,
          has_more: response.has_more,
          total: response.total,
        };
      } catch (error) {
        throw error;
      } finally {
        this.inFlightRequests.delete(cacheKey);
      }
    })();

    this.inFlightRequests.set(cacheKey, promise);
    return promise;
  }

  async getCacheSummary(): Promise<ScanSummary> {
    try {
      const result = await this.api.invoke<ScanSummary>('getCacheSummary');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getTrashSummary(): Promise<ScanSummary> {
    try {
      const result = await this.api.invoke<ScanSummary>('getTrashSummary');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getLogSummary(): Promise<ScanSummary> {
    try {
      const result = await this.api.invoke<ScanSummary>('getLogSummary');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getLargeFilesSummary(): Promise<ScanSummary> {
    try {
      const result = await this.api.invoke<ScanSummary>('getLargeFilesSummary');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async clearSelectedCacheFiles(paths: string[]): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clearSelectedCacheFiles', { paths });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async clearSelectedTrashFiles(paths: string[]): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clearSelectedTrashFiles', { paths });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async clearSelectedLogFiles(paths: string[]): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clearSelectedLogFiles', { paths });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async clearSelectedLargeFiles(paths: string[]): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clearSelectedLargeFiles', { paths });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async clearTrash(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clearTrash');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async clearCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clearCache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async clearAllLogs(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clearAllLogs');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async clearAllLargeFiles(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clearAllLargeFiles');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async previewFile(path: string): Promise<FilePreviewResult> {
    try {
      const result = await this.api.invoke<FilePreviewResult>('previewFile', { path });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async openFile(path: string, command?: string): Promise<void> {
    try {
      await this.api.invoke<void>('openFile', { path, command: command || null });
    } catch (error) {
      throw error;
    }
  }

  async deleteFiles(paths: string[]): Promise<string> {
    try {
      const result = await this.api.invoke<string>('deleteFiles', { paths });
      return result;
    } catch (error) {
      throw error;
    }
  }
}
