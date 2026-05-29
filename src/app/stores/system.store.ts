import { Injectable, signal, inject } from '@angular/core';
import { TauriApiService } from '@api/tauri-api.service';
import { SystemServiceItem, ProcessItem } from '@models/system.model';

@Injectable()
export class SystemStore {
  private api = inject(TauriApiService);

  readonly services = signal<SystemServiceItem[]>([]);
  readonly processes = signal<ProcessItem[]>([]);
  readonly loading = signal(false);
  readonly selectedServices = signal<Set<string>>(new Set());
  readonly selectedProcesses = signal<Set<number>>(new Set());

  async loadSystemServices(): Promise<SystemServiceItem[]> {
    return await this.api.invoke<SystemServiceItem[]>('getSystemServices');
  }

  async loadAllServices(): Promise<SystemServiceItem[]> {
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
