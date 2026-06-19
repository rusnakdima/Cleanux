import { Injectable, signal, computed, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { UnifiedStorageService } from '@app/core/services/unified-storage.service';
import { SystemServiceItem, ProcessItem } from '@entities/system.model';

@Injectable({ providedIn: 'root' })
export class SystemStore {
  private api = inject(ApiService);
  private storage = inject(UnifiedStorageService);

  private _services = signal<SystemServiceItem[]>([]);
  private _processes = signal<ProcessItem[]>([]);
  private _loading = signal(false);
  private _error = signal<string | null>(null);

  readonly services = this._services.asReadonly();
  readonly processes = this._processes.asReadonly();
  readonly loading = this._loading.asReadonly();
  readonly error = this._error.asReadonly();

  readonly runningServices = computed(() => this._services().filter((s) => s.status === 'running'));
  readonly stoppedServices = computed(() => this._services().filter((s) => s.status === 'stopped'));

  async loadSystemServices(): Promise<SystemServiceItem[]> {
    this._loading.set(true);
    this._error.set(null);
    try {
      const services = await this.api.invoke<SystemServiceItem[]>('getSystemServices');
      this._services.set(services);
      return services;
    } catch (e) {
      this._error.set(e instanceof Error ? e.message : 'Failed to load system services');
      return [];
    } finally {
      this._loading.set(false);
    }
  }

  async loadAllServices(): Promise<SystemServiceItem[]> {
    this._loading.set(true);
    this._error.set(null);
    try {
      const services = await this.api.invoke<SystemServiceItem[]>('getAllServices');
      this._services.set(services);
      return services;
    } catch (e) {
      this._error.set(e instanceof Error ? e.message : 'Failed to load all services');
      return [];
    } finally {
      this._loading.set(false);
    }
  }

  async stopService(service: string): Promise<string> {
    this._loading.set(true);
    this._error.set(null);
    try {
      return await this.api.invoke<string>('stopService', { service });
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to stop service';
      this._error.set(message);
      throw new Error(message);
    } finally {
      this._loading.set(false);
    }
  }

  async stopSelectedServices(services: string[]): Promise<string> {
    this._loading.set(true);
    this._error.set(null);
    try {
      return await this.api.invoke<string>('stopSelectedServices', { services });
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to stop services';
      this._error.set(message);
      throw new Error(message);
    } finally {
      this._loading.set(false);
    }
  }

  async startService(service: string): Promise<string> {
    this._loading.set(true);
    this._error.set(null);
    try {
      return await this.api.invoke<string>('startService', { service });
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to start service';
      this._error.set(message);
      throw new Error(message);
    } finally {
      this._loading.set(false);
    }
  }

  async enableService(service: string): Promise<string> {
    this._loading.set(true);
    this._error.set(null);
    try {
      return await this.api.invoke<string>('enableService', { service });
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to enable service';
      this._error.set(message);
      throw new Error(message);
    } finally {
      this._loading.set(false);
    }
  }

  async enableSelectedServices(services: string[]): Promise<string> {
    this._loading.set(true);
    this._error.set(null);
    try {
      return await this.api.invoke<string>('enableSelectedServices', { services });
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to enable services';
      this._error.set(message);
      throw new Error(message);
    } finally {
      this._loading.set(false);
    }
  }

  async getProcesses(): Promise<ProcessItem[]> {
    this._loading.set(true);
    this._error.set(null);
    try {
      const processes = await this.api.invoke<ProcessItem[]>('getProcesses');
      this._processes.set(processes);
      return processes;
    } catch (e) {
      this._error.set(e instanceof Error ? e.message : 'Failed to load processes');
      return [];
    } finally {
      this._loading.set(false);
    }
  }

  async killProcess(pid: number): Promise<string> {
    this._loading.set(true);
    this._error.set(null);
    try {
      return await this.api.invoke<string>('killProcess', { pid });
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to kill process';
      this._error.set(message);
      throw new Error(message);
    } finally {
      this._loading.set(false);
    }
  }

  async killSelectedProcesses(pids: number[]): Promise<string> {
    this._loading.set(true);
    this._error.set(null);
    try {
      return await this.api.invoke<string>('killSelectedProcesses', { pids });
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to kill processes';
      this._error.set(message);
      throw new Error(message);
    } finally {
      this._loading.set(false);
    }
  }

  clearError(): void {
    this._error.set(null);
  }
}
