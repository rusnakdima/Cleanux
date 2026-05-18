import { Component, Input, Output, EventEmitter, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatDialogModule, MatDialog } from '@angular/material/dialog';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { ApiService } from '@services/api.service';

export interface EmptyDirectory {
  path: string;
  depth: number;
  parent: string | null;
}

@Component({
  selector: 'app-empty-dir-cleaner',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    MatIconModule,
    MatButtonModule,
    MatCheckboxModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    MatDialogModule,
    MatInputModule,
    MatFormFieldModule,
  ],
  templateUrl: './empty-dir-cleaner.component.html',
  styleUrl: './empty-dir-cleaner.component.css',
})
export class EmptyDirCleanerComponent {
  @Input() rootPath = '';
  @Output() close = new EventEmitter<void>();

  emptyDirs = signal<EmptyDirectory[]>([]);
  selectedPaths = signal<Set<string>>(new Set());
  loading = signal(false);
  scanning = signal(false);
  error = signal<string | null>(null);
  confirmDialogOpen = signal(false);

  allSelected = signal(false);
  indeterminate = signal(false);

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
      this.emptyDirs.set(dirs);
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
      this.emptyDirs.set(dirs);
    } catch (e) {
      this.error.set(
        e instanceof Error ? e.message : 'Failed to scan for nested empty directories'
      );
    } finally {
      this.scanning.set(false);
    }
  }

  toggleSelectAll(checked: boolean): void {
    if (checked) {
      const allKeys = new Set(this.emptyDirs().map((d) => d.path));
      this.selectedPaths.set(allKeys);
    } else {
      this.selectedPaths.set(new Set());
    }
    this.updateSelectionState();
  }

  toggleSelectItem(path: string, checked: boolean): void {
    const current = new Set(this.selectedPaths());
    if (checked) {
      current.add(path);
    } else {
      current.delete(path);
    }
    this.selectedPaths.set(current);
    this.updateSelectionState();
  }

  updateSelectionState(): void {
    const total = this.emptyDirs().length;
    const selected = this.selectedPaths().size;

    if (selected === 0) {
      this.allSelected.set(false);
      this.indeterminate.set(false);
    } else if (selected === total) {
      this.allSelected.set(true);
      this.indeterminate.set(false);
    } else {
      this.allSelected.set(false);
      this.indeterminate.set(true);
    }
  }

  isSelected(path: string): boolean {
    return this.selectedPaths().has(path);
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
