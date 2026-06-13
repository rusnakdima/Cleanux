/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';

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
  private logger = inject(LoggerService);

  private inFlightRequests = new Map<string, Promise<unknown>>();
  private abortController: AbortController | null = null;

  constructor() {
    this.logger.logInfo('service', 'FileService', 'init', 'FileService initialized');
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
    this.logger.logInfo('service', 'FileService', 'getCacheFiles', 'Getting cache files', {
      limit,
      offset,
    });
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
        this.logger.logInfo('service', 'FileService', 'getCacheFiles', 'Cache files retrieved', {
          total: response.total,
        });
        return {
          data: response.data,
          has_more: response.has_more,
          total: response.total,
        };
      } catch (error) {
        this.logger.logError(
          'service',
          'FileService',
          'getCacheFiles',
          'Operation failed',
          error as Error
        );
        throw error;
      } finally {
        this.inFlightRequests.delete(cacheKey);
      }
    })();

    this.inFlightRequests.set(cacheKey, promise);
    return promise;
  }

  async getTrashFiles(limit?: number, offset?: number): Promise<TrashFileItem[]> {
    this.logger.logInfo('service', 'FileService', 'getTrashFiles', 'Getting trash files', {
      limit,
      offset,
    });
    try {
      const result = await this.api.invoke<TrashFileItem[]>('getTrashFiles', {
        limit: limit ?? null,
        offset: offset ?? null,
      });
      this.logger.logInfo('service', 'FileService', 'getTrashFiles', 'Trash files retrieved', {
        count: result.length,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'getTrashFiles',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getSystemLogs(limit?: number, offset?: number): Promise<LogFileItem[]> {
    this.logger.logInfo('service', 'FileService', 'getSystemLogs', 'Getting system logs', {
      limit,
      offset,
    });
    try {
      const result = await this.api.invoke<LogFileItem[]>('getSystemLogs', {
        limit: limit ?? null,
        offset: offset ?? null,
      });
      this.logger.logInfo('service', 'FileService', 'getSystemLogs', 'System logs retrieved', {
        count: result.length,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'getSystemLogs',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getLargeFiles(limit?: number, offset?: number): Promise<PaginatedData<LargeFileItem>> {
    this.logger.logInfo('service', 'FileService', 'getLargeFiles', 'Getting large files', {
      limit,
      offset,
    });
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
        this.logger.logInfo('service', 'FileService', 'getLargeFiles', 'Large files retrieved', {
          total: response.total,
        });
        return {
          data: response.data,
          has_more: response.has_more,
          total: response.total,
        };
      } catch (error) {
        this.logger.logError(
          'service',
          'FileService',
          'getLargeFiles',
          'Operation failed',
          error as Error
        );
        throw error;
      } finally {
        this.inFlightRequests.delete(cacheKey);
      }
    })();

    this.inFlightRequests.set(cacheKey, promise);
    return promise;
  }

  async getCacheSummary(): Promise<ScanSummary> {
    this.logger.logInfo('service', 'FileService', 'getCacheSummary', 'Getting cache summary');
    try {
      const result = await this.api.invoke<ScanSummary>('getCacheSummary');
      this.logger.logInfo('service', 'FileService', 'getCacheSummary', 'Cache summary retrieved');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'getCacheSummary',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getTrashSummary(): Promise<ScanSummary> {
    this.logger.logInfo('service', 'FileService', 'getTrashSummary', 'Getting trash summary');
    try {
      const result = await this.api.invoke<ScanSummary>('getTrashSummary');
      this.logger.logInfo('service', 'FileService', 'getTrashSummary', 'Trash summary retrieved');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'getTrashSummary',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getLogSummary(): Promise<ScanSummary> {
    this.logger.logInfo('service', 'FileService', 'getLogSummary', 'Getting log summary');
    try {
      const result = await this.api.invoke<ScanSummary>('getLogSummary');
      this.logger.logInfo('service', 'FileService', 'getLogSummary', 'Log summary retrieved');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'getLogSummary',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getLargeFilesSummary(): Promise<ScanSummary> {
    this.logger.logInfo(
      'service',
      'FileService',
      'getLargeFilesSummary',
      'Getting large files summary'
    );
    try {
      const result = await this.api.invoke<ScanSummary>('getLargeFilesSummary');
      this.logger.logInfo(
        'service',
        'FileService',
        'getLargeFilesSummary',
        'Large files summary retrieved'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'getLargeFilesSummary',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async clearSelectedCacheFiles(paths: string[]): Promise<string> {
    this.logger.logInfo(
      'service',
      'FileService',
      'clearSelectedCacheFiles',
      'Clearing selected cache files',
      { count: paths.length }
    );
    try {
      const result = await this.api.invoke<string>('clearSelectedCacheFiles', { paths });
      this.logger.logInfo(
        'service',
        'FileService',
        'clearSelectedCacheFiles',
        'Selected cache files cleared'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'clearSelectedCacheFiles',
        'Operation failed',
        error as Error,
        { paths }
      );
      throw error;
    }
  }

  async clearSelectedTrashFiles(paths: string[]): Promise<string> {
    this.logger.logInfo(
      'service',
      'FileService',
      'clearSelectedTrashFiles',
      'Clearing selected trash files',
      { count: paths.length }
    );
    try {
      const result = await this.api.invoke<string>('clearSelectedTrashFiles', { paths });
      this.logger.logInfo(
        'service',
        'FileService',
        'clearSelectedTrashFiles',
        'Selected trash files cleared'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'clearSelectedTrashFiles',
        'Operation failed',
        error as Error,
        { paths }
      );
      throw error;
    }
  }

  async clearSelectedLogFiles(paths: string[]): Promise<string> {
    this.logger.logInfo(
      'service',
      'FileService',
      'clearSelectedLogFiles',
      'Clearing selected log files',
      { count: paths.length }
    );
    try {
      const result = await this.api.invoke<string>('clearSelectedLogFiles', { paths });
      this.logger.logInfo(
        'service',
        'FileService',
        'clearSelectedLogFiles',
        'Selected log files cleared'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'clearSelectedLogFiles',
        'Operation failed',
        error as Error,
        { paths }
      );
      throw error;
    }
  }

  async clearSelectedLargeFiles(paths: string[]): Promise<string> {
    this.logger.logInfo(
      'service',
      'FileService',
      'clearSelectedLargeFiles',
      'Clearing selected large files',
      { count: paths.length }
    );
    try {
      const result = await this.api.invoke<string>('clearSelectedLargeFiles', { paths });
      this.logger.logInfo(
        'service',
        'FileService',
        'clearSelectedLargeFiles',
        'Selected large files cleared'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'clearSelectedLargeFiles',
        'Operation failed',
        error as Error,
        { paths }
      );
      throw error;
    }
  }

  async clearTrash(): Promise<string> {
    this.logger.logInfo('service', 'FileService', 'clearTrash', 'Clearing trash');
    try {
      const result = await this.api.invoke<string>('clearTrash');
      this.logger.logInfo('service', 'FileService', 'clearTrash', 'Trash cleared');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'clearTrash',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async clearCache(): Promise<string> {
    this.logger.logInfo('service', 'FileService', 'clearCache', 'Clearing cache');
    try {
      const result = await this.api.invoke<string>('clearCache');
      this.logger.logInfo('service', 'FileService', 'clearCache', 'Cache cleared');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'clearCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async clearAllLogs(): Promise<string> {
    this.logger.logInfo('service', 'FileService', 'clearAllLogs', 'Clearing all logs');
    try {
      const result = await this.api.invoke<string>('clearAllLogs');
      this.logger.logInfo('service', 'FileService', 'clearAllLogs', 'All logs cleared');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'clearAllLogs',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async clearAllLargeFiles(): Promise<string> {
    this.logger.logInfo('service', 'FileService', 'clearAllLargeFiles', 'Clearing all large files');
    try {
      const result = await this.api.invoke<string>('clearAllLargeFiles');
      this.logger.logInfo(
        'service',
        'FileService',
        'clearAllLargeFiles',
        'All large files cleared'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'clearAllLargeFiles',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async previewFile(path: string): Promise<FilePreviewResult> {
    this.logger.logInfo('service', 'FileService', 'previewFile', 'Previewing file', { path });
    try {
      const result = await this.api.invoke<FilePreviewResult>('previewFile', { path });
      this.logger.logInfo('service', 'FileService', 'previewFile', 'File previewed', {
        type: result.type,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'previewFile',
        'Operation failed',
        error as Error,
        { path }
      );
      throw error;
    }
  }

  async openFile(path: string, command?: string): Promise<void> {
    this.logger.logInfo('service', 'FileService', 'openFile', 'Opening file', { path, command });
    try {
      await this.api.invoke<void>('openFile', { path, command: command || null });
      this.logger.logInfo('service', 'FileService', 'openFile', 'File opened');
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'openFile',
        'Operation failed',
        error as Error,
        { path, command }
      );
      throw error;
    }
  }

  async deleteFiles(paths: string[]): Promise<string> {
    this.logger.logInfo('service', 'FileService', 'deleteFiles', 'Deleting files', {
      count: paths.length,
    });
    try {
      const result = await this.api.invoke<string>('deleteFiles', { paths });
      this.logger.logInfo('service', 'FileService', 'deleteFiles', 'Files deleted');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'FileService',
        'deleteFiles',
        'Operation failed',
        error as Error,
        { paths }
      );
      throw error;
    }
  }
}
