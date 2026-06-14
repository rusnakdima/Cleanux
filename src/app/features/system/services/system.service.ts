/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';

/* models */
import { SystemServiceItem, ProcessItem } from '@models/system.model';

export type { SystemServiceItem, ProcessItem } from '@models/system.model';

@Injectable({
  providedIn: 'root',
})
export class SystemService {
  private api = inject(ApiService);
  private loggingService = getLoggingService();

  constructor() {
    this.loggingService.info('SystemService initialized');
  }

  async getSystemServices(): Promise<SystemServiceItem[]> {
    this.loggingService.info('Getting system services');
    try {
      const result = await this.api.invoke<SystemServiceItem[]>('getSystemServices');
      this.loggingService.info('System services retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getAllServices(): Promise<SystemServiceItem[]> {
    this.loggingService.info('Getting all services');
    try {
      const result = await this.api.invoke<SystemServiceItem[]>('getAllServices');
      this.loggingService.info('All services retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async stopService(service: string): Promise<string> {
    this.loggingService.info('Stopping service', { service });
    try {
      const result = await this.api.invoke<string>('stopService', { service });
      this.loggingService.info('Service stopped', { service });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { service });
      throw error;
    }
  }

  async stopSelectedServices(services: string[]): Promise<string> {
    this.loggingService.info('Stopping selected services', { count: services.length });
    try {
      const result = await this.api.invoke<string>('stopSelectedServices', { services });
      this.loggingService.info('Selected services stopped');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { services });
      throw error;
    }
  }

  async startService(service: string): Promise<string> {
    this.loggingService.info('Starting service', { service });
    try {
      const result = await this.api.invoke<string>('startService', { service });
      this.loggingService.info('Service started', { service });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { service });
      throw error;
    }
  }

  async enableService(service: string): Promise<string> {
    this.loggingService.info('Enabling service', { service });
    try {
      const result = await this.api.invoke<string>('enableService', { service });
      this.loggingService.info('Service enabled', { service });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { service });
      throw error;
    }
  }

  async enableSelectedServices(services: string[]): Promise<string> {
    this.loggingService.info('Enabling selected services', { count: services.length });
    try {
      const result = await this.api.invoke<string>('enableSelectedServices', { services });
      this.loggingService.info('Selected services enabled');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { services });
      throw error;
    }
  }

  async getProcesses(): Promise<ProcessItem[]> {
    this.loggingService.info('Getting processes');
    try {
      const result = await this.api.invoke<ProcessItem[]>('getProcesses');
      this.loggingService.info('Processes retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async killProcess(pid: number): Promise<string> {
    this.loggingService.info('Killing process', { pid });
    try {
      const result = await this.api.invoke<string>('killProcess', { pid });
      this.loggingService.info('Process killed', { pid });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { pid });
      throw error;
    }
  }

  async killSelectedProcesses(pids: number[]): Promise<string> {
    this.loggingService.info('Killing selected processes', { count: pids.length });
    try {
      const result = await this.api.invoke<string>('killSelectedProcesses', { pids });
      this.loggingService.info('Selected processes killed');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { pids });
      throw error;
    }
  }
}
