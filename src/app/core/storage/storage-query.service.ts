import { Injectable, inject } from '@angular/core';
import { StorageEntityService, EntityFilter, PaginationParams } from './storage-entity.service';
import { StorageCacheService } from './storage-cache.service';

export interface QueryOptions {
  useCache?: boolean;
  cacheTtl?: number;
  skipCache?: boolean;
}

@Injectable({ providedIn: 'root' })
export class StorageQueryService {
  private entityService = inject(StorageEntityService);
  private cacheService = inject(StorageCacheService);

  async query<T>(
    table: string,
    filter?: EntityFilter,
    pagination?: PaginationParams,
    sortBy?: string,
    sortAsc: boolean = true,
    options: QueryOptions = {}
  ): Promise<T[]> {
    const { useCache = true, cacheTtl, skipCache = false } = options;

    const cacheKey = this.buildCacheKey(table, filter, pagination, sortBy, sortAsc);

    if (useCache && !skipCache && this.cacheService.has(cacheKey)) {
      return this.cacheService.get<T[]>(cacheKey) ?? [];
    }

    const result = await this.entityService.findMany<T>(table, filter, pagination, sortBy, sortAsc);

    if (useCache) {
      this.cacheService.set(cacheKey, result, cacheTtl);
    }

    return result;
  }

  async queryById<T>(table: string, id: string, options: QueryOptions = {}): Promise<T | null> {
    const { useCache = true, cacheTtl } = options;

    const cacheKey = `${table}:${id}`;

    if (useCache && this.cacheService.has(cacheKey)) {
      return this.cacheService.get<T>(cacheKey) ?? null;
    }

    const result = await this.entityService.findById<T>(table, id);

    if (useCache && result) {
      this.cacheService.set(cacheKey, result, cacheTtl);
    }

    return result;
  }

  invalidate(table: string, id?: string): void {
    if (id) {
      this.cacheService.delete(`${table}:${id}`);
    } else {
      this.cacheService.clearPattern(`^${table}:`);
    }
  }

  invalidateAll(): void {
    this.cacheService.clear();
  }

  private buildCacheKey(
    table: string,
    filter?: EntityFilter,
    pagination?: PaginationParams,
    sortBy?: string,
    sortAsc?: boolean
  ): string {
    return `${table}:${JSON.stringify({ filter, pagination, sortBy, sortAsc })}`;
  }
}
