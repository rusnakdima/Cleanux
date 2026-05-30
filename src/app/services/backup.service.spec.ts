import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@services/api.service');

describe('BackupService', () => {
  let injector: Injector;
  let mockApi: { invoke: ReturnType<typeof vi.fn> };

  beforeEach(async () => {
    const { ApiService } = await import('@services/api.service');
    mockApi = { invoke: vi.fn() };
    ApiService.prototype.api = mockApi;

    injector = Injector.create({
      providers: [{ provide: ApiService, useValue: mockApi }],
    });
  });

  it('should call create_backup', async () => {
    const { BackupService } = await import('@services/backup.service');
    mockApi.invoke.mockResolvedValue('Backup created');

    const service = runInInjectionContext(injector, () => new BackupService(mockApi as any));
    const result = await service.createBackup(
      ['/home/user/Documents'],
      '/home/user/backups/docs.tar.gz'
    );

    expect(mockApi.invoke).toHaveBeenCalledWith('create_backup', {
      paths: ['/home/user/Documents'],
      archivePath: '/home/user/backups/docs.tar.gz',
    });
    expect(result).toBe('Backup created');
  });

  it('should call restore_backup', async () => {
    const { BackupService } = await import('@services/backup.service');
    mockApi.invoke.mockResolvedValue('Backup restored');

    const service = runInInjectionContext(injector, () => new BackupService(mockApi as any));
    const result = await service.restoreBackup(
      '/home/user/backups/docs.tar.gz',
      '/home/user/Documents'
    );

    expect(mockApi.invoke).toHaveBeenCalledWith('restore_backup', {
      archivePath: '/home/user/backups/docs.tar.gz',
      destination: '/home/user/Documents',
    });
    expect(result).toBe('Backup restored');
  });

  it('should call list_backups', async () => {
    const { BackupService } = await import('@services/backup.service');
    const mockBackups = [
      {
        name: 'backup0.tar.gz',
        path: '/home/user/backups/backup0.tar.gz',
        size: 50 * 1024 * 1024,
        modified: new Date().toISOString(),
      },
    ];
    mockApi.invoke.mockResolvedValue(mockBackups);

    const service = runInInjectionContext(injector, () => new BackupService(mockApi as any));
    const result = await service.listBackups();

    expect(mockApi.invoke).toHaveBeenCalledWith('list_backups');
    expect(result).toHaveLength(1);
    expect(result[0].name).toBe('backup0.tar.gz');
  });

  it('should call delete_backup', async () => {
    const { BackupService } = await import('@services/backup.service');
    mockApi.invoke.mockResolvedValue('Backup deleted');

    const service = runInInjectionContext(injector, () => new BackupService(mockApi as any));
    const result = await service.deleteBackup('/home/user/backups/backup0.tar.gz');

    expect(mockApi.invoke).toHaveBeenCalledWith('delete_backup', {
      archivePath: '/home/user/backups/backup0.tar.gz',
    });
    expect(result).toBe('Backup deleted');
  });

  it('should call get_backup_dir', async () => {
    const { BackupService } = await import('@services/backup.service');
    mockApi.invoke.mockResolvedValue('/home/user/.config/cleanux/backups');

    const service = runInInjectionContext(injector, () => new BackupService(mockApi as any));
    const result = await service.getBackupDir();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_backup_dir');
    expect(result).toBe('/home/user/.config/cleanux/backups');
  });
});
