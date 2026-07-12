/* angular */
import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { toSignal } from '@angular/core/rxjs-interop';

/* library */
import { AboutService as LibraryAboutService } from '@tauri-front/shared';

/* env */
import { environment } from '@env/environment';

const httpOptions = {
  headers: new HttpHeaders({
    'Content-Type': 'application/json',
  }),
};

@Injectable({
  providedIn: 'root',
})
export class AboutService extends LibraryAboutService {
  private http = inject(HttpClient);

  constructor() {
    super(environment.nameProduct, environment.githubUser, environment.gitRepoName);
  }

  getDate(version: string) {
    return toSignal(
      this.http.get<{ published_at?: string; tag_name?: string }>(
        `https://api.github.com/repos/${this.owner}/${this.repo}/releases/tags/v${version}`,
        httpOptions
      )
    );
  }
}
