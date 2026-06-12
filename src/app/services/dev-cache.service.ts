import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { LoggerService } from '@services/logger.service';

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
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'DevCacheService', 'init', 'DevCacheService initialized');
  }

  async getDevCacheSummary(): Promise<DevCacheSummary> {
    this.logger.logInfo(
      'service',
      'DevCacheService',
      'getDevCacheSummary',
      'Getting dev cache summary'
    );
    try {
      const result = await this.api.invoke<DevCacheSummary>('get_dev_cache_summary');
      this.logger.logInfo(
        'service',
        'DevCacheService',
        'getDevCacheSummary',
        'Dev cache summary retrieved'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'DevCacheService',
        'getDevCacheSummary',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanNpmCache(): Promise<string> {
    this.logger.logInfo('service', 'DevCacheService', 'cleanNpmCache', 'Cleaning npm cache');
    try {
      const result = await this.api.invoke<string>('clean_npm_cache');
      this.logger.logInfo('service', 'DevCacheService', 'cleanNpmCache', 'npm cache cleaned');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'DevCacheService',
        'cleanNpmCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanPipCache(): Promise<string> {
    this.logger.logInfo('service', 'DevCacheService', 'cleanPipCache', 'Cleaning pip cache');
    try {
      const result = await this.api.invoke<string>('clean_pip_cache');
      this.logger.logInfo('service', 'DevCacheService', 'cleanPipCache', 'pip cache cleaned');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'DevCacheService',
        'cleanPipCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanCargoCache(): Promise<string> {
    this.logger.logInfo('service', 'DevCacheService', 'cleanCargoCache', 'Cleaning cargo cache');
    try {
      const result = await this.api.invoke<string>('clean_cargo_cache');
      this.logger.logInfo('service', 'DevCacheService', 'cleanCargoCache', 'cargo cache cleaned');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'DevCacheService',
        'cleanCargoCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanGoCache(): Promise<string> {
    this.logger.logInfo('service', 'DevCacheService', 'cleanGoCache', 'Cleaning go cache');
    try {
      const result = await this.api.invoke<string>('clean_go_cache');
      this.logger.logInfo('service', 'DevCacheService', 'cleanGoCache', 'go cache cleaned');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'DevCacheService',
        'cleanGoCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanMavenCache(): Promise<string> {
    this.logger.logInfo('service', 'DevCacheService', 'cleanMavenCache', 'Cleaning maven cache');
    try {
      const result = await this.api.invoke<string>('clean_maven_cache');
      this.logger.logInfo('service', 'DevCacheService', 'cleanMavenCache', 'maven cache cleaned');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'DevCacheService',
        'cleanMavenCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanGradleCache(): Promise<string> {
    this.logger.logInfo('service', 'DevCacheService', 'cleanGradleCache', 'Cleaning gradle cache');
    try {
      const result = await this.api.invoke<string>('clean_gradle_cache');
      this.logger.logInfo('service', 'DevCacheService', 'cleanGradleCache', 'gradle cache cleaned');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'DevCacheService',
        'cleanGradleCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanAllDevCaches(): Promise<string> {
    this.logger.logInfo(
      'service',
      'DevCacheService',
      'cleanAllDevCaches',
      'Cleaning all dev caches'
    );
    try {
      const result = await this.api.invoke<string>('clean_all_dev_caches');
      this.logger.logInfo(
        'service',
        'DevCacheService',
        'cleanAllDevCaches',
        'All dev caches cleaned'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'DevCacheService',
        'cleanAllDevCaches',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }
}
