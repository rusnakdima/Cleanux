import { Component, ChangeDetectionStrategy, input, computed } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-loading-skeleton',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  template: `
    <div
      [class]="getClasses()"
      [style.width]="width() !== '100%' ? width() : variant() === 'circle' ? '40px' : '100%'"
      [style.height]="
        height() ||
        (variant() === 'text'
          ? '1em'
          : variant() === 'circle'
            ? '40px'
            : variant() === 'rect'
              ? '100px'
              : '120px')
      "
      [style.border-radius]="rounded() ? '50%' : variant() === 'card' ? '8px' : '4px'"
    ></div>
  `,
})
export class LoadingSkeletonComponent {
  variant = input<'text' | 'circle' | 'rect' | 'card'>('text');
  width = input<string>('100%');
  height = input<string | null>(null);
  rounded = input<boolean>(false);

  getClasses(): string {
    const base = 'bg-[var(--bg-tertiary)] animate-pulse';
    const variants = {
      text: 'w-full',
      circle: 'w-10 h-10 rounded-full',
      rect: 'w-full h-[100px]',
      card: 'w-full h-[120px] rounded-lg',
    };
    return `${base} ${variants[this.variant()]}`;
  }
}

@Component({
  selector: 'app-loading-placeholder',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, LoadingSkeletonComponent],
  template: `
    <div class="flex flex-col" [style.gap]="gap()">
      @for (item of items(); track trackByIndex($index)) {
        <app-loading-skeleton
          [variant]="variant()"
          [width]="width()"
          [height]="height()"
          [rounded]="rounded()"
        />
      }
    </div>
  `,
})
export class LoadingPlaceholderComponent {
  count = input<number>(3);
  variant = input<'text' | 'circle' | 'rect' | 'card'>('text');
  width = input<string>('100%');
  height = input<string | null>(null);
  rounded = input<boolean>(false);
  gap = input<string>('0.5rem');

  items = computed(() => {
    const c = this.count();
    return Array.from({ length: c }, (_, i) => i);
  });

  trackByIndex(index: number): number {
    return index;
  }
}
