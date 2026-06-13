/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';

/* models */
import { SystemServiceItem, ProcessItem } from '@models/system.model';

export type { SystemServiceItem, ProcessItem } from '@models/system.model';

@Injectable({
  providedIn: 'root',
})
export class SystemService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'SystemService', 'init', 'SystemService initialized');
  }

  async getSystemServices(): Promise<SystemServiceItem[]> {
    this.logger.logInfo('service', 'SystemService', 'getSystemServices', 'Getting system services');
    try {
      const result = await this.api.invoke<SystemServiceItem[]>('getSystemServices');
      this.logger.logInfo(
        'service',
        'SystemService',
        'getSystemServices',
        'System services retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'SystemService',
        'getSystemServices',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getAllServices(): Promise<SystemServiceItem[]> {
    this.logger.logInfo('service', 'SystemService', 'getAllServices', 'Getting all services');
    try {
      const result = await this.api.invoke<SystemServiceItem[]>('getAllServices');
      this.logger.logInfo('service', 'SystemService', 'getAllServices', 'All services retrieved', {
        count: result.length,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'SystemService',
        'getAllServices',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async stopService(service: string): Promise<string> {
    this.logger.logInfo('service', 'SystemService', 'stopService', 'Stopping service', { service });
    try {
      const result = await this.api.invoke<string>('stopService', { service });
      this.logger.logInfo('service', 'SystemService', 'stopService', 'Service stopped', {
        service,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'SystemService',
        'stopService',
        'Operation failed',
        error as Error,
        { service }
      );
      throw error;
    }
  }

  async stopSelectedServices(services: string[]): Promise<string> {
    this.logger.logInfo(
      'service',
      'SystemService',
      'stopSelectedServices',
      'Stopping selected services',
      { count: services.length }
    );
    try {
      const result = await this.api.invoke<string>('stopSelectedServices', { services });
      this.logger.logInfo(
        'service',
        'SystemService',
        'stopSelectedServices',
        'Selected services stopped'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'SystemService',
        'stopSelectedServices',
        'Operation failed',
        error as Error,
        { services }
      );
      throw error;
    }
  }

  async startService(service: string): Promise<string> {
    this.logger.logInfo('service', 'SystemService', 'startService', 'Starting service', {
      service,
    });
    try {
      const result = await this.api.invoke<string>('startService', { service });
      this.logger.logInfo('service', 'SystemService', 'startService', 'Service started', {
        service,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'SystemService',
        'startService',
        'Operation failed',
        error as Error,
        { service }
      );
      throw error;
    }
  }

  async enableService(service: string): Promise<string> {
    this.logger.logInfo('service', 'SystemService', 'enableService', 'Enabling service', {
      service,
    });
    try {
      const result = await this.api.invoke<string>('enableService', { service });
      this.logger.logInfo('service', 'SystemService', 'enableService', 'Service enabled', {
        service,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'SystemService',
        'enableService',
        'Operation failed',
        error as Error,
        { service }
      );
      throw error;
    }
  }

  async enableSelectedServices(services: string[]): Promise<string> {
    this.logger.logInfo(
      'service',
      'SystemService',
      'enableSelectedServices',
      'Enabling selected services',
      { count: services.length }
    );
    try {
      const result = await this.api.invoke<string>('enableSelectedServices', { services });
      this.logger.logInfo(
        'service',
        'SystemService',
        'enableSelectedServices',
        'Selected services enabled'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'SystemService',
        'enableSelectedServices',
        'Operation failed',
        error as Error,
        { services }
      );
      throw error;
    }
  }

  async getProcesses(): Promise<ProcessItem[]> {
    this.logger.logInfo('service', 'SystemService', 'getProcesses', 'Getting processes');
    try {
      const result = await this.api.invoke<ProcessItem[]>('getProcesses');
      this.logger.logInfo('service', 'SystemService', 'getProcesses', 'Processes retrieved', {
        count: result.length,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'SystemService',
        'getProcesses',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async killProcess(pid: number): Promise<string> {
    this.logger.logInfo('service', 'SystemService', 'killProcess', 'Killing process', { pid });
    try {
      const result = await this.api.invoke<string>('killProcess', { pid });
      this.logger.logInfo('service', 'SystemService', 'killProcess', 'Process killed', { pid });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'SystemService',
        'killProcess',
        'Operation failed',
        error as Error,
        { pid }
      );
      throw error;
    }
  }

  async killSelectedProcesses(pids: number[]): Promise<string> {
    this.logger.logInfo(
      'service',
      'SystemService',
      'killSelectedProcesses',
      'Killing selected processes',
      { count: pids.length }
    );
    try {
      const result = await this.api.invoke<string>('killSelectedProcesses', { pids });
      this.logger.logInfo(
        'service',
        'SystemService',
        'killSelectedProcesses',
        'Selected processes killed'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'SystemService',
        'killSelectedProcesses',
        'Operation failed',
        error as Error,
        { pids }
      );
      throw error;
    }
  }
}
