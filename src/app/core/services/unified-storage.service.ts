import { Injectable, inject } from '@angular/core';
import {
  StorageEntityService,
  EntityFilter,
  PaginationParams,
} from '../../services/storage-entity.service';
import { StorageCacheService } from './storage-cache.service';
import { StorageQueryService } from './storage-query.service';

@Injectable({ providedIn: 'root' })
export class UnifiedStorageService {
  private entityService = inject(StorageEntityService);
  private cacheService = inject(StorageCacheService);
  private queryService = inject(StorageQueryService);

  get entity(): StorageEntityService {
    return this.entityService;
  }

  get cache(): StorageCacheService {
    return this.cacheService;
  }

  get query(): StorageQueryService {
    return this.queryService;
  }

  async save<T>(table: string, data: any, id?: string): Promise<T> {
    if (id) {
      const result = await this.entityService.update<T>(table, id, data);
      this.queryService.invalidate(table, id);
      return result;
    } else {
      const result = await this.entityService.create<T>(table, data);
      this.queryService.invalidate(table);
      return result;
    }
  }

  async delete(table: string, id: string): Promise<boolean> {
    const result = await this.entityService.delete(table, id);
    this.queryService.invalidate(table, id);
    return result;
  }

  async findById<T>(
    table: string,
    id: string,
    options?: { useCache?: boolean; cacheTtl?: number }
  ): Promise<T | null> {
    return this.queryService.queryById<T>(table, id, options);
  }

  async findMany<T>(
    table: string,
    filter?: EntityFilter,
    pagination?: PaginationParams,
    sortBy?: string,
    sortAsc: boolean = true,
    options?: { useCache?: boolean; cacheTtl?: number }
  ): Promise<T[]> {
    return this.queryService.query<T>(table, filter, pagination, sortBy, sortAsc, options);
  }

  async count(table: string, filter?: EntityFilter): Promise<number> {
    return this.entityService.count(table, filter);
  }

  clearCache(): void {
    this.queryService.invalidateAll();
  }

  invalidate(table: string, id?: string): void {
    this.queryService.invalidate(table, id);
  }
}
