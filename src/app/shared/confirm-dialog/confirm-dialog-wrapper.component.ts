import { Component, inject, ChangeDetectionStrategy } from '@angular/core';
import { ConfirmDialogService } from './confirm-dialog.service';
import { ConfirmDialogComponent } from './confirm-dialog.component';

@Component({
  selector: 'app-confirm-dialog-wrapper',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [ConfirmDialogComponent],
  templateUrl: './confirm-dialog-wrapper.component.html',
})
export class ConfirmDialogWrapperComponent {
  dialogService = inject(ConfirmDialogService);
}
