/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';
import { LoggerService } from '@services/logger.service';

/* models */
import { ScheduleConfig } from '@models/schedule.model';

@Injectable({
  providedIn: 'root',
})
export class SchedulerService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'SchedulerService', 'init', 'SchedulerService initialized');
  }

  async getScheduleConfig(): Promise<ScheduleConfig | null> {
    this.logger.logInfo(
      'service',
      'SchedulerService',
      'getScheduleConfig',
      'Getting schedule config'
    );
    try {
      const result = await this.api.invoke<ScheduleConfig | null>('get_schedule_config');
      this.logger.logInfo(
        'service',
        'SchedulerService',
        'getScheduleConfig',
        'Schedule config retrieved',
        { hasConfig: !!result }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'SchedulerService',
        'getScheduleConfig',
        'Operation failed',
        error as Error
      );
      return null;
    }
  }

  async saveScheduleConfig(config: ScheduleConfig): Promise<void> {
    this.logger.logInfo(
      'service',
      'SchedulerService',
      'saveScheduleConfig',
      'Saving schedule config'
    );
    try {
      await this.api.invoke('save_schedule_config', { config });
      this.logger.logInfo(
        'service',
        'SchedulerService',
        'saveScheduleConfig',
        'Schedule config saved'
      );
    } catch (error) {
      this.logger.logError(
        'service',
        'SchedulerService',
        'saveScheduleConfig',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async deleteScheduleConfig(): Promise<void> {
    this.logger.logInfo(
      'service',
      'SchedulerService',
      'deleteScheduleConfig',
      'Deleting schedule config'
    );
    try {
      await this.api.invoke('delete_schedule_config');
      this.logger.logInfo(
        'service',
        'SchedulerService',
        'deleteScheduleConfig',
        'Schedule config deleted'
      );
    } catch (error) {
      this.logger.logError(
        'service',
        'SchedulerService',
        'deleteScheduleConfig',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async runCleaningNow(cleaningType: string): Promise<void> {
    this.logger.logInfo('service', 'SchedulerService', 'runCleaningNow', 'Running cleaning now', {
      cleaningType,
    });
    try {
      await this.api.invoke('run_cleaning_now', { cleaningType });
      this.logger.logInfo('service', 'SchedulerService', 'runCleaningNow', 'Cleaning started');
    } catch (error) {
      this.logger.logError(
        'service',
        'SchedulerService',
        'runCleaningNow',
        'Operation failed',
        error as Error,
        { cleaningType }
      );
      throw error;
    }
  }
}
