/* sys lib */
import { ChangeDetectionStrategy, Component, input, computed } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';

/* other */
import { ThemeService } from '@services/theme.service';

@Component({
  selector: 'app-stat-card',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule],
  templateUrl: './stat-card.component.html',
})
export class StatCardComponent {
  label = input.required<string>();
  value = input.required<string | number>();
  icon = input.required<string>();
  accentGradient = input<string>();

  protected computedGradient = computed(
    () => this.accentGradient() || this.themeService.getAccentGradient()
  );

  constructor(private themeService: ThemeService) {}
}
