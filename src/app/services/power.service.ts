import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';
import { BatteryInfo, PowerProfile, ThermalInfo } from '@models/power.model';

export type { BatteryInfo, PowerProfile, ThermalInfo } from '@models/power.model';

@Injectable({
  providedIn: 'root',
})
export class PowerService {
  private api = inject(ApiService);

  async getBatteryInfo(): Promise<BatteryInfo | null> {
    return await this.api.invoke<BatteryInfo | null>('get_battery_info');
  }

  async getPowerProfiles(): Promise<PowerProfile[]> {
    return await this.api.invoke<PowerProfile[]>('get_power_profiles');
  }

  async setPowerProfile(profile: string): Promise<boolean> {
    return await this.api.invoke<boolean>('set_power_profile', { profile });
  }

  async getThermalInfo(): Promise<ThermalInfo[]> {
    return await this.api.invoke<ThermalInfo[]>('get_thermal_info');
  }
}
