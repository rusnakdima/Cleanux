/* sys lib */
import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { toSignal } from '@angular/core/rxjs-interop';

/* env */
import { environment } from '@env/environment';

/* models */
import { GitHubReleaseByTag, GitHubReleaseLatest } from '@entities/github-release.model';

const httpOptions = {
  headers: new HttpHeaders({
    'Content-Type': 'application/json',
  }),
};

@Injectable({
  providedIn: 'root',
})
export class AboutService {
  private http = inject(HttpClient);

  gitRepoName: string = environment.gitRepoName;
  githubUser: string = environment.githubUser;

  constructor() {}

  getDate(version: string) {
    return toSignal(
      this.http.get<GitHubReleaseByTag>(
        `https://api.github.com/repos/${this.githubUser}/${this.gitRepoName}/releases/tags/v${version}`,
        httpOptions
      )
    );
  }

  checkUpdate() {
    return toSignal(
      this.http.get<GitHubReleaseLatest>(
        `https://api.github.com/repos/${this.githubUser}/${this.gitRepoName}/releases/latest`,
        httpOptions
      )
    );
  }
}
