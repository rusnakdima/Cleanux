import { inject } from '@angular/core';
import { ResolveFn, Router } from '@angular/router';
import { TauriApiService } from '@api/tauri-api.service';
import { SystemServiceItem } from '@models/system.model';

export const systemServicesResolver: ResolveFn<SystemServiceItem[]> = async () => {
  const api = inject(TauriApiService);
  const router = inject(Router);

  try {
    return await api.invoke<SystemServiceItem[]>('getAllServices');
  } catch (error) {
    console.error('Failed to resolve system services:', error);
    return [];
  }
};

export const systemStatsResolver: ResolveFn<{
  cpu_usage: number;
  memory_usage_percent: number;
}> = async () => {
  const api = inject(TauriApiService);

  try {
    return await api.invoke<{ cpu_usage: number; memory_usage_percent: number }>(
      'get_system_stats'
    );
  } catch (error) {
    console.error('Failed to resolve system stats:', error);
    return { cpu_usage: 0, memory_usage_percent: 0 };
  }
};
