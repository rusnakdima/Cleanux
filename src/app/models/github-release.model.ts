/** Minimal GitHub API shapes used by AboutService. */
export interface GitHubReleaseByTag {
  published_at?: string;
  tag_name?: string;
}

export interface GitHubReleaseLatest {
  tag_name?: string;
}
