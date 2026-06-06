import { Injectable } from '@angular/core';
import { BaseApiService } from '@services/base-api.service';
import { StartupItem } from '@models/startup.model';

@Injectable({
  providedIn: 'root',
})
export class StartupService extends BaseApiService {
  async getStartupItems(): Promise<StartupItem[]> {
    return this.call<StartupItem[]>('get_startup_items');
  }

  async disableStartupItem(path: string): Promise<string> {
    return this.call<string>('disable_startup_item', { path });
  }

  async enableStartupItem(path: string): Promise<string> {
    return this.call<string>('enable_startup_item', { path });
  }
}