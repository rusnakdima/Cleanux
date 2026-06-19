import { Injectable } from '@angular/core';
import { BaseApiService } from '@services/base-api.service';
import { BatteryInfo, PowerProfile, ThermalInfo } from '@entities/power.model';

@Injectable({
  providedIn: 'root',
})
export class PowerService extends BaseApiService {
  async getBatteryInfo(): Promise<BatteryInfo | null> {
    return await this.call<BatteryInfo | null>('get_battery_info');
  }

  async getPowerProfiles(): Promise<PowerProfile[]> {
    return await this.call<PowerProfile[]>('get_power_profiles');
  }

  async setPowerProfile(profile: string): Promise<boolean> {
    return await this.call<boolean>('set_power_profile', { profile });
  }

  async getThermalInfo(): Promise<ThermalInfo[]> {
    return await this.call<ThermalInfo[]>('get_thermal_info');
  }
}
