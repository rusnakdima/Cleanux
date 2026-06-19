import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
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

  constructor() {}

  async scan(rootPath: string): Promise<EmptyDirectory[]> {
    try {
      const result = await this.api.invoke<EmptyDirectory[]>('find_empty_directories', {
        path: rootPath,
      });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async scanNested(rootPath: string): Promise<EmptyDirectory[]> {
    try {
      const result = await this.api.invoke<EmptyDirectory[]>('find_nested_empty_directories', {
        path: rootPath,
      });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async removeEmptyDirectories(paths: string[]): Promise<RemoveResult> {
    try {
      const result = await this.api.invoke<RemoveResult>('remove_empty_directories', { paths });
      return result;
    } catch (error) {
      throw error;
    }
  }
}
