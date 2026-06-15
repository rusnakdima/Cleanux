import { Injectable } from '@angular/core';

const TOKEN_KEY = 'auth_token';
const USER_KEY = 'auth_user';

@Injectable({ providedIn: 'root' })
export class StorageService {
  setToken(token: string): void {
    localStorage.setItem(TOKEN_KEY, token);
  }

  getToken(): string | null {
    return localStorage.getItem(TOKEN_KEY);
  }

  hasToken(): boolean {
    const token = this.getToken();
    return token !== null && token.length > 0;
  }

  clearToken(): void {
    localStorage.removeItem(TOKEN_KEY);
    sessionStorage.removeItem(USER_KEY);
  }

  setUser<T>(user: T): void {
    sessionStorage.setItem(USER_KEY, JSON.stringify(user));
  }

  getUser<T>(): T | null {
    const data = sessionStorage.getItem(USER_KEY);
    if (!data) return null;
    try {
      return JSON.parse(data) as T;
    } catch {
      return null;
    }
  }

  clearUser(): void {
    sessionStorage.removeItem(USER_KEY);
  }

  clearAll(): void {
    this.clearToken();
    this.clearUser();
  }
}
