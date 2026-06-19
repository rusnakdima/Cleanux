export interface MemoryInfo {
  total: number;
  used: number;
  available: number;
  cached: number;
  buffers: number;
}

export interface SwapInfo {
  total: number;
  used: number;
}

export interface ProcessMemory {
  pid: number;
  name: string;
  memory_mb: number;
  cpu_percent: number;
}
