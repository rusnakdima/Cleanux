/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';

/* models */
import { ScheduleConfig } from '@entities/schedule.model';

@Injectable({
  providedIn: 'root',
})
export class SchedulerService {
  private api = inject(ApiService);

  async getScheduleConfig(): Promise<ScheduleConfig | null> {
    try {
      const result = await this.api.invoke<ScheduleConfig | null>('get_schedule_config');
      return result;
    } catch (error) {
      return null;
    }
  }

  async saveScheduleConfig(config: ScheduleConfig): Promise<void> {
    await this.api.invoke('save_schedule_config', { config });
  }

  async deleteScheduleConfig(): Promise<void> {
    await this.api.invoke('delete_schedule_config');
  }

  async runCleaningNow(cleaningType: string): Promise<void> {
    await this.api.invoke('run_cleaning_now', { cleaningType });
  }
}
