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
} from '@models/system.model';

export type {
  CacheFileItem,
  TrashFileItem,
  LogFileItem,
  LargeFileItem,
  ScanSummary,
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

  async getCacheFiles(): Promise<CacheFileItem[]> {
    return await this.api.invoke<CacheFileItem[]>('getCacheFiles');
  }

  async getTrashFiles(): Promise<TrashFileItem[]> {
    return await this.api.invoke<TrashFileItem[]>('getTrashFiles');
  }

  async getSystemLogs(): Promise<LogFileItem[]> {
    return await this.api.invoke<LogFileItem[]>('getSystemLogs');
  }

  async getLargeFiles(): Promise<LargeFileItem[]> {
    return await this.api.invoke<LargeFileItem[]>('getLargeFiles');
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
