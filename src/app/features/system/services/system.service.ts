/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';

/* models */
import { SystemServiceItem, ProcessItem } from '@entities/system.model';

export type { SystemServiceItem, ProcessItem } from '@entities/system.model';

@Injectable({
  providedIn: 'root',
})
export class SystemService {
  private api = inject(ApiService);

  constructor() {}

  async getSystemServices(): Promise<SystemServiceItem[]> {
    try {
      const result = await this.api.invoke<SystemServiceItem[]>('getSystemServices');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getAllServices(): Promise<SystemServiceItem[]> {
    try {
      const result = await this.api.invoke<SystemServiceItem[]>('getAllServices');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async stopService(service: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('stopService', { service });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async stopSelectedServices(services: string[]): Promise<string> {
    try {
      const result = await this.api.invoke<string>('stopSelectedServices', { services });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async startService(service: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('startService', { service });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async enableService(service: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('enableService', { service });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async enableSelectedServices(services: string[]): Promise<string> {
    try {
      const result = await this.api.invoke<string>('enableSelectedServices', { services });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getProcesses(): Promise<ProcessItem[]> {
    try {
      const result = await this.api.invoke<ProcessItem[]>('getProcesses');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async killProcess(pid: number): Promise<string> {
    try {
      const result = await this.api.invoke<string>('killProcess', { pid });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async killSelectedProcesses(pids: number[]): Promise<string> {
    try {
      const result = await this.api.invoke<string>('killSelectedProcesses', { pids });
      return result;
    } catch (error) {
      throw error;
    }
  }
}
