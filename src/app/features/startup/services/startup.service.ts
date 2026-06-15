import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { LoggerService } from '@services/logger.service';
import { StartupItem } from '@models/startup.model';

@Injectable({
  providedIn: 'root',
})
export class StartupService {
  private api = inject(ApiService);
  private loggingService = new LoggerService();

  constructor() {
    this.loggingService.info('StartupService initialized');
  }

  async getStartupItems(): Promise<StartupItem[]> {
    this.loggingService.info('Getting startup items');
    try {
      const result = await this.api.invoke<StartupItem[]>('get_startup_items');
      this.loggingService.info('Startup items retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async disableStartupItem(path: string): Promise<string> {
    this.loggingService.info('Disabling startup item', { path });
    try {
      const result = await this.api.invoke<string>('disable_startup_item', { path });
      this.loggingService.info('Startup item disabled');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { path });
      throw error;
    }
  }

  async enableStartupItem(path: string): Promise<string> {
    this.loggingService.info('Enabling startup item', { path });
    try {
      const result = await this.api.invoke<string>('enable_startup_item', { path });
      this.loggingService.info('Startup item enabled');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { path });
      throw error;
    }
  }
}
