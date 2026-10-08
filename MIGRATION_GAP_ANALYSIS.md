# Cleanux Migration Gap Analysis
**Comparing:** `master` (Tauri/Angular) → `dioxus-era` (Dioxus Desktop)
**Generated:** 2026-10-07

---

## Executive Summary

| Category | Master (Tauri) | Dioxus-era | Gap |
|----------|----------------|------------|-----|
| **Pages** | 28 Angular pages | 6 Rust pages | **22 pages missing (79%)** |
| **KAS Handlers** | ~140 Tauri commands | ~120 handlers | **~20 commands missing (14%)** |
| **Services** | 30+ service files | 5 implemented, rest stubbed | **25+ services missing (83%)** |
| **Entities** | 5 entity files | 5 entity files | **Covered** |

---

## 1. PAGES: 22 of 28 Missing

### ✅ EXISTING in dioxus-era (6 pages)
| Page | File | Status |
|------|------|--------|
| Dashboard | `src/presentation/pages/dashboard_page.rs` | Implemented |
| Clean | `src/presentation/pages/clean_page.rs` | Implemented |
| Files | `src/presentation/pages/files_page.rs` | Implemented |
| Power | `src/presentation/pages/power_page.rs` | Implemented |
| Settings | `src/presentation/pages/settings_page.rs` | Implemented |
| *(mod.rs)* | `src/presentation/pages/mod.rs` | Module re-exports |

### ❌ MISSING from master (22 pages)

| # | Page | Master Path | Missing Commands/Features |
|---|------|-------------|--------------------------|
| 1 | **advanced-cleaner** | `src/app/pages/advanced-cleaner/` | Multi-category junk scanning, selective cleaning |
| 2 | **app-residue-cleaner** | `src/app/pages/app-residue-cleaner/` | App config/data/cache residue scanning, orphaned package detection |
| 3 | **automation** | `src/app/pages/automation/` | Quick actions, recipe CRUD, recipe execution |
| 4 | **backup** | `src/app/pages/backup/` | Backup creation, restore, listing, deletion |
| 5 | **cleaner** | `src/app/pages/cleaner/` | Unified cleaner view with category selection |
| 6 | **clipboard** | `src/app/pages/clipboard/` | Clipboard history management |
| 7 | **container-cleaner** | `src/app/pages/container-cleaner/` | Docker/Podman image, container, volume pruning |
| 8 | **dev-cleaner** | `src/app/pages/dev-cleaner/` | npm, pip, cargo, go, maven, gradle cache cleaning |
| 9 | **disk-usage** | `src/app/pages/disk-usage/` | Disk space visualization, large file discovery |
| 10 | **downloads** | `src/app/pages/downloads/` | Downloads folder cleanup |
| 11 | **duplicate-finder** | `src/app/pages/duplicate-finder/` | SHA256-based duplicate file detection |
| 12 | **kernel-cleaner** | `src/app/pages/kernel-cleaner/` | Old kernel removal, initramfs cleanup, GRUB update |
| 13 | **large-files** | `src/app/pages/large-files/` | Configurable size threshold file finder |
| 14 | **log-manager** | `src/app/pages/log-manager/` | Journal vacuuming, logrotate config, rotated logs |
| 15 | **media-cleaner** | `src/app/pages/media-cleaner/` | Steam shader/download cache, Spotify, VLC, thumbnails |
| 16 | **memory-optimizer** | `src/app/pages/memory-optimizer/` | Memory drop caches, swap optimization |
| 17 | **package-deep-clean** | `src/app/pages/package-deep-clean/` | apt/dnf/pacman/zypper cache cleaning |
| 18 | **processes** | `src/app/pages/processes/` | Process listing, killing, memory usage |
| 19 | **profiles** | `src/app/pages/profiles/` | Cleaning profile CRUD, profile application |
| 20 | **recent** | `src/app/pages/recent/` | Recently modified files |
| 21 | **reports** | `src/app/pages/reports/` | Cleaning history, report generation, snapshot comparison |
| 22 | **startup** | `src/app/pages/startup/` | Startup item management |
| 23 | **system-repair** | `src/app/pages/system-repair/` | Broken symlinks, orphaned packages, permission repair |

