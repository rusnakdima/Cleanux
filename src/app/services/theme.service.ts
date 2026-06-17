import { Injectable, signal, effect } from '@angular/core';
import {
  AppearanceSettings,
  ThemePreset,
  THEME_PRESETS,
  DEFAULT_APPEARANCE_SETTINGS,
  getAccentShades,
} from '@models/theme.model';
import { LoggerService } from '@services/logger.service';

const STORAGE_KEY = 'appearance_settings';

const ACCENT_COLORS_LIST = [
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

export type ThemeMode = 'light' | 'dark' | 'system';

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

@Injectable({
  providedIn: 'root',
})
export class ThemeService {
  private loggingService = new LoggerService();
  private settings = signal<AppearanceSettings>(this.loadSettings());

  mode = signal<'light' | 'dark' | 'system'>(this.settings().mode);
  preset = signal<ThemePreset>(this.settings().preset);
  accentColors = signal<string[]>([...ACCENT_COLORS_LIST]);

  private htmlEl = document.querySelector('html');

  constructor() {
    effect(() => {
      const mode = this.mode();
      const preset = this.preset();
      const effectiveMode =
        mode === 'system'
          ? window.matchMedia('(prefers-color-scheme: dark)').matches
            ? 'dark'
            : 'light'
          : mode;

      if (this.htmlEl) {
        this.htmlEl.setAttribute('data-theme', effectiveMode);
        this.htmlEl.classList.toggle('dark', effectiveMode === 'dark');
        this.htmlEl.classList.toggle('light', effectiveMode === 'light');
        this.applyAccentColor(preset.accentColor);
      }
    });
  }

  private applyAccentColor(hexColor: string): void {
    if (!this.htmlEl) return;
    const shades = getAccentShades(hexColor);
    this.htmlEl.style.setProperty('--accent-color', shades['500']);
    this.htmlEl.style.setProperty('--accent-50', shades['50']);
    this.htmlEl.style.setProperty('--accent-100', shades['100']);
    this.htmlEl.style.setProperty('--accent-200', shades['200']);
    this.htmlEl.style.setProperty('--accent-300', shades['300']);
    this.htmlEl.style.setProperty('--accent-400', shades['400']);
    this.htmlEl.style.setProperty('--accent-500', shades['500']);
    this.htmlEl.style.setProperty('--accent-600', shades['600']);
    this.htmlEl.style.setProperty('--accent-700', shades['700']);
    this.htmlEl.style.setProperty('--accent-800', shades['800']);
    this.htmlEl.style.setProperty('--accent-900', shades['900']);
  }

  private loadSettings(): AppearanceSettings {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        const parsed = JSON.parse(stored) as AppearanceSettings;
        const preset = THEME_PRESETS.find((p) => p.id === parsed.preset.id) || THEME_PRESETS[0];
        return { ...parsed, preset };
      }
    } catch (error) {
      this.loggingService.warn('Failed to load theme settings: ' + error);
    }

    const legacyTheme = localStorage.getItem('theme');
    if (legacyTheme === 'dark' || legacyTheme === 'light') {
      return {
        mode: legacyTheme,
        preset: THEME_PRESETS[0],
      };
    }

    return DEFAULT_APPEARANCE_SETTINGS;
  }

  getSettings(): AppearanceSettings {
    return {
      mode: this.mode(),
      preset: this.preset(),
    };
  }

  updateMode(mode: 'light' | 'dark' | 'system'): void {
    this.mode.set(mode);
    this.persistSettings();
  }

  updatePreset(preset: ThemePreset): void {
    this.preset.set(preset);
    this.applyAccentColor(preset.accentColor);
    this.persistSettings();
  }

  updateAccentColor(color: string): void {
    const currentPreset = this.preset();
    this.preset.set({ ...currentPreset, accentColor: color });
    this.persistSettings();
  }

  private persistSettings(): void {
    const settings = this.getSettings();
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  }

  getAccentShades() {
    return getAccentShades(this.preset().accentColor);
  }

  toggleMode(): void {
    const current = this.mode();
    if (current === 'light') {
      this.mode.set('dark');
    } else if (current === 'dark') {
      this.mode.set('light');
    } else {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      this.mode.set(prefersDark ? 'light' : 'dark');
    }
    this.persistSettings();
  }

  getEffectiveMode(): 'light' | 'dark' {
    const current = this.mode();
    if (current === 'system') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    return current;
  }

  setMode(mode: 'light' | 'dark' | 'system'): void {
    this.updateMode(mode);
  }

  setPreset(preset: ThemePreset): void {
    this.updatePreset(preset);
  }

  resetToDefaults(): void {
    this.mode.set(DEFAULT_APPEARANCE_SETTINGS.mode);
    this.preset.set(DEFAULT_APPEARANCE_SETTINGS.preset);
    this.persistSettings();
  }

  applyTheme(_config?: ThemeConfig): void {
    const mode = this.mode();
    const preset = this.preset();
    const effectiveMode =
      mode === 'system'
        ? window.matchMedia('(prefers-color-scheme: dark)').matches
          ? 'dark'
          : 'light'
        : mode;

    if (this.htmlEl) {
      this.htmlEl.setAttribute('data-theme', effectiveMode);
      this.htmlEl.classList.toggle('dark', effectiveMode === 'dark');
      this.htmlEl.classList.toggle('light', effectiveMode === 'light');
      this.applyAccentColor(preset.accentColor);
    }
  }

  currentTheme(): ThemeConfig {
    const mode = this.mode();
    const preset = this.preset();
    return {
      mode,
      accentConfig: {
        general: preset.accentColor,
        buttons: preset.accentColor,
        navigation: preset.accentColor,
        borders: preset.accentColor,
        icons: preset.accentColor,
      },
      glassOpacity: 0.7,
    };
  }

  setAccentColor(color: string): void {
    this.updateAccentColor(color);
  }

  setGlassOpacity(_opacity: number): void {}

  getAccentGradient(): string {
    const color = this.preset().accentColor;
    const r = parseInt(color.slice(1, 3), 16);
    const g = parseInt(color.slice(3, 5), 16);
    const b = parseInt(color.slice(5, 7), 16);
    return `linear-gradient(135deg, ${color}, rgba(${r}, ${g}, ${b}, 0.8))`;
  }

  getAccentLightGradient(): string {
    const color = this.preset().accentColor;
    return `linear-gradient(135deg, ${color}33, ${color}22)`;
  }

  getSecondaryAccentGradient(): string {
    return this.getAccentGradient();
  }

  getStatIconGradient(_category: 'default' | 'success' | 'warning' | 'error' = 'default'): string {
    return this.getAccentGradient();
  }
}
