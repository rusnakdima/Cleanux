import { ChangeDetectionStrategy, Injectable, signal, effect, inject } from '@angular/core';
import { LoggerService } from './logger.service';
import {
  ACCENT_COLORS,
  DEFAULT_ACCENT_COLOR,
  GLASS_OPACITY_DEFAULT,
} from '@shared/constants/theme.constants';
import { THEME_TRANSITION_TIMEOUT_MS } from '@shared/constants/timeout.constants';

export type ThemeMode = 'dark' | 'light' | 'system';

export type AccentCategory = 'general' | 'buttons' | 'navigation' | 'borders' | 'icons';

export interface AccentConfig {
  [key: string]: string;
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
  general: DEFAULT_ACCENT_COLOR,
  buttons: DEFAULT_ACCENT_COLOR,
  navigation: DEFAULT_ACCENT_COLOR,
  borders: DEFAULT_ACCENT_COLOR,
  icons: DEFAULT_ACCENT_COLOR,
};

const DEFAULT_THEME: ThemeConfig = {
  mode: 'dark',
  accentConfig: { ...DEFAULT_ACCENT_CONFIG },
  glassOpacity: GLASS_OPACITY_DEFAULT,
};

@Injectable({ providedIn: 'root' })
export class ThemeService {
  private readonly STORAGE_KEY = 'cleanux_theme';
  private loggingService = new LoggerService();

  currentTheme = signal<ThemeConfig>(this.loadTheme());
  accentColors = signal<string[]>([...ACCENT_COLORS]);

  constructor() {
    this.loggingService.info('ThemeService initialized');
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
    this.loggingService.debug('Setting theme mode', { mode });
    this.currentTheme.update((t) => ({ ...t, mode }));
    this.saveTheme(this.currentTheme());
  }

  setAccentColor(color: string) {
    this.loggingService.debug('Setting accent color', { color });
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
    this.loggingService.debug('Setting accent for category', { category, color });
    this.currentTheme.update((t) => ({
      ...t,
      accentConfig: { ...t.accentConfig, [category]: color },
    }));
    this.saveTheme(this.currentTheme());
  }

  resetAccentToDefault(category: AccentCategory) {
    this.loggingService.debug('Resetting accent to default', { category });
    this.setAccentForCategory(category, DEFAULT_ACCENT_CONFIG[category]);
  }

  resetAllAccents() {
    this.loggingService.debug('Resetting all accents');
    this.currentTheme.update((t) => ({
      ...t,
      accentConfig: { ...DEFAULT_ACCENT_CONFIG },
    }));
    this.saveTheme(this.currentTheme());
  }

  setGlassOpacity(opacity: number) {
    this.loggingService.debug('Setting glass opacity', { opacity });
    this.currentTheme.update((t) => ({ ...t, glassOpacity: Math.max(0, Math.min(1, opacity)) }));
    this.saveTheme(this.currentTheme());
  }

  applyTheme(config: ThemeConfig) {
    this.loggingService.debug('Applying theme', { mode: config.mode });
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
      }, THEME_TRANSITION_TIMEOUT_MS);
    });
  }

  getEffectiveMode(): boolean {
    const config = this.currentTheme();
    return (
      config.mode === 'dark' ||
      (config.mode === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)
    );
  }

  getAccentColor(): string {
    return this.currentTheme().accentConfig.general;
  }

  getAccentGradient(): string {
    const color = this.getAccentColor();
    const r = parseInt(color.slice(1, 3), 16);
    const g = parseInt(color.slice(3, 5), 16);
    const b = parseInt(color.slice(5, 7), 16);
    return `linear-gradient(135deg, ${color}, rgba(${r}, ${g}, ${b}, 0.8))`;
  }

  getAccentLightGradient(): string {
    const color = this.getAccentColor();
    return `linear-gradient(135deg, ${color}33, ${color}22)`;
  }

  getSecondaryAccentGradient(): string {
    const secondary = this.currentTheme().accentConfig.buttons;
    const color = this.currentTheme().accentConfig.general;
    const r = parseInt(secondary.slice(1, 3), 16);
    const g = parseInt(secondary.slice(3, 5), 16);
    const b = parseInt(secondary.slice(5, 7), 16);
    return `linear-gradient(135deg, ${color}, rgba(${r}, ${g}, ${b}, 0.8))`;
  }

  getStatIconGradient(category: 'default' | 'success' | 'warning' | 'error' = 'default'): string {
    const color = this.getAccentColor();
    const r = parseInt(color.slice(1, 3), 16);
    const g = parseInt(color.slice(3, 5), 16);
    const b = parseInt(color.slice(5, 7), 16);
    return `linear-gradient(135deg, ${color}, rgba(${r}, ${g}, ${b}, 0.8))`;
  }
}
