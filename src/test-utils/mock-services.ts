import { vi } from 'vitest';

export const mockApiService = {
  invoke: vi.fn(),
  listen: vi.fn(),
};

export const mockTauriApiService = {
  invoke: vi.fn(),
  listen: vi.fn(),
};

export function setupMockInvoke<T>(mockReturn: T) {
  (mockTauriApiService.invoke as ReturnType<typeof vi.fn>).mockResolvedValue(mockReturn);
}

export function setupMockInvokeRejected(error: unknown) {
  (mockTauriApiService.invoke as ReturnType<typeof vi.fn>).mockRejectedValue(error);
}

export function resetMockInvoke() {
  (mockTauriApiService.invoke as ReturnType<typeof vi.fn>).mockReset();
  (mockTauriApiService.listen as ReturnType<typeof vi.fn>).mockReset();
}

export function createMockResponse<T>(
  data: T,
  status: 'success' | 'error' = 'success',
  message = ''
) {
  return {
    status,
    message,
    data,
  };
}
