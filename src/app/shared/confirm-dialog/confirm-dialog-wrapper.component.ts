import { Component, inject, ChangeDetectionStrategy } from '@angular/core';
import { ConfirmDialogService } from './confirm-dialog.service';
import { ConfirmDialogComponent } from './confirm-dialog.component';

@Component({
  selector: 'app-confirm-dialog-wrapper',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [ConfirmDialogComponent],
  template: `
    @if (dialogService.dialogOpen()) {
      <app-confirm-dialog
        [config]="dialogService.dialogConfig()"
        (confirm)="dialogService.resolve(true)"
        (cancel)="dialogService.resolve(false)"
      />
    }
  `,
})
export class ConfirmDialogWrapperComponent {
  dialogService = inject(ConfirmDialogService);
}
