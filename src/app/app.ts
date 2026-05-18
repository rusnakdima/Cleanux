/* sys lib */
import { Component, signal, inject, effect } from '@angular/core';
import { DOCUMENT } from '@angular/common';
import { RouterOutlet } from '@angular/router';

/* components */
import { SidebarComponent } from '@components/sidebar/sidebar.component';

/* services */
import { ThemeService } from '@services/theme.service';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, SidebarComponent],
  templateUrl: './app.html',
})
export class App {
  private document = inject(DOCUMENT);
  private themeService = inject(ThemeService);

  constructor() {
    this.themeService.applyTheme(this.themeService.currentTheme());
  }
}
