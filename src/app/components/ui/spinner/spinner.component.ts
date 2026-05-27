import { Component, input } from '@angular/core';

@Component({
  selector: 'app-spinner',
  standalone: true,
  template: `
    <div [class]="getClasses()" role="status" aria-label="Loading">
      <span class="sr-only">Loading...</span>
    </div>
  `,
  styles: [`
    :host { display: inline-block; }
    .spinner {
      width: v-bind("size() + 'px'");
      height: v-bind("size() + 'px'");
      border: 2px solid var(--border-color);
      border-top-color: var(--accent-color);
      border-radius: 50%;
      animation: spin 0.8s linear infinite;
    }
    @keyframes spin {
      to { transform: rotate(360deg); }
    }
  `]
})
export class SpinnerComponent {
  size = input(24);

  getClasses(): string {
    return 'spinner';
  }
}