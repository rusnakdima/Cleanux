import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { LoggerService } from '@services/logger.service';
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
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo(
      'service',
      'EmptyDirCleanerService',
      'init',
      'EmptyDirCleanerService initialized'
    );
  }

  async scan(rootPath: string): Promise<EmptyDirectory[]> {
    this.logger.logInfo(
      'service',
      'EmptyDirCleanerService',
      'scan',
      'Scanning for empty directories',
      { rootPath }
    );
    try {
      const result = await this.api.invoke<EmptyDirectory[]>('find_empty_directories', {
        path: rootPath,
      });
      this.logger.logInfo(
        'service',
        'EmptyDirCleanerService',
        'scan',
        'Empty directories scanned',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'EmptyDirCleanerService',
        'scan',
        'Operation failed',
        error as Error,
        { rootPath }
      );
      throw error;
    }
  }

  async scanNested(rootPath: string): Promise<EmptyDirectory[]> {
    this.logger.logInfo(
      'service',
      'EmptyDirCleanerService',
      'scanNested',
      'Scanning for nested empty directories',
      { rootPath }
    );
    try {
      const result = await this.api.invoke<EmptyDirectory[]>('find_nested_empty_directories', {
        path: rootPath,
      });
      this.logger.logInfo(
        'service',
        'EmptyDirCleanerService',
        'scanNested',
        'Nested empty directories scanned',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'EmptyDirCleanerService',
        'scanNested',
        'Operation failed',
        error as Error,
        { rootPath }
      );
      throw error;
    }
  }

  async removeEmptyDirectories(paths: string[]): Promise<RemoveResult> {
    this.logger.logInfo(
      'service',
      'EmptyDirCleanerService',
      'removeEmptyDirectories',
      'Removing empty directories',
      { count: paths.length }
    );
    try {
      const result = await this.api.invoke<RemoveResult>('remove_empty_directories', { paths });
      this.logger.logInfo(
        'service',
        'EmptyDirCleanerService',
        'removeEmptyDirectories',
        'Empty directories removed',
        { removed: result.removed, failed: result.failed.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'EmptyDirCleanerService',
        'removeEmptyDirectories',
        'Operation failed',
        error as Error,
        { paths }
      );
      throw error;
    }
  }
}
