/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  OnInit,
  signal,
  inject,
  computed,
} from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatSelectModule } from '@angular/material/select';
import { MatFormFieldModule } from '@angular/material/form-field';

/* services */
import { PowerService, BatteryInfo, PowerProfile, ThermalInfo } from '@services/power.service';

/* components */
import { HeaderComponent } from '@components/header/header.component';

@Component({
  selector: 'app-power-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatSelectModule,
    MatFormFieldModule,
    HeaderComponent,
  ],
  templateUrl: './power.view.html',
})
export class PowerView implements OnInit {
  private powerService = inject(PowerService);

  batteryInfo = signal<BatteryInfo | null>(null);
  powerProfiles = signal<PowerProfile[]>([]);
  thermalInfo = signal<ThermalInfo[]>([]);
  loading = signal(false);
  selectedProfile = signal<string>('');

  hasBattery = computed(() => this.batteryInfo() !== null);

  getBatteryStatusColor(): string {
    const info = this.batteryInfo();
    if (!info) return 'text-slate-500';

    if (info.status === 'Charging') return 'text-green-500';
    if (info.charge_percent < 20) return 'text-red-500';
    if (info.charge_percent < 50) return 'text-yellow-500';
    return 'text-blue-500';
  }

  getHealthColor(health: number): string {
    if (health >= 80) return 'text-green-500';
    if (health >= 50) return 'text-yellow-500';
    return 'text-red-500';
  }

  getTemperatureColor(temp: number): string {
    if (temp < 50) return 'text-green-500';
    if (temp < 70) return 'text-yellow-500';
    return 'text-red-500';
  }

  getTemperatureBgColor(temp: number): string {
    if (temp < 50) return 'bg-green-100 dark:bg-green-900/30';
    if (temp < 70) return 'bg-yellow-100 dark:bg-yellow-900/30';
    return 'bg-red-100 dark:bg-red-900/30';
  }

  async ngOnInit() {
    await this.loadData();
  }

  async loadData() {
    this.loading.set(true);
    try {
      const [battery, profiles, thermal] = await Promise.all([
        this.powerService.getBatteryInfo(),
        this.powerService.getPowerProfiles(),
        this.powerService.getThermalInfo(),
      ]);

      this.batteryInfo.set(battery);

      const activeProfile = profiles.find((p) => p.active);
      if (activeProfile) {
        this.selectedProfile.set(activeProfile.name);
      }
      this.powerProfiles.set(profiles);
      this.thermalInfo.set(thermal);
    } catch (error: unknown) {
      console.error('Failed to load power data:', error);
    } finally {
      this.loading.set(false);
    }
  }

  async selectProfile(profile: string) {
    try {
      await this.powerService.setPowerProfile(profile);
      this.selectedProfile.set(profile);
      await this.loadData();
    } catch (error: unknown) {
      console.error('Failed to set power profile:', error);
      alert('Failed to set power profile');
    }
  }

  formatStatus(status: string): string {
    return status.charAt(0).toUpperCase() + status.slice(1).toLowerCase();
  }
}
