/* sys lib */
import { ChangeDetectionStrategy, Component, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';

/* forms */
import { FormsModule } from '@angular/forms';

/* env */
import { environment } from '@env/environment';

/* services */
import { AboutService } from '@services/about.service';
import { SchedulerService } from '@services/scheduler.service';
import { ThemeService, ThemeMode } from '@services/theme.service';

/* models */
import { GitHubReleaseByTag, GitHubReleaseLatest } from '@models/github-release.model';
import { ScheduleConfig, defaultScheduleConfig } from '@models/schedule.model';

@Component({
  selector: 'app-settings',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, FormsModule],
  templateUrl: './settings.view.html',
})
export class SettingsView {
  themeService = inject(ThemeService);

  constructor(
    private aboutService: AboutService,
    private schedulerService: SchedulerService
  ) {
    this.loadSchedule();
  }

  themeModes: ThemeMode[] = ['dark', 'light', 'system'];

  deepScan = signal(false);
  autoClean = signal(false);

  scheduleConfig = signal<ScheduleConfig>({ ...defaultScheduleConfig });
  isSaving = signal(false);
  isRunning = signal(false);

  version = environment.version;
  nameProduct = environment.nameProduct;
  yearCreate = environment.yearCreate;
  companyName = environment.companyName;

  dateVersion = signal(localStorage.getItem('dateVersion') || 'Unknown');
  dateCheck = signal(localStorage.getItem('dateCheck') || 'Unknown');

  isChecking = signal(false);
  isUpdateAvailable = signal(false);
  lastVersion = signal('');

  async loadSchedule() {
    const config = await this.schedulerService.getScheduleConfig();
    if (config) {
      this.scheduleConfig.set(config);
    }
  }

  toggleDeepScan() {
    this.deepScan.update((v) => !v);
  }

  toggleAutoClean() {
    this.autoClean.update((v) => !v);
  }

  toggleSchedule() {
    this.scheduleConfig.update((c) => ({ ...c, enabled: !c.enabled }));
  }

  onIntervalChange() {
    // Signal update handled by ngModel
  }

  onCleaningTypeChange() {
    // Signal update handled by ngModel
  }

  async saveSchedule() {
    this.isSaving.set(true);
    try {
      await this.schedulerService.saveScheduleConfig(this.scheduleConfig());
      alert('Schedule saved successfully');
    } catch (e) {
      alert('Failed to save schedule');
    } finally {
      this.isSaving.set(false);
    }
  }

  async runNow() {
    this.isRunning.set(true);
    try {
      await this.schedulerService.runCleaningNow(this.scheduleConfig().cleaning_type);
      alert('Cleaning completed');
      this.loadSchedule();
    } catch (e) {
      alert('Failed to run cleaning');
    } finally {
      this.isRunning.set(false);
    }
  }

  formatDate(date: string): string {
    return new Date(date).toISOString().split('T')[0];
  }

  formatDateTime(date: string | null): string {
    if (!date) return 'Never';
    return new Date(date).toLocaleString();
  }

  matchVersion(lastVer: string): boolean {
    const v1Components = lastVer.replace('v', '').split('.').map(Number);
    const v2Components = this.version.split('.').map(Number);

    for (let i = 0; i < Math.max(v1Components.length, v2Components.length); i++) {
      const v1Value = v1Components[i] || 0;
      const v2Value = v2Components[i] || 0;

      if (v1Value < v2Value) {
        return false;
      } else if (v1Value > v2Value) {
        return true;
      }
    }

    return false;
  }

  getDate() {
    this.aboutService.getDate(this.version).subscribe({
      next: (res: GitHubReleaseByTag) => {
        if (res && res.published_at) {
          localStorage.setItem('dateVersion', String(this.formatDate(res.published_at)));
          this.dateVersion.set(String(this.formatDate(res.published_at)));
        }
      },
      error: () => {},
    });
  }

  setThemeMode(mode: ThemeMode) {
    this.themeService.setMode(mode);
  }

  setAccentColor(color: string) {
    this.themeService.setAccentColor(color);
  }

  setGlassOpacity(opacity: number) {
    this.themeService.setGlassOpacity(opacity);
  }

  checkUpdate() {
    this.isChecking.set(true);
    localStorage.setItem('dateCheck', String(this.formatDate(new Date().toUTCString())));
    this.dateCheck.set(localStorage.getItem('dateCheck') || '');

    this.aboutService.checkUpdate().subscribe({
      next: (res: GitHubReleaseLatest) => {
        if (res && res.tag_name) {
          const ver: string = res.tag_name;
          setTimeout(() => {
            if (this.matchVersion(ver)) {
              this.isUpdateAvailable.set(true);
              this.lastVersion.set(ver);
              alert(`A new version ${ver} is available!`);
            } else {
              alert('You have the latest version!');
            }
            this.isChecking.set(false);
          }, 1000);
        } else {
          this.isChecking.set(false);
        }
      },
      error: () => {
        this.isChecking.set(false);
        alert('Failed to check for updates');
      },
    });
  }
}
