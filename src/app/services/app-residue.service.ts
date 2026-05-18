import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';

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

  async getResidueSummary(): Promise<AppResidueSummary> {
    return await this.api.invoke<AppResidueSummary>('get_residue_summary');
  }

  async scanUserConfigs(): Promise<AppResidue[]> {
    return await this.api.invoke<AppResidue[]>('scan_user_configs');
  }

  async scanUserData(): Promise<AppResidue[]> {
    return await this.api.invoke<AppResidue[]>('scan_user_data');
  }

  async scanUserCaches(): Promise<AppResidue[]> {
    return await this.api.invoke<AppResidue[]>('scan_user_caches');
  }

  async scanHomeResidues(): Promise<AppResidue[]> {
    return await this.api.invoke<AppResidue[]>('scan_home_residues');
  }

  async getOrphanedConfigs(): Promise<OrphanedConfig[]> {
    return await this.api.invoke<OrphanedConfig[]>('get_orphaned_configs');
  }

  async cleanResidue(path: string): Promise<CleanResult> {
    return await this.api.invoke<CleanResult>('clean_app_residue', { path });
  }

  async cleanMultipleResidues(paths: string[]): Promise<CleanResult> {
    return await this.api.invoke<CleanResult>('clean_multiple_app_residues', { paths });
  }
}
