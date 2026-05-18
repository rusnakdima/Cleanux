/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';

/* models */
import { SystemServiceItem, ProcessItem } from '@models/system.model';

export type { SystemServiceItem, ProcessItem } from '@models/system.model';

@Injectable({
  providedIn: 'root',
})
export class SystemService {
  private api = inject(ApiService);

  async getSystemServices(): Promise<SystemServiceItem[]> {
    return await this.api.invoke<SystemServiceItem[]>('getSystemServices');
  }

  async getAllServices(): Promise<SystemServiceItem[]> {
    return await this.api.invoke<SystemServiceItem[]>('getAllServices');
  }

  async stopService(service: string): Promise<string> {
    return await this.api.invoke<string>('stopService', { service });
  }

  async stopSelectedServices(services: string[]): Promise<string> {
    return await this.api.invoke<string>('stopSelectedServices', { services });
  }

  async startService(service: string): Promise<string> {
    return await this.api.invoke<string>('startService', { service });
  }

  async enableService(service: string): Promise<string> {
    return await this.api.invoke<string>('enableService', { service });
  }

  async enableSelectedServices(services: string[]): Promise<string> {
    return await this.api.invoke<string>('enableSelectedServices', { services });
  }

  async getProcesses(): Promise<ProcessItem[]> {
    return await this.api.invoke<ProcessItem[]>('getProcesses');
  }

  async killProcess(pid: number): Promise<string> {
    return await this.api.invoke<string>('killProcess', { pid });
  }

  async killSelectedProcesses(pids: number[]): Promise<string> {
    return await this.api.invoke<string>('killSelectedProcesses', { pids });
  }
}
