/* sys lib */
import { ChangeDetectionStrategy, Component, input, output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';

@Component({
  selector: 'app-path-input',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, MatProgressSpinnerModule],
  templateUrl: './path-input.component.html',
})
export class PathInputComponent {
  placeholder = input<string>('Enter directory path to scan');
  loading = input<boolean>(false);
  path = input<string>('');

  scan = output<string>();

  onScan(): void {
    if (this.path()) {
      this.scan.emit(this.path());
    }
  }
}
