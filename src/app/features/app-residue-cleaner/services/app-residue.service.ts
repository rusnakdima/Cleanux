import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';

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

export interface CleanResult {
  removed: boolean;
  failed: string[];
  spaceFreed: number;
  message: string;
}

export interface AppResidueSummary {
  total_configs: number;
  total_data: number;
  total_caches: number;
  total_size: number;
  found_uninstalled: number;
}

@Injectable({
  providedIn: 'root',
})
export class AppResidueService {
  private api = inject(ApiService);
  private loggingService = getLoggingService();

  constructor() {
    this.loggingService.info('AppResidueService initialized');
  }

  async getResidueSummary(): Promise<AppResidueSummary> {
    this.loggingService.info('Getting residue summary');
    try {
      const result = await this.api.invoke<AppResidueSummary>('get_residue_summary');
      this.loggingService.info('Residue summary retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async scanUserConfigs(): Promise<AppResidue[]> {
    this.loggingService.info('Scanning user configs');
    try {
      const result = await this.api.invoke<AppResidue[]>('scan_user_configs');
      this.loggingService.info('User configs scanned', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async scanUserData(): Promise<AppResidue[]> {
    this.loggingService.info('Scanning user data');
    try {
      const result = await this.api.invoke<AppResidue[]>('scan_user_data');
      this.loggingService.info('User data scanned', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async scanUserCaches(): Promise<AppResidue[]> {
    this.loggingService.info('Scanning user caches');
    try {
      const result = await this.api.invoke<AppResidue[]>('scan_user_caches');
      this.loggingService.info('User caches scanned', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async scanHomeResidues(): Promise<AppResidue[]> {
    this.loggingService.info('Scanning home residues');
    try {
      const result = await this.api.invoke<AppResidue[]>('scan_home_residues');
      this.loggingService.info('Home residues scanned', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async getOrphanedConfigs(): Promise<OrphanedConfig[]> {
    this.loggingService.info('Getting orphaned configs');
    try {
      const result = await this.api.invoke<OrphanedConfig[]>('get_orphaned_configs');
      this.loggingService.info('Orphaned configs retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanResidue(path: string): Promise<CleanResult> {
    this.loggingService.info('Cleaning residue', { path });
    try {
      const result = await this.api.invoke<CleanResult>('clean_app_residue', { path });
      this.loggingService.info('Residue cleaned', { removed: result.removed });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { path });
      throw error;
    }
  }

  async cleanMultipleResidues(paths: string[]): Promise<CleanResult> {
    this.loggingService.info('Cleaning multiple residues', { count: paths.length });
    try {
      const result = await this.api.invoke<CleanResult>('clean_multiple_app_residues', { paths });
      this.loggingService.info('Multiple residues cleaned', { removed: result.removed });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { paths });
      throw error;
    }
  }
}
