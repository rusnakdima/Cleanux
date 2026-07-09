import {
  Component,
  input,
  Output,
  EventEmitter,
  ChangeDetectionStrategy,
  signal,
} from '@angular/core';
import { MatIconModule } from '@angular/material/icon';
import { MatTooltipModule } from '@angular/material/tooltip';

@Component({
  selector: 'app-checkbox',
  standalone: true,
  imports: [MatIconModule, MatTooltipModule],
  templateUrl: './checkbox.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CheckboxComponent {
  checked = input(false);
  indeterminate = input(false);
  disabled = input(false);
  tooltip = input('');

  @Output() changed = new EventEmitter<boolean>();

  _checked = signal(false);
  _indeterminate = signal(false);

  constructor() {
    this._checked.set(this.checked());
    this._indeterminate.set(this.indeterminate());
  }

  onClick(event: MouseEvent): void {
    if (this.disabled()) return;
    event.stopPropagation();
    this.toggle();
  }

  onKeydown(event: KeyboardEvent): void {
    if (event.key === ' ' || event.key === 'Enter') {
      event.preventDefault();
      this.toggle();
    }
  }

  private toggle(): void {
    if (this.disabled()) return;
    const newValue = !this._checked();
    this._checked.set(newValue);
    this._indeterminate.set(false);
    this.changed.emit(newValue);
  }
}
