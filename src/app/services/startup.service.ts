import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';
import { StartupItem } from '@models/startup.model';

@Injectable({
  providedIn: 'root',
})
export class StartupService {
  private api = inject(ApiService);

  async getStartupItems(): Promise<StartupItem[]> {
    return await this.api.invoke<StartupItem[]>('get_startup_items');
  }

  async disableStartupItem(path: string): Promise<string> {
    return await this.api.invoke<string>('disable_startup_item', { path });
  }

  async enableStartupItem(path: string): Promise<string> {
    return await this.api.invoke<string>('enable_startup_item', { path });
  }
}
