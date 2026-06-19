import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';

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

  constructor() {}

  async getResidueSummary(): Promise<AppResidueSummary> {
    try {
      const result = await this.api.invoke<AppResidueSummary>('get_residue_summary');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async scanUserConfigs(): Promise<AppResidue[]> {
    try {
      const result = await this.api.invoke<AppResidue[]>('scan_user_configs');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async scanUserData(): Promise<AppResidue[]> {
    try {
      const result = await this.api.invoke<AppResidue[]>('scan_user_data');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async scanUserCaches(): Promise<AppResidue[]> {
    try {
      const result = await this.api.invoke<AppResidue[]>('scan_user_caches');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async scanHomeResidues(): Promise<AppResidue[]> {
    try {
      const result = await this.api.invoke<AppResidue[]>('scan_home_residues');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getOrphanedConfigs(): Promise<OrphanedConfig[]> {
    try {
      const result = await this.api.invoke<OrphanedConfig[]>('get_orphaned_configs');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanResidue(path: string): Promise<CleanResult> {
    try {
      const result = await this.api.invoke<CleanResult>('clean_app_residue', { path });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanMultipleResidues(paths: string[]): Promise<CleanResult> {
    try {
      const result = await this.api.invoke<CleanResult>('clean_multiple_app_residues', { paths });
      return result;
    } catch (error) {
      throw error;
    }
  }
}
