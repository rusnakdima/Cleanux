import { Injectable } from '@angular/core';

const STORAGE_PREFIX = 'cleanux_';

@Injectable({
  providedIn: 'root',
})
export class LocalStorageService {
  private prefix: string = STORAGE_PREFIX;

  private makeKey(key: string): string {
    return `${this.prefix}${key}`;
  }

  set<T>(key: string, value: T): void {
    try {
      const serialized = JSON.stringify(value);
      localStorage.setItem(this.makeKey(key), serialized);
    } catch (e) {
      console.error('Failed to serialize value for storage:', e);
    }
  }

  get<T>(key: string, defaultValue?: T): T | null {
    try {
      const item = localStorage.getItem(this.makeKey(key));
      if (item === null) return defaultValue ?? null;
      return JSON.parse(item) as T;
    } catch (e) {
      console.error('Failed to deserialize value from storage:', e);
      return defaultValue ?? null;
    }
  }

  remove(key: string): void {
    localStorage.removeItem(this.makeKey(key));
  }

  clear(): void {
    const keysToRemove: string[] = [];
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key?.startsWith(this.prefix)) {
        keysToRemove.push(key);
      }
    }
    keysToRemove.forEach((key) => localStorage.removeItem(key));
  }

  has(key: string): boolean {
    return localStorage.getItem(this.makeKey(key)) !== null;
  }
}
