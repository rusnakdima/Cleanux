import { Component, input } from '@angular/core';

@Component({
  selector: 'app-card',
  standalone: true,
  template: `
    <div [class]="getClasses()">
      <ng-content></ng-content>
    </div>
  `,
  styles: [`
    :host { display: block; }
  `]
})
export class CardComponent {
  variant = input<'glass' | 'solid'>('glass');
  padding = input<'sm' | 'md' | 'lg'>('md');

  getClasses(): string {
    const base = 'rounded-xl transition-all duration-150';
    const glass = 'glass-card';
    const solid = 'bg-[var(--bg-card-solid)] border border-[var(--border-color)] shadow-sm hover:shadow-md';
    const paddings = {
      sm: 'p-4',
      md: 'p-5',
      lg: 'p-6'
    };
    const variant = this.variant() === 'glass' ? glass : solid;
    return `${variant} ${paddings[this.padding()]}`;
  }
}