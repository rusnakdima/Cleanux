import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@services/api.service');

describe('ProfileService', () => {
  let injector: Injector;
  let mockApi: { invoke: ReturnType<typeof vi.fn> };

  const createMockProfile = (name: string) => ({
    name,
    description: `Test profile ${name}`,
    created_at: new Date().toISOString(),
    paths: ['/path/to/clean'],
    exclude_patterns: ['*.log', '*.tmp'],
    clean_cache: true,
    clean_trash: true,
    clean_logs: false,
    min_large_file_size: 100 * 1024 * 1024,
  });

  beforeEach(async () => {
    const { ApiService } = await import('@services/api.service');
    mockApi = { invoke: vi.fn() };
    ApiService.prototype.api = mockApi;

    injector = Injector.create({
      providers: [{ provide: ApiService, useValue: mockApi }],
    });
  });

  it('should call save_profile', async () => {
    const { ProfileService } = await import('@services/profile.service');
    mockApi.invoke.mockResolvedValue('Profile saved');
    const profile = createMockProfile('Profile 1');

    const service = runInInjectionContext(injector, () => new ProfileService(mockApi as any));
    const result = await service.saveProfile(profile);

    expect(mockApi.invoke).toHaveBeenCalledWith('save_profile', { profile });
    expect(result).toBe('Profile saved');
  });

  it('should call load_profile', async () => {
    const { ProfileService } = await import('@services/profile.service');
    const mockProfile = createMockProfile('Profile 1');
    mockApi.invoke.mockResolvedValue(mockProfile);

    const service = runInInjectionContext(injector, () => new ProfileService(mockApi as any));
    const result = await service.loadProfile('Profile 1');

    expect(mockApi.invoke).toHaveBeenCalledWith('load_profile', { name: 'Profile 1' });
    expect(result).toEqual(mockProfile);
  });

  it('should call list_profiles', async () => {
    const { ProfileService } = await import('@services/profile.service');
    const mockProfiles = [createMockProfile('Profile 1'), createMockProfile('Profile 2')];
    mockApi.invoke.mockResolvedValue(mockProfiles);

    const service = runInInjectionContext(injector, () => new ProfileService(mockApi as any));
    const result = await service.listProfiles();

    expect(mockApi.invoke).toHaveBeenCalledWith('list_profiles');
    expect(result).toHaveLength(2);
  });

  it('should call delete_profile', async () => {
    const { ProfileService } = await import('@services/profile.service');
    mockApi.invoke.mockResolvedValue('Profile deleted');

    const service = runInInjectionContext(injector, () => new ProfileService(mockApi as any));
    const result = await service.deleteProfile('Profile 1');

    expect(mockApi.invoke).toHaveBeenCalledWith('delete_profile', { name: 'Profile 1' });
    expect(result).toBe('Profile deleted');
  });

  it('should call apply_profile', async () => {
    const { ProfileService } = await import('@services/profile.service');
    mockApi.invoke.mockResolvedValue('Profile applied');

    const service = runInInjectionContext(injector, () => new ProfileService(mockApi as any));
    const result = await service.applyProfile('Profile 1');

    expect(mockApi.invoke).toHaveBeenCalledWith('apply_profile', { name: 'Profile 1' });
    expect(result).toBe('Profile applied');
  });

  it('should parse valid profile file', async () => {
    const { ProfileService } = await import('@services/profile.service');
    const profile = createMockProfile('Imported Profile');
    const file = new File([JSON.stringify(profile)], 'profile.json', { type: 'application/json' });

    const service = runInInjectionContext(injector, () => new ProfileService(mockApi as any));
    const result = await service.importProfile(file);

    expect(result).toEqual(profile);
  });

  it('should reject invalid profile file', async () => {
    const { ProfileService } = await import('@services/profile.service');
    const file = new File(['invalid json'], 'profile.json', { type: 'application/json' });

    const service = runInInjectionContext(injector, () => new ProfileService(mockApi as any));
    await expect(service.importProfile(file)).rejects.toThrow('Invalid profile file');
  });
});
