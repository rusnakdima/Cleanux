import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';

export interface DevCacheItem {
  name: string;
  cache_path: string;
  size: number;
  description: string;
}

export interface DevCacheSummary {
  npm: DevCacheItem;
  pip: DevCacheItem;
  cargo: DevCacheItem;
  go: DevCacheItem;
  maven: DevCacheItem;
  gradle: DevCacheItem;
}

@Injectable({
  providedIn: 'root',
})
export class DevCacheService {
  private api = inject(ApiService);

  constructor() {}

  async getDevCacheSummary(): Promise<DevCacheSummary> {
    try {
      const result = await this.api.invoke<DevCacheSummary>('get_dev_cache_summary');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanNpmCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_npm_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanPipCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_pip_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanCargoCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_cargo_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanGoCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_go_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanMavenCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_maven_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanGradleCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_gradle_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanAllDevCaches(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_all_dev_caches');
      return result;
    } catch (error) {
      throw error;
    }
  }
}
