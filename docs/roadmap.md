# Cleanux Project Roadmap

## Table of Contents

1. [Project Overview](#project-overview)
2. [Tech Stack](#tech-stack)
3. [Complete Feature List](#complete-feature-list)
4. [New Cleaning Features 2025](#new-cleaning-features-2025)
5. [Implementation Phases](#implementation-phases)
6. [File Structure](#file-structure)

---

## Project Overview

**Cleanux** is a Linux system cleaning utility built with Tauri v2 (Angular 21 frontend + Rust backend).

**Current Status**: 
- ✅ Original Phase 1-4: 15 features
- ✅ 2025 Features Phase 1: 10 features
- ✅ 2025 Features Phase 2: 8 new cleaning features
- **TOTAL: 33 features implemented**

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Angular 21.1.0 (Standalone, Signals, OnPush) |
| Backend | Rust, Tauri ~2.10 |
| Build | Bun 1.3.5 |
| State | Angular Signals with OnPush |
| Styling | TailwindCSS 4, Glassmorphic UI |
| i18n | 7 languages (EN, ES, FR, DE, RU, ZH, JA) |

---

## Complete Feature List

### ✅ Original Features (15)

| # | Feature | Status | Files |
|---|---------|--------|-------|
| 1 | Dashboard | ✅ | dashboard.service.rs, dashboard.view.ts |
| 2 | Cleaner (Cache/Trash/Logs) | ✅ | cleaner.service.rs, cleaner.view.ts |
| 3 | Large Files Finder | ✅ | scanner.service.rs, large-files.view.ts |
| 4 | System Services Manager | ✅ | system.service.rs, system.view.ts |
| 5 | Duplicate File Finder | ✅ | scanner.service.rs, duplicate-finder.view.ts |
| 6 | Process Manager | ✅ | process.service.rs, processes.view.ts |
| 7 | Startup Manager | ✅ | startup.service.rs, startup.view.ts |
| 8 | Disk Usage Analyzer | ✅ | directory.service.rs, disk-usage.view.ts |
| 9 | Real-time System Monitor | ✅ | monitor.service.rs, system-monitor.component.ts |
| 10 | Temperature Monitoring | ✅ | temperature.service.rs, temperature-widget.component.ts |
| 11 | Scheduled Cleaning | ✅ | scheduler.service.rs, settings.view.ts |
| 12 | Cleaning Profile System | ✅ | profile.service.rs, profiles.view.ts |
| 13 | Backup Before Clean | ✅ | backup.service.rs, backup.view.ts |
| 14 | Health History (SQLite) | ✅ | health_history.service.rs, dashboard.view.ts |
| 15 | Settings & Preferences | ✅ | settings.view.ts |

---

### ✅ 2025 New Features Phase 1 (10)

| # | Feature | Status | Files |
|---|---------|--------|-------|
| 16 | Memory Optimizer | ✅ | memory.service.rs, memory-optimizer.view.ts |
| 17 | Advanced Junk Cleaner | ✅ | junk_cleaner.service.rs, advanced-cleaner.view.ts |
| 18 | App Manager | ✅ | app_manager.service.rs, app-manager.view.ts |
| 19 | Battery & Power | ✅ | power.service.rs, power.view.ts |
| 20 | System Repair | ✅ | repair.service.rs, system-repair.view.ts |
| 21 | Reports & Analytics | ✅ | report.service.rs, reports.view.ts |
| 22 | Automation Features | ✅ | automation.service.rs, automation.view.ts |
| 23 | i18n (7 languages) | ✅ | i18n/*.json, i18n.service.ts |
| 24 | Theme Customization | ✅ | theme.service.ts, settings.view.ts |
| 25 | Dashboard Widgets | ✅ | widget-container.component.ts |

---

### ✅ 2025 New Cleaning Features Phase 2 (8)

| # | Feature | Status | Priority | Description |
|---|---------|--------|----------|-------------|
| 26 | Dev Cache Cleaner | ✅ | HIGH | npm, pip, cargo, go, maven, gradle |
| 27 | Container Cleanup | ✅ | HIGH | Docker/Podman images, containers, volumes |
| 28 | Media App Cleaner | ✅ | HIGH | Steam shader cache, Spotify, VLC, thumbnails |
| 29 | Kernel & Boot Cleanup | ✅ | CRITICAL | Old kernel removal with safety checks |
| 30 | Snapshot Manager | ✅ | HIGH | Timeshift, Snapper, Btrfs snapshots |
| 31 | Log Manager & Rotation | ✅ | MEDIUM | Journal vacuum, rotated logs, logrotate |
| 32 | Package Manager Deep Clean | ✅ | HIGH | apt, dnf, pacman, zypper deep cleaning |
| 33 | App Residue Cleaner | ✅ | MEDIUM | Config/data residue after uninstall |

---

## New Cleaning Features 2025 - Detailed

### 🆕 26. Dev Cache Cleaner ✅

**Path**: `/dev-cleaner`

**Supported Tools**:
| Tool | Cache Path | Size Detection |
|------|------------|----------------|
| NPM | `~/.npm/` | ✅ |
| PIP | `~/.cache/pip/` | ✅ |
| Cargo | `~/.cargo/registry/`, `~/.cargo/git/` | ✅ |
| Go | `~/go/pkg/mod/` | ✅ |
| Maven | `~/.m2/repository/` | ✅ |
| Gradle | `~/.gradle/caches/` | ✅ |

**Backend**: `dev_cache.service.rs`, `dev_cache.route.rs`
**Frontend**: `dev-cleaner.view.ts`, `dev-cache.service.ts`

---

### 🆕 27. Container Cleanup (Docker/Podman) ✅

**Path**: `/container-cleaner`

**Features**:
| Container | Commands | Safety |
|-----------|----------|--------|
| Docker | `docker system prune`, `docker image prune -a`, `docker volume prune` | ✅ --dry-run preview |
| Podman | `podman system prune`, `podman image prune` | ✅ |

**Displayed Info**:
- Images size, containers count, volumes size
- Prune buttons with confirmation
- "Clean All" with multiple confirmations for -a flag

**Backend**: `container.service.rs`, `container.route.rs`
**Frontend**: `container-cleaner.view.ts`, `container.service.ts`

---

### 🆕 28. Media App Cleaner ✅

**Path**: `/media-cleaner`

**Supported Apps**:
| App | Cache Path | Warning |
|-----|------------|---------|
| Steam | `~/.steam/steam/steamapps/shader/` | ⚠️ Close Steam |
| Spotify | `~/.cache/spotify/` | ⚠️ Close Spotify |
| VLC | `~/.cache/vlc/` | Safe |
| Thumbnails | `~/.cache/thumbnails/` | Safe |
| Icons | `~/.cache/fontconfig/` | Safe |

**Backend**: `media_cache.service.rs`, `media_cache.route.rs`
**Frontend**: `media-cleaner.view.ts`, `media-cache.service.ts`

---

### 🆕 29. Kernel & Boot Cleanup ✅ (CRITICAL SAFETY)

**Path**: `/kernel-cleaner`

**Safety Features**:
| Protection | Implementation |
|------------|----------------|
| Never delete current kernel | `uname -r` check |
| Keep 2 kernels minimum | Count verification |
| Keep latest fallback | Version comparison |
| Auto GRUB update | `update-grub` after removal |

**Displayed**:
- Current kernel (protected)
- List of removable old kernels with sizes
- /boot space usage
- "Safe Remove" button

**Backend**: `kernel_cleaner.service.rs`, `kernel_cleaner.route.rs`
**Frontend**: `kernel-cleaner.view.ts`, `kernel-cleaner.service.ts`

---

### 🆕 30. Snapshot Manager (Timeshift/Snapper) ✅ (CRITICAL SAFETY)

**Path**: `/snapshot-manager`

**Supported Tools**:
| Tool | Snapshots Location | Commands |
|------|-------------------|----------|
| Timeshift | `/timeshift/` | `timeshift --delete` |
| Snapper | `/.snapshots/` | `snapper delete` |
| Btrfs | Direct subvolumes | `btrfs subvolume delete` |

**Safety Features**:
| Protection | Implementation |
|------------|----------------|
| Never delete < 3 days old | Date check |
| Keep minimum 3 snapshots | Count verification |
| Multi-confirmation | Type "DELETE" to confirm |

**Backend**: `snapshot.service.rs`, `snapshot.route.rs`
**Frontend**: `snapshot-manager.view.ts`, `snapshot.service.ts`

---

### 🆕 31. Log Manager & Rotation ✅

**Path**: `/log-manager`

**Features**:
| Category | Function | Command |
|----------|----------|---------|
| Journal | Vacuum by size/days | `journalctl --vacuum-size/time` |
| Rotated Logs | Clean old .gz, .old | `find /var/log -name "*.gz"` |
| Logrotate | Config analysis | Parse `/etc/logrotate.conf` |
| /var/log Usage | Largest files | `du -sh /var/log/*` |

**Backend**: `log_manager.service.rs`, `log_manager.route.rs`
**Frontend**: `log-manager.view.ts`, `log-manager.service.ts`

---

### 🆕 32. Package Manager Deep Clean ✅

**Path**: `/package-deep-clean`

**Supported Package Managers**:
| Manager | Commands | Features |
|---------|----------|----------|
| APT/Debian | `apt-get clean`, `apt-get autoremove` | Orphan detection, .part files |
| DNF/YUM | `dnf clean all` | Cache clean |
| Pacman | `pacman -Sc`, `pacman -Scc` | Keep recent, FULL CLEAN warning |
| Zypper | `zypper clean` | Cache clean |

**Backend**: `package_deep_clean.service.rs`, `package_deep_clean.route.rs`
**Frontend**: `package-deep-clean.view.ts`, `package-deep-clean.service.ts`

---

### 🆕 33. App Residue Cleaner ✅

**Path**: `/app-residue-cleaner`

**Features**:
| Type | Location | Detection |
|------|----------|-----------|
| User Configs | `~/.config/` | Compare with installed apps |
| User Data | `~/.local/share/` | Compare with installed apps |
| User Caches | `~/.cache/` | Compare with installed apps |
| System Orphans | `/etc/` | `dpkg -l | grep "^rc"` |

**Safety**: Preview before delete, backup reminder

**Backend**: `app_residue.service.rs`, `app_residue.route.rs`
**Frontend**: `app-residue-cleaner.view.ts`, `app-residue.service.ts`

---

## Implementation Phases

### ✅ Phase 1: Original Features (Completed)
- 15 original features

### ✅ Phase 2: 2025 New Features (Completed)  
- 10 new UI/UX features

### ✅ Phase 3: 2025 Cleaning Features (Completed)
- 8 new system cleaning features

---

## File Structure

```
cleanux/
├── src/
│   ├── app/
│   │   ├── i18n/                    # 7 languages
│   │   │   ├── en.json, es.json, fr.json, de.json, ru.json, zh.json, ja.json
│   │   │   └── i18n.service.ts
│   │   ├── services/
│   │   │   ├── api.service.ts
│   │   │   ├── dev-cache.service.ts              # 🆕
│   │   │   ├── container.service.ts              # 🆕
│   │   │   ├── media-cache.service.ts            # 🆕
│   │   │   ├── kernel-cleaner.service.ts         # 🆕
│   │   │   ├── snapshot.service.ts                # 🆕
│   │   │   ├── log-manager.service.ts             # 🆕
│   │   │   ├── package-deep-clean.service.ts      # 🆕
│   │   │   ├── app-residue.service.ts             # 🆕
│   │   │   └── ... (20+ services total)
│   │   ├── views/
│   │   │   ├── dev-cleaner/                       # 🆕
│   │   │   ├── container-cleaner/                 # 🆕
│   │   │   ├── media-cleaner/                     # 🆕
│   │   │   ├── kernel-cleaner/                    # 🆕
│   │   │   ├── snapshot-manager/                  # 🆕
│   │   │   ├── log-manager/                       # 🆕
│   │   │   ├── package-deep-clean/                # 🆕
│   │   │   ├── app-residue-cleaner/               # 🆕
│   │   │   └── ... (25+ views total)
│   │   └── components/
│   │       ├── system-monitor/
│   │       ├── temperature-widget/
│   │       └── widget-container/
├── src-tauri/
│   └── src/
│       ├── services/
│       │   ├── dev_cache.service.rs               # 🆕
│       │   ├── container.service.rs                # 🆕
│       │   ├── media_cache.service.rs              # 🆕
│       │   ├── kernel_cleaner.service.rs           # 🆕
│       │   ├── snapshot.service.rs                 # 🆕
│       │   ├── log_manager.service.rs              # 🆕
│       │   ├── package_deep_clean.service.rs        # 🆕
│       │   ├── app_residue.service.rs              # 🆕
│       │   └── ... (30+ services total)
│       └── routes/
│           ├── dev_cache.route.rs                  # 🆕
│           ├── container.route.rs                   # 🆕
│           ├── media_cache.route.rs                 # 🆕
│           ├── kernel_cleaner.route.rs              # 🆕
│           ├── snapshot.route.rs                    # 🆕
│           ├── log_manager.route.rs                 # 🆕
│           ├── package_deep_clean.route.rs          # 🆕
│           └── app_residue.route.rs                 # 🆕
└── docs/
    └── roadmap.md
```

---

## Summary

### ✅ All Features Complete

| Phase | Features | Status |
|-------|----------|--------|
| Original | 15 | ✅ Complete |
| 2025 Phase 1 | 10 | ✅ Complete |
| 2025 Phase 2 | 8 | ✅ Complete |
| **TOTAL** | **33** | **✅ 100%** |

### Feature Categories

| Category | Count |
|----------|-------|
| Core Cleaning | 6 |
| System Optimization | 5 |
| Health & Monitoring | 4 |
| User Experience | 6 |
| Development Tools | 1 |
| Containers | 1 |
| Media Apps | 1 |
| Kernel Management | 1 |
| Snapshots | 1 |
| Log Management | 1 |
| Package Managers | 1 |
| App Residue | 1 |

### Safety Features Implemented

| Feature | Protection |
|---------|------------|
| Kernel Cleaner | Never delete current, keep 2 minimum |
| Snapshot Manager | Never delete < 3 days, keep 3 minimum |
| Container Cleanup | Preview with --dry-run |
| App Residue | Preview before delete |

---

**Last Updated**: May 2026
**Status**: 33 Features - Feature Complete ✅