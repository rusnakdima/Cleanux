import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';
import { EmptyDirectory } from '@components/empty-dir-cleaner/empty-dir-cleaner.component';

export interface RemoveResult {
  removed: number;
  failed: string[];
  total: number;
}

@Injectable({
  providedIn: 'root',
})
export class EmptyDirCleanerService {
  private api = inject(ApiService);
  private loggingService = getLoggingService();

  constructor() {
    this.loggingService.info('EmptyDirCleanerService initialized');
  }

  async scan(rootPath: string): Promise<EmptyDirectory[]> {
    this.loggingService.info('Scanning for empty directories', { rootPath });
    try {
      const result = await this.api.invoke<EmptyDirectory[]>('find_empty_directories', {
        path: rootPath,
      });
      this.loggingService.info('Empty directories scanned', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { rootPath });
      throw error;
    }
  }

  async scanNested(rootPath: string): Promise<EmptyDirectory[]> {
    this.loggingService.info('Scanning for nested empty directories', { rootPath });
    try {
      const result = await this.api.invoke<EmptyDirectory[]>('find_nested_empty_directories', {
        path: rootPath,
      });
      this.loggingService.info('Nested empty directories scanned', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { rootPath });
      throw error;
    }
  }

  async removeEmptyDirectories(paths: string[]): Promise<RemoveResult> {
    this.loggingService.info('Removing empty directories', { count: paths.length });
    try {
      const result = await this.api.invoke<RemoveResult>('remove_empty_directories', { paths });
      this.loggingService.info('Empty directories removed', { removed: result.removed, failed: result.failed.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { paths });
      throw error;
    }
  }
}
