/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';

/* models */
import { DuplicateGroup, DuplicateScanResult } from '@entities/duplicate.model';

@Injectable({
  providedIn: 'root',
})
export class DuplicateService {
  private api = inject(ApiService);

  constructor() {}

  async findDuplicates(path: string, extensionFilter?: string): Promise<DuplicateScanResult> {
    try {
      const result = await this.api.invoke<DuplicateScanResult>('find_duplicates', {
        path,
        extension_filter: extensionFilter || null,
      });
      return result;
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
