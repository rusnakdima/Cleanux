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
    const body = document.body;

    const accentColor = config.accentColor;
    const r = parseInt(accentColor.slice(1, 3), 16);
    const g = parseInt(accentColor.slice(3, 5), 16);
    const b = parseInt(accentColor.slice(5, 7), 16);

    root.style.setProperty('--accent', accentColor);
    root.style.setProperty('--accent-500', accentColor);
    root.style.setProperty('--accent-light', `rgba(${r}, ${g}, ${b}, 0.1)`);
    root.style.setProperty('--accent-glow', `rgba(${r}, ${g}, ${b}, 0.3)`);
    root.style.setProperty('--accent-rgb', `${r}, ${g}, ${b}`);
    root.style.setProperty('--glass-opacity', config.glassOpacity.toString());

    root.style.setProperty('--icon-cpu', accentColor);
    root.style.setProperty('--icon-memory', accentColor);

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
