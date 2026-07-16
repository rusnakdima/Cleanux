/* angular */
import { Injectable, inject } from '@angular/core';

/* library */
import { InvokeWrapperService } from '@tauri-front/shared';

export interface SystemServiceInfo {
  name: string;
  description: string;
  load: string;
  active: string;
  status: string;
  isRunning: boolean;
}

export interface ProcessInfo {
  pid: number;
  name: string;
  cpu_usage: number;
  memory_usage: number;
}

@Injectable({ providedIn: 'root' })
export class SystemService {
  private api = inject(InvokeWrapperService);

  async getSystemServices(): Promise<SystemServiceInfo[]> {
    return this.api.invoke<SystemServiceInfo[]>('getSystemServices');
  }

  async getAllServices(): Promise<SystemServiceInfo[]> {
    return this.api.invoke<SystemServiceInfo[]>('getAllServices');
  }

  async stopService(service: string): Promise<string> {
    return this.api.invoke<string>('stopService', { service });
  }

  async stopSelectedServices(services: string[]): Promise<string> {
    return this.api.invoke<string>('stopSelectedServices', { services });
  }

  async startService(service: string): Promise<string> {
    return this.api.invoke<string>('startService', { service });
  }

  async enableService(service: string): Promise<string> {
    return this.api.invoke<string>('enableService', { service });
  }

  async enableSelectedServices(services: string[]): Promise<string> {
    return this.api.invoke<string>('enableSelectedServices', { services });
  }

  async getProcesses(): Promise<ProcessInfo[]> {
    return this.api.invoke<ProcessInfo[]>('getProcesses');
  }

  async killProcess(pid: number): Promise<string> {
    return this.api.invoke<string>('killProcess', { pid });
  }

  async killSelectedProcesses(pids: number[]): Promise<string> {
    return this.api.invoke<string>('killSelectedProcesses', { pids });
  }
}
