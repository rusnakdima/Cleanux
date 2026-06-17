use crate::utils::home_dir;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonPath {
  Trash,
  TrashFiles,
  MozillaCache,
  ChromeCache,
  BraveCache,
  EdgeCache,
  Thumbnails,
  FlatpakCache,
  FlatpakAlt,
  SnapCache,
  SnapAlt,
  AppImageCache,
  SteamShaderCache,
  SteamDownloadCache,
  SteamRoot,
  SpotifyCache,
  SpotifyLocalShare,
  VlcCache,
  VlcConfig,
  IconCache,
}

impl CommonPath {
  pub fn path(&self) -> Option<PathBuf> {
    let home = home_dir().ok()?;
    Some(match self {
      CommonPath::Trash => home.join(".local/share/Trash"),
      CommonPath::TrashFiles => home.join(".local/share/Trash/files"),
      CommonPath::MozillaCache => home.join(".cache/mozilla"),
      CommonPath::ChromeCache => home.join(".cache/google-chrome"),
      CommonPath::BraveCache => home.join(".cache/Brave"),
      CommonPath::EdgeCache => home.join(".cache/microsoft-edge"),
      CommonPath::Thumbnails => home.join(".cache/thumbnails"),
      CommonPath::FlatpakCache => home.join(".cache/flatpak"),
      CommonPath::FlatpakAlt => home.join(".var/app"),
      CommonPath::SnapCache => home.join("snap"),
      CommonPath::SnapAlt => home.join(".cache/snap"),
      CommonPath::AppImageCache => home.join(".cache/appimage"),
      CommonPath::SteamShaderCache => home.join(".steam/steam/steamapps/shader"),
      CommonPath::SteamDownloadCache => home.join(".local/share/Steam"),
      CommonPath::SteamRoot => home.join(".steam/steam"),
      CommonPath::SpotifyCache => home.join(".cache/spotify"),
      CommonPath::SpotifyLocalShare => home.join(".local/share/spotify"),
      CommonPath::VlcCache => home.join(".cache/vlc"),
      CommonPath::VlcConfig => home.join(".config/vlc"),
      CommonPath::IconCache => home.join(".cache/icons"),
    })
  }

  pub fn exists(&self) -> bool {
    self.path().map(|p| p.exists()).unwrap_or(false)
  }

  pub fn to_path_buf(&self) -> PathBuf {
    self.path().unwrap_or_default()
  }
}
