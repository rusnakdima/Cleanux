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

  async scan(rootPath: string): Promise<EmptyDirectory[]> {
    return this.api.invoke<EmptyDirectory[]>('find_empty_directories', {
      path: rootPath,
    });
  }

  async scanNested(rootPath: string): Promise<EmptyDirectory[]> {
    return this.api.invoke<EmptyDirectory[]>('find_nested_empty_directories', {
      path: rootPath,
    });
  }

  async removeEmptyDirectories(paths: string[]): Promise<RemoveResult> {
    return this.api.invoke<RemoveResult>('remove_empty_directories', { paths });
  }
}
