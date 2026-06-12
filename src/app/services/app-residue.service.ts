import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';

export interface AppResidue {
  path: string;
  app_name: string;
  size: number;
  residue_type: 'Config' | 'Data' | 'Cache' | 'Both';
  detected_as_uninstalled: boolean;
}

export interface OrphanedConfig {
  path: string;
  package_name: string;
  config_type: string;
}

export interface AppResidueSummary {
  total_configs: number;
  total_data: number;
  total_caches: number;
  total_size: number;
  found_uninstalled: number;
}

export interface CleanResult {
  removed: number;
  failed: string[];
}

@Injectable({
  providedIn: 'root',
})
export class AppResidueService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'AppResidueService', 'init', 'AppResidueService initialized');
  }

  async getResidueSummary(): Promise<AppResidueSummary> {
    this.logger.logInfo(
      'service',
      'AppResidueService',
      'getResidueSummary',
      'Getting residue summary'
    );
    try {
      const result = await this.api.invoke<AppResidueSummary>('get_residue_summary');
      this.logger.logInfo(
        'service',
        'AppResidueService',
        'getResidueSummary',
        'Residue summary retrieved'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AppResidueService',
        'getResidueSummary',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async scanUserConfigs(): Promise<AppResidue[]> {
    this.logger.logInfo('service', 'AppResidueService', 'scanUserConfigs', 'Scanning user configs');
    try {
      const result = await this.api.invoke<AppResidue[]>('scan_user_configs');
      this.logger.logInfo(
        'service',
        'AppResidueService',
        'scanUserConfigs',
        'User configs scanned',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AppResidueService',
        'scanUserConfigs',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async scanUserData(): Promise<AppResidue[]> {
    this.logger.logInfo('service', 'AppResidueService', 'scanUserData', 'Scanning user data');
    try {
      const result = await this.api.invoke<AppResidue[]>('scan_user_data');
      this.logger.logInfo('service', 'AppResidueService', 'scanUserData', 'User data scanned', {
        count: result.length,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AppResidueService',
        'scanUserData',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async scanUserCaches(): Promise<AppResidue[]> {
    this.logger.logInfo('service', 'AppResidueService', 'scanUserCaches', 'Scanning user caches');
    try {
      const result = await this.api.invoke<AppResidue[]>('scan_user_caches');
      this.logger.logInfo('service', 'AppResidueService', 'scanUserCaches', 'User caches scanned', {
        count: result.length,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AppResidueService',
        'scanUserCaches',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async scanHomeResidues(): Promise<AppResidue[]> {
    this.logger.logInfo(
      'service',
      'AppResidueService',
      'scanHomeResidues',
      'Scanning home residues'
    );
    try {
      const result = await this.api.invoke<AppResidue[]>('scan_home_residues');
      this.logger.logInfo(
        'service',
        'AppResidueService',
        'scanHomeResidues',
        'Home residues scanned',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AppResidueService',
        'scanHomeResidues',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async getOrphanedConfigs(): Promise<OrphanedConfig[]> {
    this.logger.logInfo(
      'service',
      'AppResidueService',
      'getOrphanedConfigs',
      'Getting orphaned configs'
    );
    try {
      const result = await this.api.invoke<OrphanedConfig[]>('get_orphaned_configs');
      this.logger.logInfo(
        'service',
        'AppResidueService',
        'getOrphanedConfigs',
        'Orphaned configs retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AppResidueService',
        'getOrphanedConfigs',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanResidue(path: string): Promise<CleanResult> {
    this.logger.logInfo('service', 'AppResidueService', 'cleanResidue', 'Cleaning residue', {
      path,
    });
    try {
      const result = await this.api.invoke<CleanResult>('clean_app_residue', { path });
      this.logger.logInfo('service', 'AppResidueService', 'cleanResidue', 'Residue cleaned', {
        removed: result.removed,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AppResidueService',
        'cleanResidue',
        'Operation failed',
        error as Error,
        { path }
      );
      throw error;
    }
  }

  async cleanMultipleResidues(paths: string[]): Promise<CleanResult> {
    this.logger.logInfo(
      'service',
      'AppResidueService',
      'cleanMultipleResidues',
      'Cleaning multiple residues',
      { count: paths.length }
    );
    try {
      const result = await this.api.invoke<CleanResult>('clean_multiple_app_residues', { paths });
      this.logger.logInfo(
        'service',
        'AppResidueService',
        'cleanMultipleResidues',
        'Multiple residues cleaned',
        { removed: result.removed }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AppResidueService',
        'cleanMultipleResidues',
        'Operation failed',
        error as Error,
        { paths }
      );
      throw error;
    }
  }
}
