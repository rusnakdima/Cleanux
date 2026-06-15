/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';
import { LoggerService } from '@services/logger.service';

/* models */
import { DuplicateGroup, DuplicateScanResult } from '@models/duplicate.model';

@Injectable({
  providedIn: 'root',
})
export class DuplicateService {
  private api = inject(ApiService);
  private loggingService = new LoggerService();

  constructor() {
    this.loggingService.info('DuplicateService initialized');
  }

  async findDuplicates(path: string, extensionFilter?: string): Promise<DuplicateScanResult> {
    this.loggingService.info('Finding duplicates', { path, extensionFilter });
    try {
      const result = await this.api.invoke<DuplicateScanResult>('find_duplicates', {
        path,
        extension_filter: extensionFilter || null,
      });
      this.loggingService.info('Duplicates found', { groups: result.groups.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { path });
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
