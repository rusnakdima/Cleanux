import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';
import { TemperatureInfo } from '@models/temperature.model';

export type { TemperatureInfo } from '@models/temperature.model';

@Injectable({
  providedIn: 'root',
})
export class TemperatureService {
  private api = inject(ApiService);

  async getTemperatures(): Promise<TemperatureInfo[]> {
    return await this.api.invoke<TemperatureInfo[]>('get_temperatures');
  }

  async getCpuTemperature(): Promise<TemperatureInfo> {
    return await this.api.invoke<TemperatureInfo>('get_cpu_temperature');
  }

  async getGpuTemperature(): Promise<TemperatureInfo> {
    return await this.api.invoke<TemperatureInfo>('get_gpu_temperature');
  }
}
