/* sys lib */
import { Injectable, inject, signal } from '@angular/core';

/* services */
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';

/* models */
import { DuplicateGroup, DuplicateScanResult } from '@models/duplicate.model';

@Injectable({
  providedIn: 'root',
})
export class DuplicateService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'DuplicateService', 'init', 'DuplicateService initialized');
  }

  async findDuplicates(path: string, extensionFilter?: string): Promise<DuplicateScanResult> {
    this.logger.logInfo('service', 'DuplicateService', 'findDuplicates', 'Finding duplicates', {
      path,
      extensionFilter,
    });
    try {
      const result = await this.api.invoke<DuplicateScanResult>('find_duplicates', {
        path,
        extension_filter: extensionFilter || null,
      });
      this.logger.logInfo('service', 'DuplicateService', 'findDuplicates', 'Duplicates found', {
        groups: result.groups.length,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'DuplicateService',
        'findDuplicates',
        'Operation failed',
        error as Error,
        { path }
      );
      throw error;
    }
  }

  async deleteFiles(paths: string[]): Promise<string> {
    this.logger.logInfo('service', 'DuplicateService', 'deleteFiles', 'Deleting files', {
      count: paths.length,
    });
    try {
      const result = await this.api.invoke<string>('deleteFiles', { paths });
      this.logger.logInfo('service', 'DuplicateService', 'deleteFiles', 'Files deleted');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'DuplicateService',
        'deleteFiles',
        'Operation failed',
        error as Error,
        { paths }
      );
      throw error;
    }
  }
}
