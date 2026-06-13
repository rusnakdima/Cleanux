/* sys lib */
import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { toSignal } from '@angular/core/rxjs-interop';

/* env */
import { environment } from '@env/environment';

/* models */
import { GitHubReleaseByTag, GitHubReleaseLatest } from '@models/github-release.model';
import { LoggerService } from '@services/logger.service';

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
  private logger = inject(LoggerService);

  gitRepoName: string = environment.gitRepoName;
  githubUser: string = environment.githubUser;

  constructor() {
    this.logger.logInfo('service', 'AboutService', 'init', 'AboutService initialized');
  }

  getDate(version: string) {
    this.logger.logInfo('service', 'AboutService', 'getDate', 'Getting release date', { version });
    return toSignal(
      this.http.get<GitHubReleaseByTag>(
        `https://api.github.com/repos/${this.githubUser}/${this.gitRepoName}/releases/tags/v${version}`,
        httpOptions
      )
    );
  }

  checkUpdate() {
    this.logger.logInfo('service', 'AboutService', 'checkUpdate', 'Checking for updates');
    return toSignal(
      this.http.get<GitHubReleaseLatest>(
        `https://api.github.com/repos/${this.githubUser}/${this.gitRepoName}/releases/latest`,
        httpOptions
      )
    );
  }
}