> **Note:** User mentioned 26 pages, but git shows 28 directories. Discrepancy may be due to page count methodology.

---

## 2. KAS HANDLERS: ~20 Commands Missing

### Master Tauri Commands by Module (140+ total)

#### automation.command.rs (9 commands)
| Command | Status in dioxus |
|---------|-----------------|
| `get_automation_recipe` | ❌ Missing (only CRUD via `find_automation_recipe`) |
| `get_automation_recipes` | ❌ Missing |
| `create_automation_recipe` | ❌ Missing |
| `update_automation_recipe` | ❌ Missing |
| `delete_automation_recipe` | ❌ Missing |
| `crud_get_execution_history` | ✅ Implemented (`find_execution_history`) |
| `crud_get_quick_actions` | ✅ Implemented (`list_quick_actions`) |
| `crud_execute_action` | ✅ Implemented (`execute_quick_action`) |
| `crud_execute_recipe` | ✅ Implemented (`execute_recipe`) |

#### cleaner.command.rs (41 commands)
| Command | Status in dioxus |
|---------|-----------------|
| `find_broken_symlinks` | ✅ Implemented |
| `find_orphaned_packages` | ✅ Implemented |
| `clean_font_cache` | ❌ Missing |
| `clean_repair_icon_cache` | ❌ Missing |
| `repair_permissions` | ❌ Missing |
| `remove_broken_symlink` | ❌ Missing |
| `clean_repair_orphaned_pkg` | ❌ Missing |
| `get_startup_items` | ✅ Implemented |
| `disable_startup_item` | ✅ Implemented |
| `enable_startup_item` | ✅ Implemented |
| `get_container_summary` | ⚠️ Partial (missing podman fields) |
| `docker_system_prune` | ✅ Implemented |
| `docker_image_prune` | ❌ Missing |
| `docker_container_prune` | ❌ Missing |
| `docker_volume_prune` | ❌ Missing |
| `docker_preview_prune` | ❌ Missing |
| `podman_system_prune` | ❌ Missing |
| `podman_image_prune` | ❌ Missing |
| `get_quick_actions` | ✅ Implemented |
| `execute_action` | ✅ Implemented |
| `get_recipes` | ✅ Implemented |
| `save_recipe` | ⚠️ Via CRUD (`insert_automation_recipe`) |
| `delete_recipe` | ✅ Implemented |
| `execute_recipe` | ✅ Implemented |
| `get_execution_history` | ✅ Implemented |
| `get_residue_summary` | ❌ Missing |
| `scan_user_configs` | ❌ Missing |
| `scan_user_data` | ❌ Missing |
| `scan_user_caches` | ❌ Missing |
| `scan_home_residues` | ❌ Missing |
| `get_orphaned_configs` | ❌ Missing |
| `clean_app_residue` | ✅ Implemented |
| `clean_multiple_app_residues` | ❌ Missing |

#### dashboard.command.rs (5 commands)
| Command | Status in dioxus |
|---------|-----------------|
| `get_system_services` | ❌ Missing |
| `get_cache_summary` | ❌ Missing |
| `get_trash_summary` | ❌ Missing |
| `get_log_summary` | ❌ Missing |
| `get_large_files_summary` | ❌ Missing |

#### health.command.rs (6 commands)
| Command | Status in dioxus |
|---------|-----------------|
| `crud_get_health_snapshot` | ✅ Implemented |
| `crud_get_health_snapshots` | ✅ Implemented |
| `crud_create_health_snapshot` | ✅ Implemented |
| `crud_get_health_history` | ✅ Implemented |
| `crud_get_health_trends` | ✅ Implemented |
| `crud_save_health_snapshot` | ✅ Implemented |

#### log.command.rs (12 commands)
| Command | Status in dioxus |
|---------|-----------------|
| `get_journal_size` | ✅ Implemented |
| `get_journal_usage` | ✅ Implemented |
| `vacuum_journal` | ✅ Implemented |
| `vacuum_journal_by_days` | ✅ Implemented |
| `get_rotated_logs_size` | ✅ Implemented |
| `get_rotated_logs` | ✅ Implemented |
| `clean_rotated_logs` | ✅ Implemented |
| `get_logrotate_configs` | ✅ Implemented |
| `analyze_logrotate` | ✅ Implemented |
| `get_var_log_usage` | ✅ Implemented |
| `get_largest_log_files` | ✅ Implemented |
| `get_log_manager_summary` | ✅ Implemented |

