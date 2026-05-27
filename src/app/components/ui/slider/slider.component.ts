import { Component, input, output, forwardRef, signal } from '@angular/core';
import { ControlValueAccessor, NG_VALUE_ACCESSOR } from '@angular/forms';

@Component({
  selector: 'app-slider',
  standalone: true,
  template: `
    <div class="flex items-center gap-3">
      <input type="range"
             [min]="min()"
             [max]="max()"
             [step]="step()"
             [value]="value()"
             (input)="onInput($event)"
             (blur)="onTouched()"
             class="w-full h-2 rounded-full appearance-none cursor-pointer bg-[var(--bg-secondary)] accent-[var(--accent-color)]" />
      <span class="text-sm text-[var(--text-secondary)] min-w-[3rem] text-right">{{ value() }}{{ suffix() }}</span>
    </div>
  `,
  providers: [{
    provide: NG_VALUE_ACCESSOR,
    useExisting: forwardRef(() => SliderComponent),
    multi: true
  }]
})
export class SliderComponent implements ControlValueAccessor {
  min = input(0);
  max = input(100);
  step = input(1);
  suffix = input('');

  value = signal(50);

  onChange: (val: number) => void = () => {};
  onTouched: () => void = () => {};

  writeValue(val: number): void { this.value.set(val); }
  registerOnChange(fn: (val: number) => void): void { this.onChange = fn; }
  registerOnTouched(fn: () => void): void { this.onTouched = fn; }

  onInput(event: Event): void {
    this.value.set(+(event.target as HTMLInputElement).value);
    this.onChange(this.value());
  }
}