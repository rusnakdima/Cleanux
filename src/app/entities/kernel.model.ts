export interface KernelInfo {
  version: string;
  path: string;
  size: number;
  is_current: boolean;
}

export interface InitramfsInfo {
  version: string;
  path: string;
  size: number;
}

export interface BootSpaceInfo {
  total: number;
  used: number;
  available: number;
  usage_percent: number;
}