#### monitor.command.rs (9 commands)
| Command | Status in dioxus |
|---------|-----------------|
| `get_temperatures` | ✅ Implemented |
| `get_cpu_temperature` | ✅ Implemented |
| `get_gpu_temperature` | ✅ Implemented |
| `save_health_snapshot` | ✅ Implemented |
| `get_health_history` | ✅ Implemented |
| `get_health_trends` | ✅ Implemented |
| `get_system_stats` | ✅ Implemented |
| `start_monitoring` | ✅ Implemented |
| `stop_monitoring` | ✅ Implemented |

#### package.command.rs (2 commands)
| Command | Status in dioxus |
|---------|-----------------|
| `get_package_cache_info` | ❌ Missing |
| `clean_package_cache` | ❌ Missing |

#### profile.command.rs (6 commands)
| Command | Status in dioxus |
|---------|-----------------|
| `get_cleaning_profile` | ✅ Implemented |
| `get_cleaning_profiles` | ✅ Implemented |
| `create_cleaning_profile` | ✅ Implemented |
| `update_cleaning_profile` | ✅ Implemented |
| `delete_cleaning_profile` | ✅ Implemented |
| `apply_cleaning_profile` | ❌ Missing |

#### report.command.rs (5 commands)
| Command | Status in dioxus |
|---------|-----------------|
| `get_cleaning_report` | ✅ Implemented |
| `get_cleaning_reports` | ✅ Implemented |
| `create_cleaning_report` | ✅ Implemented |
| `crud_generate_cleaning_report` | ✅ Implemented |
| `crud_compare_snapshots` | ✅ Implemented |

#### storage.command.rs (36 commands)
| Command | Status in dioxus |
|---------|-----------------|
| `create_backup` | ✅ Implemented |
| `restore_backup` | ✅ Implemented |
| `list_backups` | ✅ Implemented |
| `delete_backup` | ✅ Implemented |
| `get_backup_dir` | ❌ Missing |
| `scan_directory` | ✅ Implemented |
| `get_directory_size` | ✅ Implemented |
| `find_empty_directories` | ✅ Implemented |
| `find_nested_empty_directories` | ❌ Missing |
| `remove_empty_directory` | ✅ Implemented |
| `remove_empty_directories` | ✅ Implemented |
| `find_duplicates` | ✅ Implemented |
| `get_junk_summary` | ✅ Implemented |
| `scan_browser_caches` | ✅ Implemented |
| `scan_thumbnail_caches` | ✅ Implemented |
| `scan_application_caches` | ❌ Missing |
| `scan_system_temp` | ❌ Missing |
| `scan_log_rotations` | ❌ Missing |
| `clean_junk_category` | ✅ Implemented |
| `get_media_cache_summary` | ❌ Missing |
| `clean_steam_shader_cache` | ❌ Missing |
| `clean_steam_download_cache` | ❌ Missing |
| `clean_spotify_cache` | ❌ Missing |
| `clean_vlc_cache` | ❌ Missing |
| `clean_thumbnail_cache` | ✅ Implemented |
| `clean_media_icon_cache` | ❌ Missing |
| `get_dev_cache_summary` | ❌ Missing |
| `clean_npm_cache` | ❌ Missing |
| `clean_pip_cache` | ❌ Missing |
| `clean_cargo_cache` | ❌ Missing |
| `clean_go_cache` | ❌ Missing |
| `clean_maven_cache` | ❌ Missing |
| `clean_gradle_cache` | ❌ Missing |
| `clean_all_dev_caches` | ❌ Missing |

