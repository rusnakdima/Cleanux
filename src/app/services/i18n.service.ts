import { Injectable, signal, computed, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { firstValueFrom } from 'rxjs';

export interface Language {
  code: string;
  name: string;
  nativeName: string;
}

export const availableLanguages: Language[] = [
  { code: 'en', name: 'English', nativeName: 'English' },
  { code: 'es', name: 'Spanish', nativeName: 'Espanol' },
  { code: 'fr', name: 'French', nativeName: 'Francais' },
  { code: 'de', name: 'German', nativeName: 'Deutsch' },
  { code: 'ru', name: 'Russian', nativeName: 'Russkiy' },
  { code: 'zh', name: 'Chinese', nativeName: 'Zhongwen' },
  { code: 'ja', name: 'Japanese', nativeName: 'Nihongo' },
];

type TranslationData = Record<string, unknown>;

@Injectable({
  providedIn: 'root',
})
export class I18nService {
  private http = inject(HttpClient);

  private translations = signal<TranslationData>({});
  private currentLanguageCode = signal<string>('en');
  private loadedLanguages = new Set<string>();

  readonly currentLanguage = computed(() => {
    const code = this.currentLanguageCode();
    return availableLanguages.find((l) => l.code === code) || availableLanguages[0];
  });

  constructor() {
    this.initLanguage();
  }

  private async initLanguage(): Promise<void> {
    const savedLang = localStorage.getItem('language');
    const browserLang = navigator.language.split('-')[0];
    const lang =
      savedLang || (availableLanguages.some((l) => l.code === browserLang) ? browserLang : 'en');
    await this.setLanguage(lang);
  }

  async setLanguage(lang: string): Promise<void> {
    if (!availableLanguages.some((l) => l.code === lang)) {
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
}
