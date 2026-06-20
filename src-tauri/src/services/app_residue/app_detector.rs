use crate::utils::stdout_string;
use std::collections::HashSet;
use std::process::Command;
pub struct AppDetector;
impl AppDetector {
  pub fn get_installed_apps() -> HashSet<String> {
    let mut installed: HashSet<String> = HashSet::new();
    if let Ok(output) = Command::new("dpkg").arg("-l").output() {
      if output.status.success() {
        let stdout = stdout_string(&output);
        for line in stdout.lines().skip(5) {
          let parts: Vec<&str> = line.split_whitespace().collect();
          if parts.len() >= 4 {
            let status = parts[0];
            let name = parts[3].to_lowercase();
            if status == "ii" {
              installed.insert(name.clone());
              if let Some(descr) = parts.get(4) {
                let alt_name = format!("{}-{}", name, *descr).to_lowercase();
                installed.insert(alt_name);
              }
            }
          }
        }
      }
    }
    if let Ok(output) = Command::new("snap").arg("list").output() {
      if output.status.success() {
        let stdout = stdout_string(&output);
        for line in stdout.lines().skip(1) {
          let parts: Vec<&str> = line.split_whitespace().collect();
          if !parts.is_empty() {
            installed.insert(parts[0].to_lowercase());
          }
        }
      }
    }
    if let Ok(output) = Command::new("flatpak").arg("list").output() {
      if output.status.success() {
        let stdout = stdout_string(&output);
        for line in stdout.lines() {
          let parts: Vec<&str> = line.split('\t').collect();
          if !parts.is_empty() {
            let name = parts[0].to_lowercase();
            installed.insert(name);
            if let Some(last) = parts.last() {
              let alt_name = last.replace('.', "-").to_lowercase();
              installed.insert(alt_name);
            }
          }
        }
      }
    }
    let common_apps = Self::common_apps_list();
    for app in common_apps {
      installed.insert(app.to_string());
    }
    installed
  }
  pub fn common_apps_list() -> &'static [&'static str] {
    &[
      "code",
      "visual-studio-code",
      "firefox",
      "chrome",
      "chromium",
      "brave",
      "opera",
      "vlc",
      "spotify",
      "discord",
      "slack",
      "teams",
      "zoom",
      "skype",
      "telegram",
      "whatsapp",
      "signal",
      "gimp",
      "inkscape",
      "blender",
      "audacity",
      "obs",
      "steam",
      "libreoffice",
      "openoffice",
      "thunderbird",
      "evolution",
      "nautilus",
      "dolphin",
      "konqueror",
      "nemo",
      "pcmanfm",
      "terminal",
      "gnome-terminal",
      "konsole",
      "xfce4-terminal",
      "file-roller",
      "ark",
      "p7zip",
      "gzip",
      "tar",
      "curl",
      "wget",
      "ssh",
      "git",
      "vim",
      "nano",
      "emacs",
      "atom",
      "sublime-text",
      "intellij",
      "pycharm",
      "webstorm",
      "clion",
      "goland",
      "rider",
      "android-studio",
      "eclipse",
      "netbeans",
      "mysql",
      "postgresql",
      "redis",
      "apache",
      "nginx",
      "docker",
      "kubernetes",
      "kubectl",
      "helm",
      "terraform",
      "ansible",
      "vagrant",
      "virtualbox",
    ]
  }
  pub fn normalize_app_name(name: &str) -> String {
    name
      .to_lowercase()
      .replace(['.', '-', '_'], "-")
      .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
  }
  pub fn matches_installed_app(app_name: &str, installed_apps: &HashSet<String>) -> bool {
    let normalized = Self::normalize_app_name(app_name);
    if installed_apps.contains(&normalized) {
      return true;
    }
    for installed in installed_apps {
      if normalized.contains(installed) || installed.contains(&normalized) {
        return true;
      }
      let parts: Vec<&str> = normalized.split('-').collect();
      if parts.len() > 1 {
        let short_name = parts[0];
        if normalized.starts_with(short_name) && installed.contains(short_name) {
          return true;
        }
      }
    }
    false
  }
}
