/* angular */
import { Injectable, inject } from '@angular/core';

/* library */
import { InvokeWrapperService } from '@tauri-front/shared';

export interface Profile {
  name: string;
  description: string;
  created_at: string;
  paths: string[];
  exclude_patterns: string[];
  clean_cache: boolean;
  clean_trash: boolean;
  clean_logs: boolean;
  min_large_file_size: number;
}

@Injectable({ providedIn: 'root' })
export class ProfileService {
  private api = inject(InvokeWrapperService);

  async saveProfile(profile: Profile): Promise<string> {
    return this.api.invoke<string>('save_profile', { profile });
  }

  async loadProfile(name: string): Promise<Profile> {
    return this.api.invoke<Profile>('load_profile', { name });
  }

  async listProfiles(): Promise<Profile[]> {
    return this.api.invoke<Profile[]>('list_profiles');
  }

  async deleteProfile(name: string): Promise<string> {
    return this.api.invoke<string>('delete_profile', { name });
  }

  async applyProfile(name: string): Promise<string> {
    return this.api.invoke<string>('apply_profile', { name });
  }

  async importProfile(file: File): Promise<Profile> {
    const text = await file.text();
    try {
      return JSON.parse(text) as Profile;
    } catch {
      throw new Error('Invalid profile file');
    }
  }
}
