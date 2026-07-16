import { describe, it, expect, vi, beforeEach } from 'vitest';
import { NotificationService } from './notification.service';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-front/shared', () => ({
  formatError: vi.fn((e: unknown) => (e instanceof Error ? e.message : String(e))),
  parseError: vi.fn((e: unknown) => {
    if (e === null || e === undefined) return 'Unknown error';
    if (e instanceof Error) return e.message;
    return String(e);
  }),
}));

describe('NotificationService', () => {
  let originalAlert: typeof window.alert;
  let originalConfirm: typeof window.confirm;
  let alertSpy: ReturnType<typeof vi.spyOn>;
  let confirmSpy: ReturnType<typeof vi.spyOn>;

  beforeEach(() => {
    originalAlert = window.alert;
    originalConfirm = window.confirm;
    alertSpy = vi.spyOn(window, 'alert').mockImplementation(vi.fn());
    confirmSpy = vi.spyOn(window, 'confirm').mockImplementation(vi.fn(() => true));
  });

  it('should call window.alert with the message', () => {
    const service = new NotificationService();

    service.alert('Test message');

    expect(alertSpy).toHaveBeenCalledWith('Test message');
  });

  it('should call window.confirm with the message and return result', () => {
    const service = new NotificationService();

    confirmSpy.mockReturnValueOnce(true);
    const result = service.confirm('Are you sure?');

    expect(confirmSpy).toHaveBeenCalledWith('Are you sure?');
    expect(result).toBe(true);
  });

  it('should return false when confirm is cancelled', () => {
    const service = new NotificationService();

    confirmSpy.mockReturnValueOnce(false);
    const result = service.confirm('Are you sure?');

    expect(result).toBe(false);
  });

  it('should format error and call alert for error()', () => {
    const service = new NotificationService();

    service.error('Operation failed', new Error('Network error'));

    expect(alertSpy).toHaveBeenCalledWith('Operation failed: Network error');
  });

  it('should handle string error in error()', () => {
    const service = new NotificationService();

    service.error('Operation failed', 'Something went wrong');

    expect(alertSpy).toHaveBeenCalledWith('Operation failed: Something went wrong');
  });

  it('should handle unknown error in error()', () => {
    const service = new NotificationService();

    service.error('Operation failed', null);

    expect(alertSpy).toHaveBeenCalledWith('Operation failed: Unknown error');
  });

  it('should call alert for success()', () => {
    const service = new NotificationService();

    service.success('Operation completed');

    expect(alertSpy).toHaveBeenCalledWith('Operation completed');
  });

  it('should prefix with Failed to for cleanError()', () => {
    const service = new NotificationService();

    service.cleanError('save data', new Error('disk full'));

    expect(alertSpy).toHaveBeenCalledWith('Failed to save data: disk full');
  });
});
