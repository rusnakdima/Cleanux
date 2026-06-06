/* sys lib */
import { Injectable } from '@angular/core';

/* services */
import { BaseApiService } from '@services/base-api.service';

/* models */
import { ScheduleConfig } from '@models/schedule.model';

@Injectable({
  providedIn: 'root',
})
export class SchedulerService extends BaseApiService {
  async getScheduleConfig(): Promise<ScheduleConfig | null> {
    try {
      return await this.call<ScheduleConfig | null>('get_schedule_config');
    } catch {
      return null;
    }
  }

  async saveScheduleConfig(config: ScheduleConfig): Promise<void> {
    await this.call('save_schedule_config', { config });
  }

  async deleteScheduleConfig(): Promise<void> {
    await this.call('delete_schedule_config');
  }

  async runCleaningNow(cleaningType: string): Promise<void> {
    await this.call('run_cleaning_now', { cleaningType });
  }
}
