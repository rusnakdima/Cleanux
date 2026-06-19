export interface FileEntity {
  name?: string;
  path: string;
  size: number;
  modified?: string;
}

export interface FileItem extends FileEntity {
  deletedDate?: string;
  fileCount?: number;
}

export interface OperationResult<T = number> {
  removed: T;
  failed: string[];
  repaired?: number;
  cleanedPaths?: string[];
}

export interface CleaningResult {
  removed: number | string[];
  failed: string[];
  repaired?: number;
  cleanedPaths?: string[];
}

export type TabDataAccessor<T> = () => T[];
export interface TabConfig<T> {
  id: string;
  label: string;
  dataAccessor: TabDataAccessor<T>;
  filteredAccessor: TabDataAccessor<T>;
  selectionAccessor: () => Set<string>;
}