#### system.command.rs (26 commands)
| Command | Status in dioxus |
|---------|-----------------|
| `get_memory_info` | ✅ Implemented |
| `get_swap_info` | ✅ Implemented |
| `get_process_memory` | ✅ Implemented |
| `optimize_memory` | ✅ Implemented |
| `get_processes` | ❌ Missing |
| `kill_process` | ❌ Missing |
| `kill_selected_processes` | ❌ Missing |
| `get_current_kernel` | ✅ Implemented |
| `get_installed_kernels` | ✅ Implemented |
| `get_old_kernels` | ✅ Implemented |
| `get_old_kernels_size` | ❌ Missing |
| `remove_kernel` | ✅ Implemented |
| `get_old_initramfs` | ❌ Missing |
| `remove_initramfs` | ❌ Missing |
| `get_boot_space_info` | ❌ Missing |
| `update_grub` | ✅ Implemented |
| `get_battery_info` | ✅ Implemented |
| `get_power_profiles` | ❌ Missing |
| `set_power_profile` | ❌ Missing |
| `get_thermal_info` | ❌ Missing |
| `stop_service` | ✅ Implemented |
| `stop_selected_services` | ❌ Missing |
| `open_file` | ✅ Implemented |
| `get_all_services` | ✅ Implemented |
| `enable_service` | ✅ Implemented |
| `start_service` | ✅ Implemented |
| `enable_selected_services` | ❌ Missing |

---

## 3. SERVICES: 25+ Missing/Stubbed

### Master Services (30+ files)

```
src-tauri/src/services/
├── app_residue/           → ❌ Missing in dioxus (stubbed in commands.rs)
├── app-residue.service.rs → ❌ Missing
├── apt.service.rs         → ❌ Missing
├── automation.service.rs  → ⚠️ Partial (domain layer exists)
├── backup.service.rs      → ❌ Missing
├── cache-cleaning.service.rs → ❌ Missing
├── container.service.rs   → ⚠️ Partial (CLI-based in cleaner_handlers)
├── crud_service.rs        → ⚠️ Via json_storage
├── dashboard.service.rs   → ❌ Missing
├── dev-cache.service.rs   → ❌ Missing
├── directory.service.rs   → ⚠️ Partial (sys_utils)
├── dnf.service.rs         → ❌ Missing
├── file-preview.service.rs → ❌ Missing
├── health-history.service.rs → ❌ Missing
├── junk/                  → ❌ Missing
├── junk-cleaner.service.rs → ❌ Missing
├── kernel-cleaner.service.rs → ⚠️ Partial (via handlers)
├── large-file-cleaning.service.rs → ❌ Missing
├── log-cleaning.service.rs → ❌ Missing
├── log-manager.service.rs  → ⚠️ Partial (journal_handlers)
├── logs/                  → ❌ Missing
├── media-cache.service.rs → ❌ Missing
├── memory.service.rs      → ✅ Implemented (MemoryService)
├── monitor.service.rs     → ⚠️ Partial (monitoring_service)
├── package.service.rs     → ❌ Missing
├── pacman.service.rs      → ❌ Missing
├── power.service.rs        → ❌ Missing
├── process.service.rs     → ❌ Missing (stubbed)
├── profile.service.rs      → ❌ Missing
├── repair.service.rs      → ❌ Missing
├── report.service.rs      → ❌ Missing
├── scanner.service.rs     → ❌ Missing
├── startup.service.rs    → ❌ Missing
├── system.service.rs      → ❌ Missing
├── temperature.service.rs → ❌ Missing
├── trash-cleaning.service.rs → ❌ Missing
└── zypper.service.rs       → ❌ Missing
```

### Dioxus-era Infrastructure Services Status

