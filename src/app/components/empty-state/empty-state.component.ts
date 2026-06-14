/* sys lib */
import { ChangeDetectionStrategy, Component, input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';

@Component({
  selector: 'app-empty-state',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule],
  templateUrl: './empty-state.component.html',
})
export class EmptyStateComponent {
  icon = input<string>('info');
  title = input.required<string>();
  description = input<string>('');
}
