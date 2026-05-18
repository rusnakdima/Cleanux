/* sys lib */
import { Component, OnInit, OnDestroy, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';
import { TemperatureInfo } from '@models/temperature.model';

@Component({
  selector: 'app-temperature-widget',
  standalone: true,
  imports: [CommonModule, MatIconModule],
  templateUrl: './temperature-widget.component.html',
  styleUrls: ['./temperature-widget.component.css'],
})
export class TemperatureWidgetComponent implements OnInit, OnDestroy {
  private pollInterval: any = null;

  cpuTemp = signal<TemperatureInfo | null>(null);
  gpuTemp = signal<TemperatureInfo | null>(null);
  isLoading = signal(true);

  ngOnInit() {
    this.fetchTemperatures();
    this.pollInterval = setInterval(() => {
      this.fetchTemperatures();
    }, 3000);
  }

  ngOnDestroy() {
    if (this.pollInterval) {
      clearInterval(this.pollInterval);
    }
  }

  async fetchTemperatures() {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const response = await invoke<any>('get_temperatures');
      if (response.status === 'success' && Array.isArray(response.data)) {
        const cpus = response.data.filter((t: TemperatureInfo) => t.sensor_type === 'cpu');
        const gpus = response.data.filter((t: TemperatureInfo) => t.sensor_type === 'gpu');
        this.cpuTemp.set(cpus.length > 0 ? cpus[0] : null);
        this.gpuTemp.set(gpus.length > 0 ? gpus[0] : null);
      }
      this.isLoading.set(false);
    } catch (error) {
      console.error('Failed to fetch temperatures:', error);
      this.isLoading.set(false);
    }
  }

  getTemperature(temp: TemperatureInfo | null): number {
    return temp?.temperature_celsius ?? 0;
  }

  getTemperatureColor(temp: number): string {
    if (temp >= 80) return '#ef4444';
    if (temp >= 60) return '#f59e0b';
    return '#22c55e';
  }

  getProgressWidth(temp: number): number {
    return Math.min((temp / 100) * 100, 100);
  }
}
