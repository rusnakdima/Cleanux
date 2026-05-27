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

export interface PaginatedResult<T> {
  data: T[];
  has_more: boolean;
  total: number;
}

@Injectable({
  providedIn: 'root',
})
export class FileService {
  private api = inject(ApiService);

  private inFlightRequests = new Map<string, Promise<unknown>>();
  private abortController: AbortController | null = null;

  private getCacheKey(cmd: string, params?: Record<string, unknown>): string {
    return `${cmd}:${JSON.stringify(params ?? {})}`;
  }

  private cancelPreviousRequest(): void {
    if (this.abortController) {
      this.abortController.abort();
      this.abortController = null;
    }
  }

  async getCacheFiles(limit?: number, offset?: number): Promise<PaginatedResult<CacheFileItem>> {
    const cacheKey = this.getCacheKey('getCacheFiles', { limit, offset });

    if (this.inFlightRequests.has(cacheKey)) {
      return this.inFlightRequests.get(cacheKey) as Promise<PaginatedResult<CacheFileItem>>;
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
      } finally {
        this.inFlightRequests.delete(cacheKey);
      }
    })();

    this.inFlightRequests.set(cacheKey, promise);
    return promise;
  }

  async getTrashFiles(limit?: number, offset?: number): Promise<TrashFileItem[]> {
    return await this.api.invoke<TrashFileItem[]>('getTrashFiles', { limit: limit ?? null, offset: offset ?? null });
  }

  async getSystemLogs(limit?: number, offset?: number): Promise<LogFileItem[]> {
    return await this.api.invoke<LogFileItem[]>('getSystemLogs', { limit: limit ?? null, offset: offset ?? null });
  }

  async getLargeFiles(limit?: number, offset?: number): Promise<PaginatedResult<LargeFileItem>> {
    const cacheKey = this.getCacheKey('getLargeFiles', { limit, offset });

    if (this.inFlightRequests.has(cacheKey)) {
      return this.inFlightRequests.get(cacheKey) as Promise<PaginatedResult<LargeFileItem>>;
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
      } finally {
        this.inFlightRequests.delete(cacheKey);
      }
    })();

    this.inFlightRequests.set(cacheKey, promise);
    return promise;
  }

  async getCacheSummary(): Promise<ScanSummary> {
    return await this.api.invoke<ScanSummary>('getCacheSummary');
  }

  async getTrashSummary(): Promise<ScanSummary> {
    return await this.api.invoke<ScanSummary>('getTrashSummary');
  }

  async getLogSummary(): Promise<ScanSummary> {
    return await this.api.invoke<ScanSummary>('getLogSummary');
  }

  async getLargeFilesSummary(): Promise<ScanSummary> {
    return await this.api.invoke<ScanSummary>('getLargeFilesSummary');
  }

  async clearSelectedCacheFiles(paths: string[]): Promise<string> {
    return await this.api.invoke<string>('clearSelectedCacheFiles', { paths });
  }

  async clearSelectedTrashFiles(paths: string[]): Promise<string> {
    return await this.api.invoke<string>('clearSelectedTrashFiles', { paths });
  }

  async clearSelectedLogFiles(paths: string[]): Promise<string> {
    return await this.api.invoke<string>('clearSelectedLogFiles', { paths });
  }

  async clearSelectedLargeFiles(paths: string[]): Promise<string> {
    return await this.api.invoke<string>('clearSelectedLargeFiles', { paths });
  }

  async clearTrash(): Promise<string> {
    return await this.api.invoke<string>('clearTrash');
  }

  async clearCache(): Promise<string> {
    return await this.api.invoke<string>('clearCache');
  }

  async clearAllLogs(): Promise<string> {
    return await this.api.invoke<string>('clearAllLogs');
  }

  async clearAllLargeFiles(): Promise<string> {
    return await this.api.invoke<string>('clearAllLargeFiles');
  }

  async previewFile(path: string): Promise<FilePreviewResult> {
    return await this.api.invoke<FilePreviewResult>('previewFile', { path });
  }

  async openFile(path: string, command?: string): Promise<void> {
    return await this.api.invoke<void>('openFile', { path, command: command || null });
  }

  async deleteFiles(paths: string[]): Promise<string> {
    return await this.api.invoke<string>('deleteFiles', { paths });
  }
}
