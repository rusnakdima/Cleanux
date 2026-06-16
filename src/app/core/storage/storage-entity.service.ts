import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';

export interface EntityFilter {
  [key: string]: any;
}

export interface PaginationParams {
  page?: number;
  limit?: number;
}

export interface SortParams {
  sortBy?: string;
  sortAsc?: boolean;
}

@Injectable({ providedIn: 'root' })
export class StorageEntityService {
  private api = inject(ApiService);

  async findById<T>(table: string, id: string): Promise<T | null> {
    const commandMap: Record<string, string> = {
      health_snapshots: 'crud_get_health_snapshot',
      cleaning_reports: 'get_cleaning_report',
      cleaning_profiles: 'get_cleaning_profile',
      automation_recipes: 'get_automation_recipe',
      execution_history: 'crud_get_execution_history',
    };
    const command = commandMap[table] || `crud_get_${table}`;
    return await this.api.invoke<T>(command, { id });
  }

  async findMany<T>(
    table: string,
    filter?: EntityFilter,
    pagination?: PaginationParams,
    sortBy?: string,
    sortAsc: boolean = true
  ): Promise<T[]> {
    const commandMap: Record<string, string> = {
      health_snapshots: 'crud_get_health_snapshots',
      cleaning_reports: 'get_cleaning_reports',
      cleaning_profiles: 'get_cleaning_profiles',
      automation_recipes: 'get_automation_recipes',
      execution_history: 'crud_get_execution_history',
    };
    const command = commandMap[table] || `get_${table}s`;
    return await this.api.invoke<T[]>(command, {
      ...pagination,
      filter,
      sortBy,
      sortAsc,
    });
  }

  async create<T>(table: string, data: any): Promise<T> {
    const commandMap: Record<string, string> = {
      health_snapshots: 'crud_create_health_snapshot',
      cleaning_reports: 'create_cleaning_report',
      cleaning_profiles: 'create_cleaning_profile',
      automation_recipes: 'create_automation_recipe',
    };
    const command = commandMap[table] || `create_${table}`;
    return await this.api.invoke<T>(command, { data });
  }

  async update<T>(table: string, id: string, data: any): Promise<T> {
    const commandMap: Record<string, string> = {
      cleaning_profiles: 'update_cleaning_profile',
      automation_recipes: 'update_automation_recipe',
    };
    const command = commandMap[table] || `update_${table}`;
    return await this.api.invoke<T>(command, { id, data });
  }

  async patch<T>(table: string, id: string, patch: any): Promise<T> {
    return await this.api.invoke<T>(`patch_${table}`, { id, patch });
  }

  async delete(table: string, id: string): Promise<boolean> {
    const commandMap: Record<string, string> = {
      cleaning_profiles: 'delete_cleaning_profile',
      automation_recipes: 'delete_automation_recipe',
    };
    const command = commandMap[table] || `delete_${table}`;
    return await this.api.invoke<boolean>(command, { id });
  }

  async count(table: string, filter?: EntityFilter): Promise<number> {
    return await this.api.invoke<number>(`count_${table}`, { filter });
  }

  async query<T>(table: string, filter: EntityFilter, pagination?: PaginationParams): Promise<T[]> {
    return this.findMany<T>(table, filter, pagination);
  }
}
