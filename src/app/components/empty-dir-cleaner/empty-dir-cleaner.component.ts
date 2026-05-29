import { Component, Input, Output, EventEmitter, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatDialogModule, MatDialog } from '@angular/material/dialog';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { ApiService } from '@services/api.service';
import { DataTableComponent } from '@components/data-table/data-table.component';
import { TableColumn, TableOptions } from '@models/data-table.model';

export interface EmptyDirectory {
  path: string;
  depth: number;
  parent: string | null;
}

export interface FlattenedDir extends EmptyDirectory {
  name: string;
}

@Component({
  selector: 'app-empty-dir-cleaner',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    MatIconModule,
    MatButtonModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    MatDialogModule,
    MatInputModule,
    MatFormFieldModule,
    DataTableComponent,
  ],
  templateUrl: './empty-dir-cleaner.component.html',
})
export class EmptyDirCleanerComponent {
  @Input() rootPath = '';
  @Output() close = new EventEmitter<void>();

  emptyDirs = signal<FlattenedDir[]>([]);
  selectedPaths = signal<Set<string>>(new Set());
  loading = signal(false);
  scanning = signal(false);
  error = signal<string | null>(null);
  confirmDialogOpen = signal(false);

  columns: TableColumn[] = [
    { key: 'name', label: 'Name', sortable: true, resizable: true, minWidth: '150px' },
    { key: 'path', label: 'Path', sortable: true, resizable: true, minWidth: '300px' },
    {
      key: 'depth',
      label: 'Depth',
      align: 'right',
      sortable: true,
      resizable: true,
      minWidth: '80px',
    },
  ];

  tableOptions: TableOptions = {
    showHeader: true,
    showCheckbox: true,
    checkboxKey: 'path',
    hoverable: true,
    showReloadButton: false,
    showSelectedActions: false,
    showPreviewButton: false,
    showSearch: true,
    searchPlaceholder: 'Search directories...',
    virtualScroll: true,
    rowHeight: 48,
  };

  constructor(
    private api: ApiService,
    private dialog: MatDialog
  ) {}

  async scan(): Promise<void> {
    if (!this.rootPath) {
      this.error.set('No path specified');
      return;
    }
    this.scanning.set(true);
    this.error.set(null);
    this.emptyDirs.set([]);
    this.selectedPaths.set(new Set());

    try {
      const dirs = await this.api.invoke<EmptyDirectory[]>('find_empty_directories', {
        path: this.rootPath,
      });
      const flattened: FlattenedDir[] = dirs.map((d) => ({
        ...d,
        name: this.getDirName(d.path),
      }));
      this.emptyDirs.set(flattened);
    } catch (e) {
      this.error.set(e instanceof Error ? e.message : 'Failed to scan directory');
    } finally {
      this.scanning.set(false);
    }
  }

  async scanNested(): Promise<void> {
    if (!this.rootPath) {
      this.error.set('No path specified');
      return;
    }
    this.scanning.set(true);
    this.error.set(null);
    this.emptyDirs.set([]);
    this.selectedPaths.set(new Set());

    try {
      const dirs = await this.api.invoke<EmptyDirectory[]>('find_nested_empty_directories', {
        path: this.rootPath,
      });
      const flattened: FlattenedDir[] = dirs.map((d) => ({
        ...d,
        name: this.getDirName(d.path),
      }));
      this.emptyDirs.set(flattened);
    } catch (e) {
      this.error.set(
        e instanceof Error ? e.message : 'Failed to scan for nested empty directories'
      );
    } finally {
      this.scanning.set(false);
    }
  }

  onSelectionChange(keys: Set<string>): void {
    this.selectedPaths.set(keys);
  }

  get selectedCount(): number {
    return this.selectedPaths().size;
  }

  get totalCount(): number {
    return this.emptyDirs().length;
  }

  openConfirmDialog(): void {
    this.confirmDialogOpen.set(true);
  }

  closeConfirmDialog(): void {
    this.confirmDialogOpen.set(false);
  }

  async removeSelected(): Promise<void> {
    this.confirmDialogOpen.set(false);
    this.loading.set(true);
    this.error.set(null);

    const paths = Array.from(this.selectedPaths());

    try {
      const result = await this.api.invoke<{ removed: number; failed: string[]; total: number }>(
        'remove_empty_directories',
        { paths }
      );

      const remaining = this.emptyDirs().filter((d) => !paths.includes(d.path));
      this.emptyDirs.set(remaining);
      this.selectedPaths.set(new Set());

      if (result.failed.length > 0) {
        this.error.set(`Removed ${result.removed}, failed: ${result.failed.join(', ')}`);
      }
    } catch (e) {
      this.error.set(e instanceof Error ? e.message : 'Failed to remove directories');
    } finally {
      this.loading.set(false);
    }
  }

  getDirName(path: string): string {
    return path.split('/').pop() || path;
  }

  onClose(): void {
    this.close.emit();
  }
}
