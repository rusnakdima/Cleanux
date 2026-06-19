import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { CleaningProfile, createEmptyProfile } from '@entities/profile.model';

@Injectable({
  providedIn: 'root',
})
export class ProfileService {
  private api = inject(ApiService);

  constructor() {}

  async saveProfile(profile: CleaningProfile): Promise<string> {
    try {
      const result = await this.api.invoke<string>('save_profile', { profile });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async loadProfile(name: string): Promise<CleaningProfile> {
    try {
      const result = await this.api.invoke<CleaningProfile>('load_profile', { name });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async listProfiles(): Promise<CleaningProfile[]> {
    try {
      const result = await this.api.invoke<CleaningProfile[]>('list_profiles');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async deleteProfile(name: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('delete_profile', { name });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async applyProfile(name: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('apply_profile', { name });
      return result;
    } catch (error) {
      throw error;
    }
  }

  exportProfile(profile: CleaningProfile): void {
    const data = JSON.stringify(profile, null, 2);
    const blob = new Blob([data], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${profile.name.replace(/\s+/g, '_')}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }

  async importProfile(file: File): Promise<CleaningProfile> {
    try {
      const result = await new Promise<CleaningProfile>((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = (e) => {
          try {
            const content = e.target?.result as string;
            const profile = JSON.parse(content) as CleaningProfile;
            resolve(profile);
          } catch (err) {
            reject(new Error('Invalid profile file'));
          }
        };
        reader.onerror = () => reject(new Error('Failed to read file'));
        reader.readAsText(file);
      });
      return result;
    } catch (error) {
      throw error;
    }
  }
}
