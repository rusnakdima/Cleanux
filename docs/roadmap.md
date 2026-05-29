# Cleanux Roadmap

**Version:** 0.2.1
**Updated:** May 2026
**License:** MIT

---

## Overview

Cleanux is a Linux system cleaning and optimization utility built with Tauri v2 (Angular 21 frontend + Rust backend).

### Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Angular 21.1.0 (Standalone, Signals, OnPush) |
| Backend | Rust, Tauri 2.x |
| Build | Bun 1.3.5 |
| State | Angular Signals |
| Styling | TailwindCSS 4 |
| i18n | 9 languages (EN, ES, FR, DE, RU, ZH, JA, KO, PT-BR, IT) |

---

## Feature Roadmap

### Implemented Features (33)

#### Core Cleaning (6)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 1 | **Dashboard** | System overview with health score, quick actions, and recent activity | ✅ |
| 2 | **Quick Cleaner** | One-click cleaning of cache, trash, and logs | ✅ |
| 3 | **Cache Cleaner** | Clear browser, system, and app caches | ✅ |
| 4 | **Trash Cleaner** | Empty trash and removed files | ✅ |
| 5 | **Log Manager** | View, analyze, and clean system logs | ✅ |
| 6 | **Log Rotation** | Manage journald and logrotate settings | ✅ |

#### File Analysis (3)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 7 | **Large Files Finder** | Find files larger than specified size | ✅ |
| 8 | **Duplicate Finder** | Find duplicate files using content hashing | ✅ |
| 9 | **Disk Usage Analyzer** | Visual breakdown of disk space usage | ✅ |

#### System Tools (6)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 10 | **Process Manager** | View and manage running processes | ✅ |
| 11 | **Service Manager** | Control systemd services | ✅ |
| 12 | **Startup Manager** | Manage boot-time applications | ✅ |
| 13 | **Kernel Cleaner** | Remove old kernels safely | ✅ |
| 14 | **System Repair** | Fix common system issues | ✅ |
| 15 | **App Residue Cleaner** | Remove leftover files after uninstall | ✅ |

#### Performance (3)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 16 | **Memory Optimizer** | Clear memory and optimize RAM usage | ✅ |
| 17 | **Battery & Power** | Power profiles and battery health | ✅ |
| 18 | **Real-time Monitor** | CPU, memory, temperature monitoring | ✅ |

#### Development Tools (4)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 19 | **Dev Cache Cleaner** | Clear npm, pip, cargo, go, maven, gradle caches | ✅ |
| 20 | **Container Cleaner** | Clean Docker/Podman images, containers, volumes | ✅ |
| 21 | **Media App Cleaner** | Clean Steam, Spotify, VLC, thumbnails caches | ✅ |
| 22 | **Package Deep Clean** | Deep clean apt, dnf, pacman, zypper | ✅ |

#### Automation & Data (4)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 23 | **Cleaning Profiles** | Save and load custom cleaning configurations | ✅ |
| 24 | **Automation** | Schedule automatic cleaning tasks | ✅ |
| 25 | **Backup & Restore** | Backup cleaning profiles and settings | ✅ |
| 26 | **Health History** | Track system health over time (SQLite) | ✅ |

#### Insights (3)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 27 | **Reports & Analytics** | View cleaning history and space saved | ✅ |
| 28 | **Dashboard Widgets** | Customizable dashboard widgets | ✅ |
| 29 | **System Information** | Detailed hardware/software information | ✅ |

#### User Experience (4)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 30 | **i18n (9 languages)** | Full translation support | ✅ |
| 31 | **Theme Support** | Dark and light modes | ✅ |
| 32 | **Settings** | App preferences and configuration | ✅ |
| 33 | **Keyboard Shortcuts** | Quick access to common actions | ✅ |

---

## Upcoming Features

### v0.3.0 (Q3 2026)

| # | Feature | Priority | Description |
|---|---------|----------|-------------|
| 34 | **Network Cleaner** | HIGH | Browser history, cookies, DNS cache |
| 35 | **Secure Delete** | HIGH | Secure file deletion with shred |
| 36 | **System Backup** | MEDIUM | Full system backup via Timeshift/Borg |

### v0.3.1 (Q4 2026)

| # | Feature | Priority | Description |
|---|---------|----------|-------------|
| 37 | **Startup Profiler** | MEDIUM | Analyze boot time, suggest optimizations |
| 38 | **Resource Monitor Widget** | MEDIUM | Advanced CPU/Memory/Network widgets |
| 39 | **Custom Recipes** | LOW | User-defined cleaning operations |

### v0.4.0 (Q1 2027)

| # | Feature | Priority | Description |
|---|---------|----------|-------------|
| 40 | **AUR Cleaner** | LOW | Arch User Repository cleanup |
| 41 | **Flatpak Cleaner** | LOW | Flatpak orphaned data |
| 42 | **Snapshot Manager** | MEDIUM | Timeshift, Snapper, Btrfs snapshots |

---

## Version History

| Version | Date | Features | Notes |
|---------|------|----------|-------|
| v0.2.1 | May 2026 | 33 | Phase 3 cleaning features complete |
| v0.2.0 | April 2026 | 25 | Phase 2 features, dev/container cleaners |
| v0.1.0 | March 2026 | 17 | Core features, dashboard, basic cleaning |

---

## Architecture

```
cleanux/
├── src/                          # Angular frontend
│   ├── app/
│   │   ├── components/           # Reusable UI components
│   │   ├── views/               # Page components (24 views)
│   │   ├── services/            # Angular services
│   │   ├── stores/              # State management (signals)
│   │   ├── models/             # TypeScript interfaces
│   │   └── i18n/               # 9 languages
│   ├── styles/                  # SCSS + Tailwind
│   └── assets/                  # Icons, images
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── services/            # 35+ business logic services
│   │   ├── routes/              # 30+ route handlers
│   │   ├── models/              # Data models
│   │   ├── helpers/             # Utility functions
│   │   └── security/            # Path validation
│   └── Cargo.toml
└── docs/                         # Documentation
```

---

## Contributing

See [CONTRIBUTING.md]() for development guidelines.

---

**Last Updated:** May 2026
**Status:** Active Development
