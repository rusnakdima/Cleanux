/* sys lib */
import { ChangeDetectionStrategy, Component, signal, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';

/* router */
import { RouterLink } from '@angular/router';

/* services */
import { MonitorStore } from '@stores/monitor.store';
import { PowerService } from '@services/power.service';
import { BatteryInfo, PowerProfile, ThermalInfo } from '@models/power.model';

@Component({
  selector: 'app-power-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, RouterLink],
  templateUrl: './power.view.html',
})
export class PowerView implements OnInit {
  protected monitorStore = inject(MonitorStore);
  private powerService = inject(PowerService);

  batteryInfo = signal<BatteryInfo | null>(null);
  powerProfiles = signal<PowerProfile[]>([]);
  thermalInfo = signal<ThermalInfo[]>([]);
  activeProfile = signal<string>('balanced');

  powerTools = [
    { id: 'battery', label: 'Battery Manager', desc: 'Monitor and optimize battery usage', icon: 'battery_saver', route: '/power' },
    { id: 'thermal', label: 'Thermal Monitor', desc: 'Track CPU and system temperatures', icon: 'thermostat', route: '/power' },
    { id: 'profiles', label: 'Power Profiles', desc: 'Switch between power modes', icon: 'tune', route: '/power' },
  ];

  ngOnInit() {
    this.loadPowerData();
  }

  async loadPowerData() {
    try {
      const [battery, profiles, thermal] = await Promise.all([
        this.powerService.getBatteryInfo(),
        this.powerService.getPowerProfiles(),
        this.powerService.getThermalInfo(),
      ]);

      this.batteryInfo.set(battery);
      this.powerProfiles.set(profiles);

      const active = profiles.find((p) => p.active);
      if (active) {
        this.activeProfile.set(active.name);
      }

      this.thermalInfo.set(thermal);
    } catch (e) {
      console.error('Failed to load power data:', e);
    }
  }

  async setProfile(profile: string) {
    try {
      await this.powerService.setPowerProfile(profile);
      this.activeProfile.set(profile);
      const updated = this.powerProfiles().map((p) => ({
        ...p,
        active: p.name === profile,
      }));
      this.powerProfiles.set(updated);
    } catch (e) {
      console.error('Failed to set power profile:', e);
    }
  }

  getProfileIcon(profileName: string): string {
    const icons: Record<string, string> = {
      'power-saver': 'battery_saver',
      'balanced': 'dashboard',
      'performance': 'bolt',
    };
    return icons[profileName] || 'settings';
  }

  getProfileLabel(profileName: string): string {
    return profileName.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ');
  }

  getBatteryIcon(): string {
    const battery = this.batteryInfo();
    if (!battery || !battery.present) return 'battery_unknown';
    if (battery.charge_percent <= 20) return 'battery_alert';
    if (battery.charge_percent <= 50) return 'battery_3_bar';
    if (battery.charge_percent <= 80) return 'battery_5_bar';
    return 'battery_full';
  }

  getBatteryStatus(): string {
    const battery = this.batteryInfo();
    if (!battery || !battery.present) return 'No Battery';
    return `${battery.charge_percent}% - ${battery.status}`;
  }

  getThermalStatus(): string {
    const thermals = this.thermalInfo();
    if (thermals.length === 0) return 'N/A';
    const maxTemp = Math.max(...thermals.map((t) => t.temperature_celsius));
    return `${maxTemp.toFixed(0)}°C`;
  }
}
