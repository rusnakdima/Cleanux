/* sys lib */
import { Injectable } from '@angular/core';

/* services */
import { BaseApiService } from '@services/base-api.service';

/* models */
import { SystemServiceItem, ProcessItem } from '@models/system.model';

export type { SystemServiceItem, ProcessItem } from '@models/system.model';

@Injectable({
  providedIn: 'root',
})
export class SystemService extends BaseApiService {
  async getSystemServices(): Promise<SystemServiceItem[]> {
    return this.call<SystemServiceItem[]>('getSystemServices');
  }

  async getAllServices(): Promise<SystemServiceItem[]> {
    return this.call<SystemServiceItem[]>('getAllServices');
  }

  async stopService(service: string): Promise<string> {
    return this.call<string>('stopService', { service });
  }

  async stopSelectedServices(services: string[]): Promise<string> {
    return this.call<string>('stopSelectedServices', { services });
  }

  async startService(service: string): Promise<string> {
    return this.call<string>('startService', { service });
  }

  async enableService(service: string): Promise<string> {
    return this.call<string>('enableService', { service });
  }

  async enableSelectedServices(services: string[]): Promise<string> {
    return this.call<string>('enableSelectedServices', { services });
  }

  async getProcesses(): Promise<ProcessItem[]> {
    return this.call<ProcessItem[]>('getProcesses');
  }

  async killProcess(pid: number): Promise<string> {
    return this.call<string>('killProcess', { pid });
  }

  async killSelectedProcesses(pids: number[]): Promise<string> {
    return this.call<string>('killSelectedProcesses', { pids });
  }
}
