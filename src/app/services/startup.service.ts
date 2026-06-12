import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';
import { StartupItem } from '@models/startup.model';

@Injectable({
  providedIn: 'root',
})
export class StartupService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'StartupService', 'init', 'StartupService initialized');
  }

  async getStartupItems(): Promise<StartupItem[]> {
    this.logger.logInfo('service', 'StartupService', 'getStartupItems', 'Getting startup items');
    try {
      const result = await this.api.invoke<StartupItem[]>('get_startup_items');
      this.logger.logInfo(
        'service',
        'StartupService',
        'getStartupItems',
        'Startup items retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'StartupService',
        'getStartupItems',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async disableStartupItem(path: string): Promise<string> {
    this.logger.logInfo(
      'service',
      'StartupService',
      'disableStartupItem',
      'Disabling startup item',
      { path }
    );
    try {
      const result = await this.api.invoke<string>('disable_startup_item', { path });
      this.logger.logInfo(
        'service',
        'StartupService',
        'disableStartupItem',
        'Startup item disabled'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'StartupService',
        'disableStartupItem',
        'Operation failed',
        error as Error,
        { path }
      );
      throw error;
    }
  }

  async enableStartupItem(path: string): Promise<string> {
    this.logger.logInfo('service', 'StartupService', 'enableStartupItem', 'Enabling startup item', {
      path,
    });
    try {
      const result = await this.api.invoke<string>('enable_startup_item', { path });
      this.logger.logInfo('service', 'StartupService', 'enableStartupItem', 'Startup item enabled');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'StartupService',
        'enableStartupItem',
        'Operation failed',
        error as Error,
        { path }
      );
      throw error;
    }
  }
}
