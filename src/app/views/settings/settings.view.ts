/* sys lib */
import { Component, signal } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';

/* forms */
import { FormsModule } from '@angular/forms';

/* env */
import { environment } from '@env/environment';

/* services */
import { AboutService } from '@services/about.service';

@Component({
  selector: 'app-settings',
  standalone: true,
  imports: [CommonModule, MatIconModule, FormsModule],
  templateUrl: './settings.view.html',
})
export class SettingsView {
  constructor(private aboutService: AboutService) { }

  deepScan = signal(false);
  autoClean = signal(false);

  version = environment.version;
  nameProduct = environment.nameProduct;
  yearCreate = environment.yearCreate;
  companyName = environment.companyName;

  dateVersion = signal(localStorage['dateVersion'] || 'Unknown');
  dateCheck = signal(localStorage['dateCheck'] || 'Unknown');

  isChecking = signal(false);
  isUpdateAvailable = signal(false);
  lastVersion = signal('');

  toggleDeepScan() {
    this.deepScan.update(v => !v);
  }

  toggleAutoClean() {
    this.autoClean.update(v => !v);
  }

  formatDate(date: string): string {
    return new Date(date).toISOString().split('T')[0];
  }

  matchVersion(lastVer: string): boolean {
    const v1Components = lastVer.replace('v', '').split('.').map(Number);
    const v2Components = this.version.split('.').map(Number);

    for (let i = 0; i < Math.max(v1Components.length, v2Components.length); i++) {
      const v1Value = v1Components[i] || 0;
      const v2Value = v2Components[i] || 0;

      if (v1Value < v2Value) {
        return false;
      } else if (v1Value > v2Value) {
        return true;
      }
    }

    return false;
  }

  getDate() {
    this.aboutService.getDate(this.version).subscribe({
      next: (res: any) => {
        if (res && res.published_at) {
          localStorage['dateVersion'] = String(this.formatDate(res.published_at));
          this.dateVersion.set(String(this.formatDate(res.published_at)));
        }
      },
      error: () => { },
    });
  }

  checkUpdate() {
    this.isChecking.set(true);
    localStorage['dateCheck'] = String(this.formatDate(new Date().toUTCString()));
    this.dateCheck.set(localStorage['dateCheck']);

    this.aboutService.checkUpdate().subscribe({
      next: (res: any) => {
        if (res && res.tag_name) {
          const ver: string = res.tag_name;
          setTimeout(() => {
            if (this.matchVersion(ver)) {
              this.isUpdateAvailable.set(true);
              this.lastVersion.set(ver);
              alert(`A new version ${ver} is available!`);
            } else {
              alert('You have the latest version!');
            }
            this.isChecking.set(false);
          }, 1000);
        } else {
          this.isChecking.set(false);
        }
      },
      error: () => {
        this.isChecking.set(false);
        alert('Failed to check for updates');
      },
    });
  }
}
