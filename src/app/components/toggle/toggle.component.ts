/* sys lib */
import { ChangeDetectionStrategy, Component, input, output } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-toggle',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  templateUrl: './toggle.component.html',
  styleUrl: './toggle.component.css',
})
export class ToggleComponent {
  checked = input(false);
  disabled = input(false);
  size = input<'sm' | 'md' | 'lg'>('md');
  label = input('');
  labelPosition = input<'left' | 'right'>('left');

  changed = output<boolean>();

  onToggle() {
    if (!this.disabled()) {
      this.changed.emit(!this.checked());
    }
  }
}
