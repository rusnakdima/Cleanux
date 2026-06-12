import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { CleaningProfile, createEmptyProfile } from '@models/profile.model';
import { LoggerService } from '@services/logger.service';

@Injectable({
  providedIn: 'root',
})
export class ProfileService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'ProfileService', 'init', 'ProfileService initialized');
  }

  async saveProfile(profile: CleaningProfile): Promise<string> {
    this.logger.logInfo('service', 'ProfileService', 'saveProfile', 'Saving profile', {
      name: profile.name,
    });
    try {
      const result = await this.api.invoke<string>('save_profile', { profile });
      this.logger.logInfo('service', 'ProfileService', 'saveProfile', 'Profile saved');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'ProfileService',
        'saveProfile',
        'Operation failed',
        error as Error,
        { name: profile.name }
      );
      throw error;
    }
  }

  async loadProfile(name: string): Promise<CleaningProfile> {
    this.logger.logInfo('service', 'ProfileService', 'loadProfile', 'Loading profile', { name });
    try {
      const result = await this.api.invoke<CleaningProfile>('load_profile', { name });
      this.logger.logInfo('service', 'ProfileService', 'loadProfile', 'Profile loaded', { name });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'ProfileService',
        'loadProfile',
        'Operation failed',
        error as Error,
        { name }
      );
      throw error;
    }
  }

  async listProfiles(): Promise<CleaningProfile[]> {
    this.logger.logInfo('service', 'ProfileService', 'listProfiles', 'Listing profiles');
    try {
      const result = await this.api.invoke<CleaningProfile[]>('list_profiles');
      this.logger.logInfo('service', 'ProfileService', 'listProfiles', 'Profiles listed', {
        count: result.length,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'ProfileService',
        'listProfiles',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async deleteProfile(name: string): Promise<string> {
    this.logger.logInfo('service', 'ProfileService', 'deleteProfile', 'Deleting profile', { name });
    try {
      const result = await this.api.invoke<string>('delete_profile', { name });
      this.logger.logInfo('service', 'ProfileService', 'deleteProfile', 'Profile deleted');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'ProfileService',
        'deleteProfile',
        'Operation failed',
        error as Error,
        { name }
      );
      throw error;
    }
  }

  async applyProfile(name: string): Promise<string> {
    this.logger.logInfo('service', 'ProfileService', 'applyProfile', 'Applying profile', { name });
    try {
      const result = await this.api.invoke<string>('apply_profile', { name });
      this.logger.logInfo('service', 'ProfileService', 'applyProfile', 'Profile applied');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'ProfileService',
        'applyProfile',
        'Operation failed',
        error as Error,
        { name }
      );
      throw error;
    }
  }

  exportProfile(profile: CleaningProfile): void {
    this.logger.logInfo('service', 'ProfileService', 'exportProfile', 'Exporting profile', {
      name: profile.name,
    });
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
    this.logger.logInfo('service', 'ProfileService', 'importProfile', 'Importing profile', {
      name: file.name,
    });
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
      this.logger.logInfo('service', 'ProfileService', 'importProfile', 'Profile imported', {
        name: file.name,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'ProfileService',
        'importProfile',
        'Operation failed',
        error as Error,
        { name: file.name }
      );
      throw error;
    }
  }
}
