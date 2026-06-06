import { Injectable } from '@angular/core';
import { BaseApiService } from './base-api.service';
import { OperationResult } from '@models/cleaner.models';

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

@Injectable({
  providedIn: 'root',
})
export class AppResidueService extends BaseApiService {
  async getResidueSummary(): Promise<AppResidueSummary> {
    return await this.call<AppResidueSummary>('get_residue_summary');
  }

  async scanUserConfigs(): Promise<AppResidue[]> {
    return await this.call<AppResidue[]>('scan_user_configs');
  }

  async scanUserData(): Promise<AppResidue[]> {
    return await this.call<AppResidue[]>('scan_user_data');
  }

  async scanUserCaches(): Promise<AppResidue[]> {
    return await this.call<AppResidue[]>('scan_user_caches');
  }

  async scanHomeResidues(): Promise<AppResidue[]> {
    return await this.call<AppResidue[]>('scan_home_residues');
  }

  async getOrphanedConfigs(): Promise<OrphanedConfig[]> {
    return await this.call<OrphanedConfig[]>('get_orphaned_configs');
  }

  async cleanResidue(path: string): Promise<OperationResult> {
    return await this.call<OperationResult>('clean_app_residue', { path });
  }

  async cleanMultipleResidues(paths: string[]): Promise<OperationResult> {
    return await this.call<OperationResult>('clean_multiple_app_residues', { paths });
  }
}
