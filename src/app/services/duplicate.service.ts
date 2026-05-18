/* sys lib */
import { Injectable, inject, signal } from '@angular/core';

/* services */
import { ApiService } from './api.service';

/* models */
import { DuplicateGroup, DuplicateScanResult } from '@models/duplicate.model';

@Injectable({
  providedIn: 'root',
})
export class DuplicateService {
  private api = inject(ApiService);

  async findDuplicates(path: string, extensionFilter?: string): Promise<DuplicateScanResult> {
    return await this.api.invoke<DuplicateScanResult>('find_duplicates', {
      path,
      extension_filter: extensionFilter || null,
    });
  }

  async deleteFiles(paths: string[]): Promise<string> {
    return await this.api.invoke<string>('deleteFiles', { paths });
  }
}
