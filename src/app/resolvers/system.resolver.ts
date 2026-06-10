import { inject } from '@angular/core';
import { ResolveFn } from '@angular/router';
import { ApiService } from '@services/api.service';
import { SystemServiceItem } from '@models/system.model';

export const systemServicesResolver: ResolveFn<SystemServiceItem[]> = async () => {
  const api = inject(ApiService);

  try {
    return await api.invoke<SystemServiceItem[]>('getAllServices');
  } catch (error) {
    throw error;
  }
};

export const systemStatsResolver: ResolveFn<{
  cpu_usage: number;
  memory_usage_percent: number;
}> = async () => {
  const api = inject(ApiService);

  try {
    return await api.invoke<{ cpu_usage: number; memory_usage_percent: number }>(
      'get_system_stats'
    );
  } catch (error) {
    throw error;
  }
};
