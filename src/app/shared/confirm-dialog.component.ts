import { Component, ChangeDetectionStrategy, signal, inject, Injectable } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatDialogModule, MatDialog } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';

export interface ConfirmDialogConfig {
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  confirmColor?: 'primary' | 'accent' | 'warn';
}

@Component({
  selector: 'app-confirm-dialog',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatDialogModule, MatButtonModule, MatIconModule],
  template: `
    <div class="p-4 min-w-[300px]">
      <h2 mat-dialog-title>{{ config().title }}</h2>
      <mat-dialog-content>
        <p>{{ config().message }}</p>
      </mat-dialog-content>
      <mat-dialog-actions align="end" class="flex gap-2 pt-4">
        <button mat-button (click)="onCancel()">
          {{ config().cancelText || 'Cancel' }}
        </button>
        <button
          mat-raised-button
          [color]="config().confirmColor || 'primary'"
          (click)="onConfirm()"
        >
          {{ config().confirmText || 'Confirm' }}
        </button>
      </mat-dialog-actions>
    </div>
  `,
})
export class ConfirmDialogComponent {
  config = signal<ConfirmDialogConfig>({
    title: 'Confirm',
    message: 'Are you sure?',
    confirmText: 'Yes',
    cancelText: 'No',
    confirmColor: 'primary',
  });

  resolve: ((result: boolean) => void) | null = null;

  open(config: ConfirmDialogConfig): Promise<boolean> {
    this.config.set(config);
    return new Promise((resolve) => {
      this.resolve = resolve;
    });
  }

  onConfirm(): void {
    this.resolve?.(true);
    this.resolve = null;
  }

  onCancel(): void {
    this.resolve?.(false);
    this.resolve = null;
  }
}

@Injectable({
  providedIn: 'root',
})
export class ConfirmDialogService {
  private dialog = inject(MatDialog);

  confirm(config: ConfirmDialogConfig): Promise<boolean> {
    const dialogRef = this.dialog.open(ConfirmDialogComponent, {
      data: config,
      disableClose: true,
    });

    const component = dialogRef.componentInstance as ConfirmDialogComponent;
    return component.open(config);
  }

  confirmDelete(itemName: string): Promise<boolean> {
    return this.confirm({
      title: 'Confirm Delete',
      message: `Are you sure you want to delete "${itemName}"? This action cannot be undone.`,
      confirmText: 'Delete',
      cancelText: 'Cancel',
      confirmColor: 'warn',
    });
  }

  confirmAction(action: string, itemCount: number = 1): Promise<boolean> {
    return this.confirm({
      title: `Confirm ${action}`,
      message: `Are you sure you want to ${action.toLowerCase()} ${itemCount} item(s)?`,
      confirmText: action,
      cancelText: 'Cancel',
      confirmColor: 'warn',
    });
  }
}
