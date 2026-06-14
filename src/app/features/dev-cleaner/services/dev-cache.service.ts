import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';

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
  private loggingService = getLoggingService();

  constructor() {
    this.loggingService.info('DevCacheService initialized');
  }

  async getDevCacheSummary(): Promise<DevCacheSummary> {
    this.loggingService.info('Getting dev cache summary');
    try {
      const result = await this.api.invoke<DevCacheSummary>('get_dev_cache_summary');
      this.loggingService.info('Dev cache summary retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanNpmCache(): Promise<string> {
    this.loggingService.info('Cleaning npm cache');
    try {
      const result = await this.api.invoke<string>('clean_npm_cache');
      this.loggingService.info('npm cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanPipCache(): Promise<string> {
    this.loggingService.info('Cleaning pip cache');
    try {
      const result = await this.api.invoke<string>('clean_pip_cache');
      this.loggingService.info('pip cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanCargoCache(): Promise<string> {
    this.loggingService.info('Cleaning cargo cache');
    try {
      const result = await this.api.invoke<string>('clean_cargo_cache');
      this.loggingService.info('cargo cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanGoCache(): Promise<string> {
    this.loggingService.info('Cleaning go cache');
    try {
      const result = await this.api.invoke<string>('clean_go_cache');
      this.loggingService.info('go cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanMavenCache(): Promise<string> {
    this.loggingService.info('Cleaning maven cache');
    try {
      const result = await this.api.invoke<string>('clean_maven_cache');
      this.loggingService.info('maven cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanGradleCache(): Promise<string> {
    this.loggingService.info('Cleaning gradle cache');
    try {
      const result = await this.api.invoke<string>('clean_gradle_cache');
      this.loggingService.info('gradle cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanAllDevCaches(): Promise<string> {
    this.loggingService.info('Cleaning all dev caches');
    try {
      const result = await this.api.invoke<string>('clean_all_dev_caches');
      this.loggingService.info('All dev caches cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }
}
