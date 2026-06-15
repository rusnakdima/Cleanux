import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { CleaningProfile, createEmptyProfile } from '@models/profile.model';
import { LoggerService } from '@services/logger.service';

@Injectable({
  providedIn: 'root',
})
export class ProfileService {
  private api = inject(ApiService);
  private loggingService = new LoggerService();

  constructor() {
    this.loggingService.info('ProfileService initialized');
  }

  async saveProfile(profile: CleaningProfile): Promise<string> {
    this.loggingService.info('Saving profile', { name: profile.name });
    try {
      const result = await this.api.invoke<string>('save_profile', { profile });
      this.loggingService.info('Profile saved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { name: profile.name });
      throw error;
    }
  }

  async loadProfile(name: string): Promise<CleaningProfile> {
    this.loggingService.info('Loading profile', { name });
    try {
      const result = await this.api.invoke<CleaningProfile>('load_profile', { name });
      this.loggingService.info('Profile loaded', { name });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { name });
      throw error;
    }
  }

  async listProfiles(): Promise<CleaningProfile[]> {
    this.loggingService.info('Listing profiles');
    try {
      const result = await this.api.invoke<CleaningProfile[]>('list_profiles');
      this.loggingService.info('Profiles listed', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async deleteProfile(name: string): Promise<string> {
    this.loggingService.info('Deleting profile', { name });
    try {
      const result = await this.api.invoke<string>('delete_profile', { name });
      this.loggingService.info('Profile deleted');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { name });
      throw error;
    }
  }

  async applyProfile(name: string): Promise<string> {
    this.loggingService.info('Applying profile', { name });
    try {
      const result = await this.api.invoke<string>('apply_profile', { name });
      this.loggingService.info('Profile applied');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { name });
      throw error;
    }
  }

  exportProfile(profile: CleaningProfile): void {
    this.loggingService.info('Exporting profile', { name: profile.name });
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
    this.loggingService.info('Importing profile', { name: file.name });
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
      this.loggingService.info('Profile imported', { name: file.name });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { name: file.name });
      throw error;
    }
  }
}
