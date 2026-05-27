import { Injectable, signal, computed, inject, LOCALE_ID, Inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { firstValueFrom } from 'rxjs';

export interface Language {
  code: string;
  name: string;
  nativeName: string;
  dir?: 'ltr' | 'rtl';
}

export const availableLanguages: Language[] = [
  { code: 'en', name: 'English', nativeName: 'English' },
  { code: 'es', name: 'Spanish', nativeName: 'Espanol' },
  { code: 'fr', name: 'French', nativeName: 'Francais' },
  { code: 'de', name: 'German', nativeName: 'Deutsch' },
  { code: 'ru', name: 'Russian', nativeName: 'Russkiy' },
  { code: 'zh', name: 'Chinese', nativeName: 'Zhongwen' },
  { code: 'ja', name: 'Japanese', nativeName: 'Nihongo' },
  { code: 'pt-BR', name: 'Portuguese (Brazil)', nativeName: 'Portugues (Brasil)' },
  { code: 'ko', name: 'Korean', nativeName: 'Hangugeo' },
  { code: 'it', name: 'Italian', nativeName: 'Italiano' },
];

type TranslationData = Record<string, unknown>;

@Injectable({
  providedIn: 'root',
})
export class I18nService {
  private http = inject(HttpClient);
  @Inject(LOCALE_ID) private localeId: string = 'en';

  private translations = signal<TranslationData>({});
  private currentLanguageCode = signal<string>('en');
  private loadedLanguages = new Set<string>();

  readonly currentLanguage = computed(() => {
    const code = this.currentLanguageCode();
    return availableLanguages.find((l) => l.code === code) || availableLanguages[0];
  });

  readonly direction = computed(() => {
    const lang = this.currentLanguage();
    return lang.dir || 'ltr';
  });

  readonly isRTL = computed(() => this.direction() === 'rtl');

  constructor() {
    this.initLanguage();
  }

  private async initLanguage(): Promise<void> {
    const savedLang = localStorage.getItem('language');
    const browserLang = navigator.language.split('-')[0];
    const browserVariant = navigator.language;

    let lang = savedLang;

    if (!lang) {
      const exactMatch = availableLanguages.find((l) => l.code === browserVariant);
      if (exactMatch) {
        lang = exactMatch.code;
      } else {
        const prefixMatch = availableLanguages.find((l) => l.code === browserLang);
        lang = prefixMatch ? prefixMatch.code : 'en';
      }
    }

    await this.setLanguage(lang);
  }

  async setLanguage(lang: string): Promise<void> {
    const langInfo = availableLanguages.find((l) => l.code === lang);
    if (!langInfo) {
      console.warn(`Language '${lang}' is not available, falling back to 'en'`);
      lang = 'en';
    }

    this.currentLanguageCode.set(lang);
    localStorage.setItem('language', lang);

    if (!this.loadedLanguages.has(lang)) {
      await this.loadLanguage(lang);
    }
  }

  private async loadLanguage(lang: string): Promise<void> {
    try {
      const translations = await firstValueFrom(
        this.http.get<TranslationData>(`/app/i18n/${lang}.json?t=${Date.now()}`)
      );
      this.translations.update((current) => ({
        ...current,
        [lang]: translations,
      }));
      this.loadedLanguages.add(lang);
    } catch (error) {
      console.error(`Failed to load translations for '${lang}':`, error);
      if (lang !== 'en') {
        await this.loadLanguage('en');
      }
    }
  }

  async translateAsync(key: string, params?: Record<string, string | number>): Promise<string> {
    return this.translateKey(key, params);
  }

  translateKey(key: string, params?: Record<string, string | number>): string {
    const lang = this.currentLanguageCode();
    const langTranslations = this.translations()[lang] as TranslationData | undefined;

    if (!langTranslations) {
      return key;
    }

    const value = this.getNestedValue(langTranslations, key);

    if (typeof value !== 'string') {
      return key;
    }

    if (params) {
      return Object.entries(params).reduce(
        (str, [paramKey, paramValue]) =>
          str.replace(new RegExp(`{{${paramKey}}}`, 'g'), String(paramValue)),
        value
      );
    }

    return value;
  }

  private getNestedValue(obj: TranslationData, path: string): unknown {
    const keys = path.split('.');
    let current: unknown = obj;

    for (const key of keys) {
      if (current && typeof current === 'object' && key in current) {
        current = (current as TranslationData)[key];
      } else {
        return undefined;
      }
    }

    return current;
  }

  formatDate(date: Date | string | number, format: string = 'mediumDate'): string {
    const d = date instanceof Date ? date : new Date(date);
    return new Intl.DateTimeFormat(this.localeId).format(d);
  }

  formatNumber(num: number, options?: Intl.NumberFormatOptions): string {
    return new Intl.NumberFormat(this.localeId, options).format(num);
  }

  formatCurrency(amount: number, currency: string = 'USD'): string {
    return new Intl.NumberFormat(this.localeId, {
      style: 'currency',
      currency,
    }).format(amount);
  }

  formatPercent(value: number, decimals: number = 0): string {
    return new Intl.NumberFormat(this.localeId, {
      style: 'percent',
      minimumFractionDigits: decimals,
      maximumFractionDigits: decimals,
    }).format(value);
  }
}