| Service | File | Status | Evidence |
|---------|------|--------|----------|
| **SystemInfoServiceImpl** | `services.rs` | ⚠️ Stub | Returns `0` for all values |
| **ClipboardServiceImpl** | `services.rs` | ⚠️ Stub | No-op implementation |
| **NotificationServiceImpl** | `services.rs` | ⚠️ Stub | No-op implementation |
| **MemoryService** | `memory_service.rs` | ✅ Implemented | Real `/proc/meminfo` parsing |
| **MonitoringService** | `monitoring_service.rs` | ⚠️ Partial | Basic CPU/memory monitoring |
| **AutomationServiceImpl** | `automation_service_impl.rs` | ❌ Missing | File doesn't exist as functional |
| **CleaningServiceImpl** | `cleaning_service_impl.rs` | ❌ Missing | Empty module |
| **HealthServiceImpl** | `health_service_impl.rs` | ❌ Missing | Empty module |
| **Commands** | `commands.rs` | ⚠️ Stub | Contains `app_residue_scan`, `app_residue_clean` only |
| **History** | `history.rs` | ❌ Missing | Empty module |
| **JsonStorage** | `json_storage.rs` | ✅ Implemented | JSON file CRUD |
| **PackageManagers** | `package_managers.rs` | ❌ Missing | Empty module |
| **SchedulerService** | `scheduler_service.rs` | ❌ Missing | Empty module |
| **Settings** | `settings.rs` | ✅ Implemented | Partial settings |
| **SysUtils** | `sys_utils.rs` | ✅ Implemented | Real file utilities |
| **DevCacheScanner** | `scanners/dev_cache_scanner.rs` | ❌ Missing | File missing |
| **MediaCacheScanner** | `scanners/media_cache_scanner.rs` | ❌ Missing | File missing |

---

## 4. DOMAIN ENTITIES: Covered

Both branches have equivalent entity files:

| Entity | master | dioxus-era |
|--------|--------|------------|
| `automation_recipe` | ✅ | ✅ |
| `cleaning_profile` | ✅ | ✅ |
| `cleaning_report` | ✅ | ✅ |
| `execution_history` | ✅ | ✅ |
| `health_snapshot` | ✅ | ✅ |

---

## 5. CRITICAL GAPS BY FEATURE AREA

### A. Cleanup Features (HIGH PRIORITY)

| Feature | Master Handler | Dioxus Handler | Gap |
|---------|---------------|----------------|-----|
| App Residue Scan | `scan_user_configs/data/caches` | ❌ None | Full scan logic missing |
| Font Cache Clean | `clean_font_cache` | ❌ None | — |
| Icon Cache Repair | `clean_repair_icon_cache` | ❌ None | — |
| Permission Repair | `repair_permissions` | ❌ None | — |
| Dev Cache (npm/pip/cargo/go/maven/gradle) | 7 commands | ❌ None | Scanner missing |
| Media Cache (Steam/Spotify/VLC) | 4 commands | ❌ None | Scanner missing |
| Package Cache (apt/dnf/pacman/zypper) | 2 commands | ❌ None | Package manager modules missing |
| Large Files | `get_large_files_summary` | ❌ None | — |
| Duplicate Finder | `find_duplicates` | ⚠️ Partial | Returns duplicates but incomplete |
| Empty Directory | `find_nested_empty_directories` | ❌ None | — |
| System Temp Scan | `scan_system_temp` | ❌ None | — |
| Log Rotation Scan | `scan_log_rotations` | ❌ None | — |

### B. System Features (MEDIUM PRIORITY)

| Feature | Master Handler | Dioxus Handler | Gap |
|---------|---------------|----------------|-----|
| Process List | `get_processes` | ❌ None | Process enumeration missing |
| Kill Process | `kill_process` | ❌ None | — |
| Power Profiles | `get_power_profiles`, `set_power_profile` | ❌ None | — |
| Thermal Info | `get_thermal_info` | ❌ None | — |
| Old Initramfs | `get_old_initramfs`, `remove_initramfs` | ❌ None | — |
| Boot Space | `get_boot_space_info` | ❌ None | — |
| System Services | `get_all_services` | ⚠️ Partial | Missing systemd details |
| Dashboard Stats | 5 commands | ❌ None | — |

### C. Automation Features (MEDIUM PRIORITY)

| Feature | Master Handler | Dioxus Handler | Gap |
|---------|---------------|----------------|-----|
| Recipe CRUD | Full CRUD | ⚠️ Partial | Via generic CRUD only |
| Quick Actions | `crud_get_quick_actions` | ✅ Implemented | — |
| Execution History | `crud_get_execution_history` | ✅ Implemented | — |

### D. Storage/Backup Features (LOW PRIORITY)

| Feature | Master Handler | Dioxus Handler | Gap |
|---------|---------------|----------------|-----|
| Backup Dir | `get_backup_dir` | ❌ None | — |
| Nested Empty Dirs | `find_nested_empty_directories` | ❌ None | — |
| App Caches Scan | `scan_application_caches` | ❌ None | — |

