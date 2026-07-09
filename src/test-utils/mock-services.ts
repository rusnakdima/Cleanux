import { vi } from 'vitest';

export const mockApiService = {
  invoke: vi.fn(),
  listen: vi.fn(),
};

export const mockInvokeWrapperService = {
  invoke: vi.fn(),
};

export function setupMockInvoke<T>(mockReturn: T) {
  (mockInvokeWrapperService.invoke as ReturnType<typeof vi.fn>).mockResolvedValue(mockReturn);
}

export function setupMockInvokeRejected(error: unknown) {
  (mockInvokeWrapperService.invoke as ReturnType<typeof vi.fn>).mockRejectedValue(error);
}

export function resetMockInvoke() {
  (mockInvokeWrapperService.invoke as ReturnType<typeof vi.fn>).mockReset();
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
