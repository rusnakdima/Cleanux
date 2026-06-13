import { Component, input, inject, ChangeDetectionStrategy, HostListener } from '@angular/core';
import { DialogService } from './dialog-wrapper.models';

@Component({
  selector: 'app-dialog-wrapper',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    @if (service().isOpen()) {
      <ng-content></ng-content>
    }
  `,
})
export class DialogWrapperComponent {
  service = input.required<DialogService>();

  @HostListener('document:keydown.escape', ['$event'])
  onEscapeKey(event: Event): void {
    if (this.service().isOpen()) {
      event.preventDefault();
      this.service().close();
    }
  }
}
