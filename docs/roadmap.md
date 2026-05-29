# Cleanux Roadmap

**Version:** 0.3.0-rev2
**Updated:** May 2026
**License:** MIT

---

## Overview

Cleanux is a Linux system cleaning and optimization utility built with Tauri v2 (Angular 21 frontend + Rust backend).

### Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Angular 21.1.0 (Standalone, Signals, OnPush) |
| Backend | Rust, Tauri 2.11.2 |
| Build | Bun 1.3.5 |
| State | Angular Signals |
| Styling | TailwindCSS 4, DaisyUI 5, SCSS |
| i18n | 10 languages (EN, ES, FR, DE, RU, ZH, JA, KO, PT-BR, IT) |

---

## Implemented Features (33+)

### Core Cleaning (6)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 1 | **Dashboard** | System overview with health score, quick actions, and recent activity | ✅ |
| 2 | **Quick Cleaner** | One-click cleaning of cache, trash, and logs | ✅ |
| 3 | **Cache Cleaner** | Clear browser, system, and app caches | ✅ |
| 4 | **Trash Cleaner** | Empty trash and removed files | ✅ |
| 5 | **Log Manager** | View, analyze, and clean system logs | ✅ |
| 6 | **Log Rotation** | Manage journald and logrotate settings | ✅ |

### File Analysis (4)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 7 | **Large Files Finder** | Find files larger than specified size | ✅ |
| 8 | **Duplicate Finder** | Find duplicate files using content hashing | ✅ |
| 9 | **Disk Usage Analyzer** | Visual breakdown of disk space usage | ✅ |
| 10 | **Files Explorer** | Browse and manage files | ✅ |

### System Tools (6)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 11 | **Process Manager** | View and manage running processes | ✅ |
| 12 | **Service Manager** | Control systemd services | ✅ |
| 13 | **Startup Manager** | Manage boot-time applications | ✅ |
| 14 | **Kernel Cleaner** | Remove old kernels safely | ✅ |
| 15 | **System Repair** | Fix common system issues | ✅ |
| 16 | **App Residue Cleaner** | Remove leftover files after uninstall | ✅ |

### Performance (4)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 17 | **Memory Optimizer** | Clear memory and optimize RAM usage | ✅ |
| 18 | **Battery & Power** | Power profiles and battery health | ✅ |
| 19 | **Real-time Monitor** | CPU, memory, temperature monitoring | ✅ |
| 20 | **System Information** | Detailed hardware/software information | ✅ |

### Development Tools (4)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 21 | **Dev Cache Cleaner** | Clear npm, pip, cargo, go, maven, gradle caches | ✅ |
| 22 | **Container Cleaner** | Clean Docker/Podman images, containers, volumes | ✅ |
| 23 | **Media App Cleaner** | Clean Steam, Spotify, VLC, thumbnails caches | ✅ |
| 24 | **Package Deep Clean** | Deep clean apt, dnf, pacman, zypper | ✅ |

### Automation & Data (4)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 25 | **Cleaning Profiles** | Save and load custom cleaning configurations | ✅ |
| 26 | **Automation/Recipes** | Schedule automatic cleaning tasks | ✅ |
| 27 | **Backup & Restore** | Backup cleaning profiles and settings | ✅ |
| 28 | **Health History** | Track system health over time (SQLite) | ✅ |

### Insights (3)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 29 | **Reports & Analytics** | View cleaning history and space saved | ✅ |
| 30 | **Dashboard Widgets** | System monitor widgets | ✅ |
| 31 | **Automation Recipes** | Predefined cleaning workflows | ✅ |

### User Experience (4)

| # | Feature | Description | Status |
|---|---------|-------------|--------|
| 32 | **i18n (10 languages)** | Full translation support | ✅ |
| 33 | **Theme Support** | Dark and light modes | ✅ |
| 34 | **Settings** | App preferences and configuration | ✅ |
| 35 | **Keyboard Shortcuts** | Quick access to common actions | ✅ |

---

## Upcoming Features

### v0.3.1 (Q3 2026)

| # | Feature | Priority | Description |
|---|---------|----------|-------------|
| 36 | **Network Cleaner** | HIGH | Browser history, cookies, DNS cache |
| 37 | **Secure Delete** | HIGH | Secure file deletion with shred |
| 38 | **Startup Profiler** | MEDIUM | Analyze boot time, suggest optimizations |

### v0.3.2 (Q4 2026)

| # | Feature | Priority | Description |
|---|---------|----------|-------------|
| 39 | **System Backup** | MEDIUM | Full system backup via Timeshift/Borg |
| 40 | **Resource Monitor Widget** | MEDIUM | Advanced CPU/Memory/Network widgets |
| 41 | **Custom Recipes** | LOW | User-defined cleaning operations |

### v0.4.0 (Q1 2027)

| # | Feature | Priority | Description |
|---|---------|----------|-------------|
| 42 | **AUR Cleaner** | LOW | Arch User Repository cleanup |
| 43 | **Flatpak Cleaner** | LOW | Flatpak orphaned data |
| 44 | **Snapshot Manager** | MEDIUM | Timeshift, Snapper, Btrfs snapshots |

---

## Version History

| Version | Date | Features | Notes |
|---------|------|----------|-------|
| v0.3.0 | May 2026 | 33+ | Refactor release + code quality maintenance |
| v0.2.1 | April 2026 | 33 | Phase 3 cleaning features complete |
| v0.2.0 | April 2026 | 25 | Phase 2 features, dev/container cleaners |
| v0.1.0 | March 2026 | 17 | Core features, dashboard, basic cleaning |

---

## Architecture

```
cleanux/
├── src/                          # Angular 21 frontend
│   ├── app/
│   │   ├── components/           # Reusable UI components
│   │   ├── views/               # Page components (31 views)
│   │   ├── services/            # 29 Angular services
│   │   ├── stores/              # 4 state stores (signals)
│   │   ├── models/              # TypeScript interfaces (19 models)
│   │   ├── guards/              # Route guards
│   │   ├── interceptors/        # HTTP interceptors (3)
│   │   └── i18n/                # 10 languages
│   ├── styles/                  # SCSS + Tailwind
│   └── assets/                  # Icons, images
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── services/            # 37 business logic services
│   │   ├── routes/              # 30 route handlers
│   │   ├── models/              # Data models
│   │   ├── helpers/             # 5 helper modules
│   │   ├── errors/              # Error types
│   │   ├── security/            # Path validation & allowlisting
│   │   └── middleware/          # Request/response middleware
│   └── tests/                   # Integration tests (24 tests)
└── docs/                         # Documentation
```

---

## Code Quality Status

**All critical code quality issues have been resolved:**

| Metric | Status |
|--------|--------|
| Duplicate type definitions | ✅ 0 |
| Unused functions (Rust) | ✅ 0 |
| Duplicate routes | ✅ 0 |
| Unused imports | ✅ 0 |
| clippy warnings | ✅ 0 |
| Angular build errors | ✅ 0 |
| Test failures | ✅ 0 |

See [QUALITY.md](../QUALITY.md) for detailed resolution status.

---

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines.

---

## References

- [QUALITY.md](../QUALITY.md) - Code quality analysis and resolution
- [PLAN.md](./PLAN.md) - Detailed refactoring plan (post-maintenance)
- [ARCHITECTURE.md](../ARCHITECTURE.md) - System architecture documentation
- [CHANGELOG.md](../CHANGELOG.md) - Version history and changes

---

**Last Updated:** May 2026
**Status:** Active Development + Maintenance Complete