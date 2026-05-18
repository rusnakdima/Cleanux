export interface DuplicateFile {
  name: string;
  path: string;
  size: number;
}

export interface DuplicateGroup {
  hash: string;
  file_size: number;
  wasted_space: number;
  files: DuplicateFile[];
}

export interface DuplicateScanResult {
  groups: DuplicateGroup[];
  totalGroups: number;
  totalDuplicates: number;
  totalWastedSpace: number;
}
