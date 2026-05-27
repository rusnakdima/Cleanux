import { Component, input } from '@angular/core';

@Component({
  selector: 'app-badge',
  standalone: true,
  template: `<span [class]="getClasses()"><ng-content></ng-content></span>`,
  styles: [`
    :host { display: inline-block; }
  `]
})
export class BadgeComponent {
  color = input<'accent' | 'success' | 'warning' | 'error'>('accent');
  size = input<'sm' | 'md'>('sm');

  getClasses(): string {
    const base = 'inline-flex items-center font-medium rounded-full';
    const sizes = { sm: 'px-2 py-0.5 text-xs', md: 'px-3 py-1 text-sm' };
    const colors = {
      accent: 'bg-[var(--accent-color)]/10 text-[var(--accent-color)]',
      success: 'bg-green-500/10 text-green-600 dark:text-green-400',
      warning: 'bg-yellow-500/10 text-yellow-600 dark:text-yellow-400',
      error: 'bg-red-500/10 text-red-600 dark:text-red-400'
    };
    return `${base} ${sizes[this.size()]} ${colors[this.color()]}`;
  }
}