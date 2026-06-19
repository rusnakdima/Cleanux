import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { StartupItem } from '@entities/startup.model';

@Injectable({
  providedIn: 'root',
})
export class StartupService {
  private api = inject(ApiService);

  constructor() {}

  async getStartupItems(): Promise<StartupItem[]> {
    try {
      const result = await this.api.invoke<StartupItem[]>('get_startup_items');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async disableStartupItem(path: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('disable_startup_item', { path });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async enableStartupItem(path: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('enable_startup_item', { path });
      return result;
    } catch (error) {
      throw error;
    }
  }
}
