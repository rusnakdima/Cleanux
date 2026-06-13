/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  signal,
  inject,
  OnDestroy,
  effect,
} from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';

/* forms */
import { FormsModule } from '@angular/forms';

/* env */
import { environment } from '@env/environment';

/* services */
import { AboutService } from '@services/about.service';
import { LoggerService } from '@services/logger.service';
import { SchedulerService } from '@services/scheduler.service';
import { ThemeService, ThemeMode } from '@services/theme.service';
import { NotificationService } from '@services/notification.service';
import { ToastService } from '@shared/toast';

/* components */
import { ToggleComponent } from '@components/toggle/toggle.component';

/* models */
import { GitHubReleaseByTag, GitHubReleaseLatest } from '@models/github-release.model';
import { ScheduleConfig, defaultScheduleConfig } from '@models/schedule.model';

@Component({
  selector: 'app-settings',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, FormsModule, ToggleComponent],
  templateUrl: './settings.view.html',
})
export class SettingsView implements OnDestroy {
  themeService = inject(ThemeService);
  private notification = inject(NotificationService);
  private logger = inject(LoggerService);
  private toastService = inject(ToastService);
  private aboutService = inject(AboutService);
  private schedulerService = inject(SchedulerService);

  private dateVersionEffect = effect(() => {
    const release = this.aboutService.getDate(this.version);
    const publishedAt = release()?.published_at;
    if (publishedAt) {
      localStorage.setItem('dateVersion', String(this.formatDate(publishedAt)));
      this.dateVersion.set(String(this.formatDate(publishedAt)));
    }
  });

  private checkUpdateEffect = effect(() => {
    const release = this.aboutService.checkUpdate();
    const tagName = release()?.tag_name;
    if (tagName) {
      const ver: string = tagName;
      setTimeout(() => {
        if (this.matchVersion(ver)) {
          this.isUpdateAvailable.set(true);
          this.lastVersion.set(ver);
          this.toastService.show(`A new version ${ver} is available!`, 'info');
        } else {
          this.toastService.show('You have the latest version!', 'info');
        }
        this.isChecking.set(false);
      }, 1000);
    } else {
      this.isChecking.set(false);
    }
  });

  constructor() {
    this.loadSchedule();
  }

  ngOnDestroy(): void {
    this.dateVersionEffect.destroy();
    this.checkUpdateEffect.destroy();
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

  onIntervalChange(): void {}

  onCleaningTypeChange(): void {}

  async saveSchedule() {
    this.isSaving.set(true);
    try {
      await this.schedulerService.saveScheduleConfig(this.scheduleConfig());
      this.notification.success('Schedule saved successfully');
    } catch (e) {
      this.notification.error('Failed to save schedule', e);
    } finally {
      this.isSaving.set(false);
    }
  }

  async runNow() {
    this.isRunning.set(true);
    try {
      await this.schedulerService.runCleaningNow(this.scheduleConfig().cleaning_type);
      this.notification.success('Cleaning completed');
      this.loadSchedule();
    } catch (e) {
      this.notification.error('Failed to run cleaning', e);
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

  getDate() {}

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
  }

  get loggingEnabled() {
    return this.logger.isGlobalEnabled();
  }
  get loggingLevel() {
    return this.logger.getMinLevel();
  }
  get levelConfig(): Record<string, boolean> {
    return this.logger.getLevelsConfig();
  }
  get sourceConfig(): Record<string, boolean> {
    return this.logger.getSourcesConfig();
  }
  get logStats() {
    return this.logger.getLogStats();
  }

  clearLogs(): void {
    this.logger.clearLogs();
  }

  accentCategories = [
    { key: 'general', label: 'General' },
    { key: 'icons', label: 'Icons' },
    { key: 'headers', label: 'Headers' },
    { key: 'borders', label: 'Borders' },
  ];

  resetAllAccents(): void {
    this.themeService.setAccentColor(this.themeService.currentTheme().accentConfig.general);
    for (const cat of this.accentCategories) {
      this.setAccentForCategory(cat.key, this.themeService.currentTheme().accentConfig.general);
    }
  }

  setAccentForCategory(category: string, color: string): void {
    this.themeService.setAccentColor(color);
  }

  toggleSchedule(): void {
    this.scheduleConfig.update((c) => ({ ...c, enabled: !c.enabled }));
  }
}
