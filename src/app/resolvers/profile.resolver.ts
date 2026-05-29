import { inject } from '@angular/core';
import { ResolveFn } from '@angular/router';
import { TauriApiService } from '@api/tauri-api.service';
import { CleaningProfile } from '@models/profile.model';

export const profilesResolver: ResolveFn<CleaningProfile[]> = async () => {
  const api = inject(TauriApiService);

  try {
    return await api.invoke<CleaningProfile[]>('list_profiles');
  } catch (error) {
    console.error('Failed to resolve profiles:', error);
    return [];
  }
};
