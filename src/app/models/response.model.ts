export type Status = 'success' | 'info' | 'warning' | 'error';

export type DataValue =
  | string
  | number
  | boolean
  | unknown[]
  | Record<string, unknown>
  | null;

export interface Response<T = unknown> {
  status: Status;
  message: string;
  data: T;
}

export function isSuccess(response: Response): boolean {
  return response.status === 'success';
}

export function isError(response: Response): boolean {
  return response.status === 'error';
}

export function getData<T>(response: Response<unknown>): T | null {
  if (response.data === null || response.data === undefined) {
    return null;
  }
  return response.data as T;
}
