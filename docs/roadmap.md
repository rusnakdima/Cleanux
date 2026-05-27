# Cleanux Project Roadmap

## Project Overview

**Cleanux** is a Linux system cleaning utility built with Tauri v2 (Angular 21 frontend + Rust backend).

**Current Version:** 0.2.1

---

## Version History

### v0.2.1 (Current)

- **Date:** May 2026
- **Status:** Feature Complete
- **Features:** 33/33 implemented

### v0.2.0

- **Date:** April 2026
- **Features:** Phase 2 completion (2025 Phase 2 features)

### v0.1.0

- **Date:** March 2026
- **Features:** Phase 1 completion (Original + 2025 Phase 1)

---

## Complete Feature List

### Phase 1: Original Features (Completed v0.1.0)

| #   | Feature                    | Status | Files                                                   |
| --- | -------------------------- | ------ | ------------------------------------------------------- |
| 1   | Dashboard                  | ✅     | dashboard.service.rs, dashboard.view.ts                 |
| 2   | Cleaner (Cache/Trash/Logs) | ✅     | cleaner.service.rs, cleaner.view.ts                     |
| 3   | Large Files Finder         | ✅     | scanner.service.rs, large-files.view.ts                 |
| 4   | System Services Manager    | ✅     | system.service.rs, system.view.ts                       |
| 5   | Duplicate File Finder      | ✅     | scanner.service.rs, duplicate-finder.view.ts            |
| 6   | Process Manager            | ✅     | process.service.rs, processes.view.ts                   |
| 7   | Startup Manager            | ✅     | startup.service.rs, startup.view.ts                     |
| 8   | Disk Usage Analyzer        | ✅     | directory.service.rs, disk-usage.view.ts                |
| 9   | Real-time System Monitor   | ✅     | monitor.service.rs, system-monitor.component.ts         |
| 10  | Temperature Monitoring     | ✅     | temperature.service.rs, temperature-widget.component.ts |
| 11  | Scheduled Cleaning         | ✅     | scheduler.service.rs, settings.view.ts                  |
| 12  | Cleaning Profile System    | ✅     | profile.service.rs, profiles.view.ts                    |
| 13  | Backup Before Clean        | ✅     | backup.service.rs, backup.view.ts                       |
| 14  | Health History (SQLite)    | ✅     | health_history.service.rs, dashboard.view.ts            |
| 15  | Settings & Preferences     | ✅     | settings.view.ts                                        |

### Phase 2: 2025 New Features (Completed v0.1.0)

| #   | Feature               | Status | Files                                             |
| --- | --------------------- | ------ | ------------------------------------------------- |
| 16  | Memory Optimizer      | ✅     | memory.service.rs, memory-optimizer.view.ts       |
| 17  | Advanced Junk Cleaner | ✅     | junk_cleaner.service.rs, advanced-cleaner.view.ts |
| 18  | App Manager           | ✅     | app_manager.service.rs, app-manager.view.ts       |
| 19  | Battery & Power       | ✅     | power.service.rs, power.view.ts                   |
| 20  | System Repair         | ✅     | repair.service.rs, system-repair.view.ts          |
| 21  | Reports & Analytics   | ✅     | report.service.rs, reports.view.ts                |
| 22  | Automation Features   | ✅     | automation.service.rs, automation.view.ts         |
| 23  | i18n (7 languages)    | ✅     | i18n/\*.json, i18n.service.ts                     |
| 24  | Theme Customization   | ✅     | theme.service.ts, settings.view.ts                |
| 25  | Dashboard Widgets     | ✅     | widget-container.component.ts                     |

### Phase 3: 2025 Cleaning Features (Completed v0.2.0)

| #   | Feature                    | Status | Priority | Description                                  |
| --- | -------------------------- | ------ | -------- | -------------------------------------------- |
| 26  | Dev Cache Cleaner          | ✅     | HIGH     | npm, pip, cargo, go, maven, gradle           |
| 27  | Container Cleanup          | ✅     | HIGH     | Docker/Podman images, containers, volumes    |
| 28  | Media App Cleaner          | ✅     | HIGH     | Steam shader cache, Spotify, VLC, thumbnails |
| 29  | Kernel & Boot Cleanup      | ✅     | CRITICAL | Old kernel removal with safety checks        |
| 30  | Snapshot Manager           | ✅     | HIGH     | Timeshift, Snapper, Btrfs snapshots          |
| 31  | Log Manager & Rotation     | ✅     | MEDIUM   | Journal vacuum, rotated logs, logrotate      |
| 32  | Package Manager Deep Clean | ✅     | HIGH     | apt, dnf, pacman, zypper deep cleaning       |
| 33  | App Residue Cleaner        | ✅     | MEDIUM   | Config/data residue after uninstall          |

---

## Future Roadmap (v0.3.0+)

### Planned Features

#### High Priority

| #   | Feature         | Description                             | Target Version |
| --- | --------------- | --------------------------------------- | -------------- |
| 34  | Network Cleaner | Browser history, cookies, network cache | v0.3.0         |
| 35  | System Backup   | Full system backup via Timeshift/Borg   | v0.3.0         |
| 36  | Secure Delete   | Secure file deletion (shred)            | v0.3.0         |

#### Medium Priority

| #   | Feature                 | Description                              | Target Version |
| --- | ----------------------- | ---------------------------------------- | -------------- |
| 37  | Startup Profiler        | Analyze boot time, suggest optimizations | v0.3.1         |
| 38  | Resource Monitor Widget | Advanced CPU/Memory/Network widgets      | v0.3.1         |
| 39  | Custom Cleaning Recipes | User-defined cleaning operations         | v0.3.1         |

