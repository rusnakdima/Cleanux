import { Component, input } from '@angular/core';

@Component({
  selector: 'app-button',
  standalone: true,
  template: `
    <button [type]="type()"
            [disabled]="disabled()"
            [class]="getClasses()">
      <ng-content></ng-content>
    </button>
  `,
  styles: [`
    :host { display: inline-block; }
  `]
})
export class ButtonComponent {
  variant = input<'primary' | 'secondary' | 'ghost' | 'danger'>('primary');
  size = input<'sm' | 'md' | 'lg'>('md');
  disabled = input(false);
  type = input<'button' | 'submit'>('button');

  getClasses(): string {
    const base = 'inline-flex items-center justify-center font-medium rounded-lg transition-all duration-150 disabled:opacity-50 disabled:cursor-not-allowed';
    const sizes = {
      sm: 'px-3 py-1.5 text-sm gap-1.5',
      md: 'px-4 py-2 text-sm gap-2',
      lg: 'px-6 py-3 text-base gap-2'
    };
    const variants = {
      primary: 'bg-[var(--accent-color)] text-white hover:opacity-90 shadow-glow',
      secondary: 'border-2 border-[var(--accent-color)] text-[var(--accent-color)] hover:bg-[var(--accent-color)]/10',
      ghost: 'text-[var(--text-secondary)] hover:bg-[var(--bg-secondary)] hover:text-[var(--text-primary)]',
      danger: 'bg-red-500 text-white hover:bg-red-600'
    };
    return `${base} ${sizes[this.size()]} ${variants[this.variant()]}`;
  }
}