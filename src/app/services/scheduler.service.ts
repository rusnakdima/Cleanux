/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';
import { LoggerService } from './logger.service';

/* models */
import { ScheduleConfig } from '@models/schedule.model';

@Injectable({
  providedIn: 'root',
})
export class SchedulerService {
  private api = inject(ApiService);
  private loggingService = new LoggerService();

  constructor() {
    this.loggingService.info('SchedulerService initialized');
  }

  async getScheduleConfig(): Promise<ScheduleConfig | null> {
    this.loggingService.info('Getting schedule config');
    try {
      const result = await this.api.invoke<ScheduleConfig | null>('get_schedule_config');
      this.loggingService.info('Schedule config retrieved', { hasConfig: !!result });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      return null;
    }
  }

  async saveScheduleConfig(config: ScheduleConfig): Promise<void> {
    this.loggingService.info('Saving schedule config');
    try {
      await this.api.invoke('save_schedule_config', { config });
      this.loggingService.info('Schedule config saved');
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async deleteScheduleConfig(): Promise<void> {
    this.loggingService.info('Deleting schedule config');
    try {
      await this.api.invoke('delete_schedule_config');
      this.loggingService.info('Schedule config deleted');
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async runCleaningNow(cleaningType: string): Promise<void> {
    this.loggingService.info('Running cleaning now', { cleaningType });
    try {
      await this.api.invoke('run_cleaning_now', { cleaningType });
      this.loggingService.info('Cleaning started');
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { cleaningType });
      throw error;
    }
  }
}
