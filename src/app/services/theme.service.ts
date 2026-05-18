import { ChangeDetectionStrategy, Injectable, signal, effect } from '@angular/core';

export type ThemeMode = 'dark' | 'light' | 'system';

export interface ThemeConfig {
  mode: ThemeMode;
  accentColor: string;
  glassOpacity: number;
}

const DEFAULT_THEME: ThemeConfig = {
  mode: 'dark',
  accentColor: '#6366f1',
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

  currentTheme = signal<ThemeConfig>(this.loadTheme());
  accentColors = signal<string[]>(ACCENT_COLORS);

  constructor() {
    effect(() => {
      this.applyTheme(this.currentTheme());
    });
  }

  private loadTheme(): ThemeConfig {
    try {
      const stored = localStorage.getItem(this.STORAGE_KEY);
      if (stored) {
        return { ...DEFAULT_THEME, ...JSON.parse(stored) };
      }
    } catch {}
    return { ...DEFAULT_THEME };
  }

  private saveTheme(config: ThemeConfig) {
    localStorage.setItem(this.STORAGE_KEY, JSON.stringify(config));
  }

  setMode(mode: ThemeMode) {
    this.currentTheme.update((t) => ({ ...t, mode }));
    this.saveTheme(this.currentTheme());
  }

  setAccentColor(color: string) {
    this.currentTheme.update((t) => ({ ...t, accentColor: color }));
    this.saveTheme(this.currentTheme());
  }

  setGlassOpacity(opacity: number) {
    this.currentTheme.update((t) => ({ ...t, glassOpacity: Math.max(0, Math.min(1, opacity)) }));
    this.saveTheme(this.currentTheme());
  }

  applyTheme(config: ThemeConfig) {
    const root = document.documentElement;

    root.style.setProperty('--accent-color', config.accentColor);
    root.style.setProperty('--glass-opacity', config.glassOpacity.toString());

    const isDark =
      config.mode === 'dark' ||
      (config.mode === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);

    root.classList.toggle('dark', isDark);
  }

  getEffectiveMode(): boolean {
    const config = this.currentTheme();
    return (
      config.mode === 'dark' ||
      (config.mode === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)
    );
  }
}