---

## 6. FILE EVIDENCE INDEX

### Missing Service Implementations

| Evidence | File | Line | Issue |
|----------|------|------|-------|
| `commands.rs:1` | `src/infrastructure/commands.rs` | — | Only 2 stub commands |
| `services.rs:36` | `src/infrastructure/services.rs` | 36 | SystemInfoServiceImpl returns zeros |
| `services.rs:80` | `src/infrastructure/services.rs` | 80 | ClipboardServiceImpl is no-op |
| `services.rs:90` | `src/infrastructure/services.rs` | 90 | NotificationServiceImpl is no-op |
| `mod.rs` | `src/infrastructure/mod.rs` | — | automation_service_impl, cleaning_service_impl, health_service_impl all re-export stubs |

### Missing Page Implementations

| Evidence | File | Issue |
|----------|------|-------|
| `pages/mod.rs` | `src/presentation/pages/mod.rs` | Only exports 6 pages |
| `ls pages/` | `src/presentation/pages/` | Only 6 .rs files exist |

### Partial Handler Implementations

| Evidence | File | Issue |
|----------|------|-------|
| `cleaner_handlers.rs:50` | `src/application/handlers/cleaner_handlers.rs` | Only basic Docker info, missing podman |
| `system_handlers.rs` | `src/application/handlers/system_handlers.rs` | Missing `get_processes`, `kill_process` |
| `journal_handlers.rs` | `src/application/handlers/journal_handlers.rs` | Only journal handling, no log-cleaning.service |

---

## 7. RECOMMENDED MIGRATION PRIORITY

### Phase 1: Critical (Must Have)
1. **Memory/Kernel/Power** — Already mostly wired, needs polish
2. **Process Management** — `get_processes`, `kill_process`
3. **App Residue Scanner** — Core cleaner feature
4. **Package Manager Cleaners** — apt/dnf/pacman/zypper

### Phase 2: High Value
5. **Dev Cache Cleaners** — npm/pip/cargo/go/maven/gradle
6. **Media Cache Cleaners** — Steam/Spotify/VLC
7. **Dashboard Aggregations** — system_services, cache_summary, etc.
8. **Backup/Restore** — Full backup system

### Phase 3: Medium
9. **Duplicate Finder** — SHA256-based (partially exists)
10. **Log Cleaner** — rotated logs, logrotate configs
11. **Container Cleaner** — Docker/Podman full pruning
12. **System Repair** — broken symlinks, permissions

### Phase 4: Nice to Have
13. **Clipboard Manager**
14. **Recent Files**
15. **Startup Manager**
16. **Profiles/Reports** — Already have CRUD, need UI

---

## 8. TEST COVERAGE GAP

| Test File (master) | Status in dioxus |
|-------------------|------------------|
| `app_residue_service_tests.rs` | ❌ Missing |
| `cache_cleaning_service_tests.rs` | ❌ Missing |
| `cache_service_tests.rs` | ❌ Missing |
| `command_tests.rs` | ❌ Missing |
| `dev_cache_service_tests.rs` | ❌ Missing |
| `directory_service_tests.rs` | ❌ Missing |
| `error_tests.rs` | ❌ Missing |
| `filesystem_helper_tests.rs` | ❌ Missing |
| `junk_cleaner_tests.rs` | ❌ Missing |
| `kernel_cleaner_service_tests.rs` | ❌ Missing |
| `log_manager_service_tests.rs` | ❌ Missing |
| `media_cache_service_tests.rs` | ❌ Missing |
| `path_validation_tests.rs` | ❌ Missing |
| `response_helper_tests.rs` | ❌ Missing |
| `response_model_tests.rs` | ❌ Missing |
| `security_tests.rs` | ❌ Missing |
| `trash_cleaning_service_tests.rs` | ❌ Missing |

**dioxus-era tests exist at:**
- `tests/parity_tests.rs`
- `tests/p_features.rs`
- `tests/ssr_dump.rs`
- `tests/template_parity.rs`

---

*Analysis based on git branches: master (Tauri/Angular) vs dioxus-era (Dioxus Desktop)*
