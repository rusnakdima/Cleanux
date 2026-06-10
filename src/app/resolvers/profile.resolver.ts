import { inject } from '@angular/core';
import { ResolveFn } from '@angular/router';
import { ApiService } from '@services/api.service';
import { CleaningProfile } from '@models/profile.model';

export const profilesResolver: ResolveFn<CleaningProfile[]> = async () => {
  const api = inject(ApiService);

  try {
    return await api.invoke<CleaningProfile[]>('list_profiles');
  } catch (error) {
    throw error;
  }
};
