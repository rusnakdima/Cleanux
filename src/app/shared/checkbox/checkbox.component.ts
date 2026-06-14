import {
  Component,
  Input,
  Output,
  EventEmitter,
  ChangeDetectionStrategy,
  signal,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';
import { MatTooltipModule } from '@angular/material/tooltip';

@Component({
  selector: 'app-checkbox',
  standalone: true,
  imports: [CommonModule, MatIconModule, MatTooltipModule],
  templateUrl: './checkbox.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CheckboxComponent {
  @Input() checked = false;
  @Input() indeterminate = false;
  @Input() disabled = false;
  @Input() tooltip = '';

  @Output() changed = new EventEmitter<boolean>();

  onClick(event: MouseEvent): void {
    if (this.disabled) return;
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
    if (this.disabled) return;
    const newValue = !this.checked;
    this.checked = newValue;
    this.indeterminate = false;
    this.changed.emit(newValue);
  }
}