#### Low Priority

| #   | Feature            | Description                     | Target Version |
| --- | ------------------ | ------------------------------- | -------------- |
| 40  | System Information | Detailed hardware/software info | v0.4.0         |
| 41  | AUR Cleaner        | Arch User Repository cleanup    | v0.4.0         |
| 42  | Flatpak Cleaner    | Flatpak orphaned data           | v0.4.0         |

---

## Version Timeline

```
v0.1.0 (Mar 2026) ────────────────────────────────── 15 + 10 features
v0.2.0 (Apr 2026) ───────────────────────────────── 8 new cleaning features
v0.2.1 (May 2026) ───────────────────────────────── 33 features complete
──────────────
v0.3.0 (Q3 2026) ───────────────────────────────── Network, Backup, Secure Delete
v0.3.1 (Q4 2026) ───────────────────────────────── Startup, Resources, Recipes
v0.4.0 (Q1 2027) ───────────────────────────────── Sys Info, AUR, Flatpak
```

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
│   │   │   ├── file.service.ts
│   │   │   ├── dev-cache.service.ts
│   │   │   ├── container.service.ts
│   │   │   ├── media-cache.service.ts
│   │   │   ├── kernel-cleaner.service.ts
│   │   │   ├── snapshot.service.ts
│   │   │   ├── log-manager.service.ts
│   │   │   ├── package-deep-clean.service.ts
│   │   │   ├── app-residue.service.ts
│   │   │   └── ... (28 services total)
│   │   ├── views/
│   │   │   ├── dashboard/
│   │   │   ├── cleaner/
│   │   │   ├── large-files/
│   │   │   ├── duplicate-finder/
│   │   │   ├── disk-usage/
│   │   │   ├── system/
│   │   │   ├── processes/
│   │   │   ├── startup/
│   │   │   ├── power/
│   │   │   ├── memory-optimizer/
│   │   │   ├── kernel-cleaner/
│   │   │   ├── log-manager/
│   │   │   ├── media-cleaner/
│   │   │   ├── container-cleaner/
│   │   │   ├── dev-cleaner/
│   │   │   ├── app-residue-cleaner/
│   │   │   ├── package-deep-clean/
│   │   │   ├── advanced-cleaner/
│   │   │   ├── backup/
│   │   │   ├── automation/
│   │   │   ├── profiles/
│   │   │   ├── reports/
│   │   │   ├── settings/
│   │   │   └── system-repair/
│   │   └── components/
│   │       ├── system-monitor/
│   │       ├── temperature-widget/
│   │       ├── widget-container/
│   │       └── data-table/
├── src-tauri/
│   └── src/
│       ├── services/                # 35 Rust services
│       │   ├── cleaner.service.rs
│       │   ├── scanner.service.rs
│       │   ├── memory.service.rs
│       │   ├── system.service.rs
│       │   ├── process.service.rs
│       │   ├── kernel_cleaner.service.rs
│       │   ├── container.service.rs
│       │   ├── dev_cache.service.rs
│       │   ├── log_manager.service.rs
│       │   ├── package_deep_clean.service.rs
│       │   ├── automation.service.rs
│       │   ├── scheduler.service.rs
│       │   └── ...
│       ├── routes/                  # 30+ route handlers
│       ├── models/
│       ├── helpers/
│       └── errors/
├── docs/
│   └── roadmap.md
├── scripts/
├── flatpak/
└── imgREADME/
```

---

## Tech Stack

| Layer    | Technology                                   |
| -------- | -------------------------------------------- |
| Frontend | Angular 21.1.0 (Standalone, Signals, OnPush) |
| Backend  | Rust, Tauri ~2.10                            |
| Build    | Bun 1.3.5                                    |
| State    | Angular Signals with OnPush                  |
| Styling  | TailwindCSS 4, Glassmorphic UI               |
| i18n     | 7 languages (EN, ES, FR, DE, RU, ZH, JA)     |

---

## Summary

### Implementation Phases

| Phase                 | Features | Status      | Version    |
| --------------------- | -------- | ----------- | ---------- |
| Original Features     | 15       | ✅ Complete | v0.1.0     |
| 2025 Phase 1          | 10       | ✅ Complete | v0.1.0     |
| 2025 Phase 2          | 8        | ✅ Complete | v0.2.0     |
| **Total Implemented** | **33**   | **✅ 100%** | **v0.2.1** |

### Future Plans

| Phase             | Features | Status     | Target |
| ----------------- | -------- | ---------- | ------ |
| 2026 Phase 1      | 3        | 🔄 Planned | v0.3.0 |
| 2026 Phase 2      | 3        | 🔄 Planned | v0.3.1 |
| 2026 Phase 3      | 3        | 🔄 Planned | v0.4.0 |
| **Total Planned** | **42**   |            |        |

### Feature Categories

| Category            | Implemented | Planned |
| ------------------- | ----------- | ------- |
| Core Cleaning       | 6           | 2       |
| System Optimization | 5           | 2       |
| Health & Monitoring | 4           | 1       |
| User Experience     | 6           | 2       |
| Development Tools   | 1           | 1       |
| Containers          | 1           | 0       |
| Media Apps          | 1           | 0       |
| Kernel Management   | 1           | 0       |
| Snapshots           | 1           | 0       |
| Log Management      | 1           | 0       |
| Package Managers    | 1           | 1       |
| App Residue         | 1           | 1       |
| Security            | 0           | 1       |
| Backup              | 0           | 1       |

---

## Contributing

See [CONTRIBUTING.md]() for development guidelines.

---

**Last Updated:** May 2026
**Current Version:** 0.2.1
**Status:** Feature Complete ✅ | Quality Complete ✅
