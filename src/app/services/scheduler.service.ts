/* angular */
import { Injectable, inject } from '@angular/core';

/* library */
import { InvokeWrapperService } from '@tauri-front/shared';

export interface ScheduleConfig {
  enabled: boolean;
  interval_hours: number;
  cleaning_type: string;
  paths: string[];
  last_run: string | null;
  next_run: string | null;
}

@Injectable({ providedIn: 'root' })
export class SchedulerService {
  private api = inject(InvokeWrapperService);

  async getScheduleConfig(): Promise<ScheduleConfig | null> {
    try {
      return await this.api.invoke<ScheduleConfig>('get_schedule_config');
    } catch {
      return null;
    }
  }

  async saveScheduleConfig(config: ScheduleConfig): Promise<void> {
    await this.api.invoke<void>('save_schedule_config', { config });
  }

  async deleteScheduleConfig(): Promise<void> {
    await this.api.invoke<void>('delete_schedule_config');
  }

  async runCleaningNow(cleaningType: string): Promise<void> {
    await this.api.invoke<void>('run_cleaning_now', { cleaningType });
  }
}
