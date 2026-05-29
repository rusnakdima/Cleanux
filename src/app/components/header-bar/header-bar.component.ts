/* sys lib */
import { ChangeDetectionStrategy, Component, input, output } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';

@Component({
  selector: 'app-header-bar',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule],
  templateUrl: './header-bar.component.html',
})
export class HeaderBarComponent {
  title = input('');
  breadcrumb = input('');
  showBack = input(false);

  backClick = output<void>();
}
