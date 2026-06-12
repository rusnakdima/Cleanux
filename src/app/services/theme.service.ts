import { ChangeDetectionStrategy, Injectable, signal, effect, inject } from '@angular/core';
import { LoggerService } from '@services/logger.service';

export type ThemeMode = 'dark' | 'light' | 'system';

export type AccentCategory = 'general' | 'buttons' | 'navigation' | 'borders' | 'icons';

export interface AccentConfig {
  general: string;
  buttons: string;
  navigation: string;
  borders: string;
  icons: string;
}

export interface ThemeConfig {
  mode: ThemeMode;
  accentConfig: AccentConfig;
  glassOpacity: number;
}

const DEFAULT_ACCENT_CONFIG: AccentConfig = {
  general: '#6366f1',
  buttons: '#6366f1',
  navigation: '#6366f1',
  borders: '#6366f1',
  icons: '#6366f1',
};

const DEFAULT_THEME: ThemeConfig = {
  mode: 'dark',
  accentConfig: { ...DEFAULT_ACCENT_CONFIG },
  glassOpacity: 0.7,
};

const ACCENT_COLORS = [
  '#6366f1',
  '#8b5cf6',
  '#ec4899',
  '#ef4444',
  '#f97316',
  '#eab308',
  '#22c55e',
  '#14b8a6',
  '#06b6d4',
  '#3b82f6',
];

@Injectable({ providedIn: 'root' })
export class ThemeService {
  private readonly STORAGE_KEY = 'cleanux_theme';
  private logger = inject(LoggerService);

  currentTheme = signal<ThemeConfig>(this.loadTheme());
  accentColors = signal<string[]>(ACCENT_COLORS);

  constructor() {
    this.logger.logInfo('service', 'ThemeService', 'init', 'ThemeService initialized');
    effect(() => {
      this.applyTheme(this.currentTheme());
    });
  }

  private loadTheme(): ThemeConfig {
    try {
      const stored = localStorage.getItem(this.STORAGE_KEY);
      if (stored) {
        const parsed = JSON.parse(stored);
        return {
          ...DEFAULT_THEME,
          ...parsed,
          accentConfig: { ...DEFAULT_ACCENT_CONFIG, ...parsed.accentConfig },
        };
      }
    } catch {}
    return { ...DEFAULT_THEME };
  }

  private saveTheme(config: ThemeConfig) {
    localStorage.setItem(this.STORAGE_KEY, JSON.stringify(config));
  }

  setMode(mode: ThemeMode) {
    this.logger.logDebug('service', 'ThemeService', 'setMode', 'Setting theme mode', { mode });
    this.currentTheme.update((t) => ({ ...t, mode }));
    this.saveTheme(this.currentTheme());
  }

  setAccentColor(color: string) {
    this.logger.logDebug('service', 'ThemeService', 'setAccentColor', 'Setting accent color', {
      color,
    });
    this.currentTheme.update((t) => ({
      ...t,
      accentConfig: {
        ...t.accentConfig,
        general: color,
        buttons: color,
        navigation: color,
        borders: color,
        icons: color,
      },
    }));
    this.saveTheme(this.currentTheme());
  }

  setAccentForCategory(category: AccentCategory, color: string) {
    this.logger.logDebug(
      'service',
      'ThemeService',
      'setAccentForCategory',
      'Setting accent for category',
      { category, color }
    );
    this.currentTheme.update((t) => ({
      ...t,
      accentConfig: { ...t.accentConfig, [category]: color },
    }));
    this.saveTheme(this.currentTheme());
  }

  resetAccentToDefault(category: AccentCategory) {
    this.logger.logDebug(
      'service',
      'ThemeService',
      'resetAccentToDefault',
      'Resetting accent to default',
      { category }
    );
    this.setAccentForCategory(category, DEFAULT_ACCENT_CONFIG[category]);
  }

  resetAllAccents() {
    this.logger.logDebug('service', 'ThemeService', 'resetAllAccents', 'Resetting all accents');
    this.currentTheme.update((t) => ({
      ...t,
      accentConfig: { ...DEFAULT_ACCENT_CONFIG },
    }));
    this.saveTheme(this.currentTheme());
  }

  setGlassOpacity(opacity: number) {
    this.logger.logDebug('service', 'ThemeService', 'setGlassOpacity', 'Setting glass opacity', {
      opacity,
    });
    this.currentTheme.update((t) => ({ ...t, glassOpacity: Math.max(0, Math.min(1, opacity)) }));
    this.saveTheme(this.currentTheme());
  }

  applyTheme(config: ThemeConfig) {
    this.logger.logDebug('service', 'ThemeService', 'applyTheme', 'Applying theme', {
      mode: config.mode,
    });
    const root = document.documentElement;
    const body = document.body;

    const applyAccentVar = (name: string, color: string) => {
      const r = parseInt(color.slice(1, 3), 16);
      const g = parseInt(color.slice(3, 5), 16);
      const b = parseInt(color.slice(5, 7), 16);
      root.style.setProperty(name, color);
      root.style.setProperty(name + '-light', `rgba(${r}, ${g}, ${b}, 0.1)`);
      root.style.setProperty(name + '-glow', `rgba(${r}, ${g}, ${b}, 0.3)`);
      root.style.setProperty(name + '-rgb', `${r}, ${g}, ${b}`);
    };

    applyAccentVar('--accent', config.accentConfig.general);
    applyAccentVar('--accent-buttons', config.accentConfig.buttons);
    applyAccentVar('--accent-nav', config.accentConfig.navigation);
    applyAccentVar('--accent-borders', config.accentConfig.borders);
    applyAccentVar('--accent-icons', config.accentConfig.icons);
    root.style.setProperty('--glass-opacity', config.glassOpacity.toString());
    root.style.setProperty('--icon-cpu', config.accentConfig.icons);
    root.style.setProperty('--icon-memory', config.accentConfig.icons);

    const isDark =
      config.mode === 'dark' ||
      (config.mode === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);

    body.classList.add('theme-transitioning');
    requestAnimationFrame(() => {
      root.classList.toggle('dark', isDark);
      root.classList.toggle('light', !isDark);
      setTimeout(() => {
        body.classList.remove('theme-transitioning');
      }, 300);
    });
  }

  getEffectiveMode(): boolean {
    const config = this.currentTheme();
    return (
      config.mode === 'dark' ||
      (config.mode === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)
    );
  }
}
