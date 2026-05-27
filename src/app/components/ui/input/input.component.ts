import { Component, input, forwardRef, signal } from '@angular/core';
import { ControlValueAccessor, NG_VALUE_ACCESSOR } from '@angular/forms';

@Component({
  selector: 'app-input',
  standalone: true,
  template: `
    <input [type]="type()"
           [placeholder]="placeholder()"
           [disabled]="disabled()"
           [value]="value()"
           (input)="onInput($event)"
           (blur)="onTouched()"
           [class]="getClasses()" />
  `,
  providers: [{
    provide: NG_VALUE_ACCESSOR,
    useExisting: forwardRef(() => InputComponent),
    multi: true
  }],
  styles: [`
    :host { display: block; }
  `]
})
export class InputComponent implements ControlValueAccessor {
  type = input('text');
  placeholder = input('');
  disabled = input(false);

  value = signal('');
  isDisabled = input(false);

  onChange: (val: string) => void = () => {};
  onTouched: () => void = () => {};

  writeValue(val: string): void { this.value.set(val); }
  registerOnChange(fn: (val: string) => void): void { this.onChange = fn; }
  registerOnTouched(fn: () => void): void { this.onTouched = fn; }

  onInput(event: Event): void {
    this.value.set((event.target as HTMLInputElement).value);
    this.onChange(this.value());
  }

  getClasses(): string {
    return 'w-full px-4 py-2.5 rounded-lg bg-[var(--bg-secondary)] border border-[var(--border-color)] text-[var(--text-primary)] placeholder-[var(--text-muted)] focus:outline-none focus:border-[var(--accent-color)] focus:ring-2 focus:ring-[var(--accent-color)]/20 transition-all duration-150 disabled:opacity-50';
  }
}