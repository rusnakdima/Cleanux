/* sys lib */
import { Injectable } from '@angular/core';

/* services */
import { BaseApiService } from '@services/base-api.service';

/* models */
import { DuplicateGroup, DuplicateScanResult } from '@models/duplicate.model';

@Injectable({
  providedIn: 'root',
})
export class DuplicateService extends BaseApiService {
  async findDuplicates(path: string, extensionFilter?: string): Promise<DuplicateScanResult> {
    return this.call<DuplicateScanResult>('find_duplicates', {
      path,
      extension_filter: extensionFilter || null,
    });
  }

  async deleteFiles(paths: string[]): Promise<string> {
    return this.call<string>('deleteFiles', { paths });
  }
}
