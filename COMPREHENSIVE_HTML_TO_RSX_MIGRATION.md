# Cleanux — Angular to Dioxus Migration: Page Comparison Report

**Reference branches:** Angular `master`, Dioxus `dioxus-era`
**Dioxus pages directory:** `src/presentation/pages/`
**Angular pages directory:** `src/app/pages/` (on `master`)

---

## Summary

| Status | Count | Pages |
|--------|-------|-------|
| ✅ Fully migrated | 0 | — |
| ⚠️ Partially migrated | 3 | `dashboard`, `clean`, `files`, `power`, `settings` |
| ❌ Missing (no Dioxus page) | 21 | `advanced-cleaner`, `app-residue-cleaner`, `automation`, `backup`, `clipboard`, `container-cleaner`, `dev-cleaner`, `duplicate-finder`, `downloads`, `kernel-cleaner`, `large-files` (→`files`), `log-manager`, `media-cleaner`, `memory-optimizer`, `package-deep-clean`, `profiles`, `reports`, `recent`, `startup`, `system`, `system-repair` |
| 🚫 Does not exist | 4 | `clipboard`, `disk-usage`, `duplicate-finder`, `downloads` |

**Dioxus pages that exist:** `clean_page.rs`, `dashboard_page.rs`, `files_page.rs`, `power_page.rs`, `settings_page.rs` (5 of 26)

---

## Page-by-Page Comparison

---

### 1. Dashboard

| Aspect | Angular | Dioxus |
|--------|---------|--------|
| **Source** | `master:src/app/pages/dashboard/` | `src/presentation/pages/dashboard_page.rs` |
| **Features** | 4 StatCards, Quick Clean, Scheduled Jobs, Recent Scans, live polling | Same structure, static/global_state data, no reactive signals |
| **Missing** | — | Live polling reactivity (no `use_signal`), `bar_pct`/value is hardcoded, schedule editing UI, scan detail navigation, category selection on dashboard |

**Evidence:**
- Angular: `master:src/app/pages/dashboard/dashboard.view.html`
- Dioxus: `src/presentation/pages/dashboard_page.rs:1–N`

**CSS Migration:** ✅ Tailwind CSS v4 (dark:, gradients, material-symbols-rounded) — already migrated. Needs Google Fonts import for `material-symbols-rounded`.

---

### 2. Clean / Cleaner

| Aspect | Angular | Dioxus |
|--------|---------|--------|
| **Source** | `master:src/app/pages/clean/` + `templates/cleanux.html[data-page="clean"]` | `src/presentation/pages/clean_page.rs` |
| **Features** | Category cards, cleaning stages, progress bars, confirmation dialog, notifications | Same CSS structure, `use_signal`/`use_effect`, `global_state` bridge calls |
| **Missing** | — | Loading spinner during scan/clean, confirmation dialog before cleaning, notification system, individual category clean buttons, Clean All confirmation, real stats (uses mock data), total cleanable size header, "More Cleaning Options" router links, error handling UI |

**Evidence:**
- Angular: `master:src/app/pages/clean/clean.view.ts`, `clean.view.html`
- Dioxus: `src/presentation/pages/clean_page.rs:1–157`
- Template: `/mnt/external/Projects/templates/cleanux.html` (data-page="clean")
- Schema: `/mnt/external/Projects/schemas/cleanux/pages/clean.yaml`

**CSS Migration:** ✅ CSS classes match template exactly (Tailwind). No custom CSS migration needed.

---

### 3. Files / Large Files

| Aspect | Angular | Dioxus |
|--------|---------|--------|
| **Source** | `master:src/app/pages/large-files/` (screenshot: `imgREADME/large-files.png`) | `src/presentation/pages/files_page.rs` |
| **Features** | Stats cards, search toolbar, table with sort, pagination, multi-select checkboxes | Card-based list with Remove button |
| **Missing** | — | Stats cards (LARGE FILES FOUND / TOTAL SPACE), refresh button, delete selected (bulk), table column sort arrows, pagination footer, checkbox multi-select, MODIFIED column, per-page selector |

**Evidence:**
- Angular screenshot: `Cleanux/imgREADME/large-files.png`
- Dioxus: `src/presentation/pages/files_page.rs`
- Angular struct (inferred): `name, path, size, modified (date), type`
- Dioxus struct: `name, size, path, age, file_type` (`global_state.rs:88`)

**CSS Migration:** Needs stats card row, toolbar search/refresh/delete, table headers with sort arrows, empty state, pagination footer.

---

### 4. Power

| Aspect | Angular | Dioxus |
|--------|---------|--------|
| **Source** | `master:src/app/pages/power/` | `src/presentation/pages/power_page.rs` |
| **Features** | Power profiles, thermal zones, power actions, scheduled cleanup | Profile selector (lines 60–125), ThermalRow (127–156), PowerActionCard (158–168), ScheduleRow (170–195) |
| **Missing** | — | Battery card (status, health%, cycles), system status card (CPU/Memory/Disk bars), power tools navigation grid (Battery Manager, Thermal Monitor, Power Profiles), `PowerProfile.active` flag per profile |

**Evidence:**
- Angular: `master:src/app/pages/power/power.view.ts`, `power.view.html`
- Dioxus: `src/presentation/pages/power_page.rs:60–327`
- Angular model: `src/app/entities/power.model.ts`
- Dioxus model: `PowerProfileInfo {current, available: Vec<String>}` — missing `active` bool per profile

**CSS Migration:** Existing Tailwind classes reused. No new CSS migration needed.

---

### 5. Settings

| Aspect | Angular | Dioxus |
|--------|---------|--------|
| **Source** | `templates/cleanux.html` (golden template) | `src/presentation/pages/settings_page.rs` |
| **Features** | Appearance, Scan Settings, Automation, Notifications | General Card, Thresholds Card, Excluded Folders |
| **Missing** | — | Appearance card (theme: light/dark/system), Include Trash/Cache/Logs/Thumbnails toggles, Schedule selector, Sound effects toggle |
| **Extra in Dioxus** | — | Secure delete, Minimize to tray, Launch on system start, Min file age threshold, Large file threshold, Excluded folders management |

**Evidence:**
- Template: `tests/golden/settings.html`
- Dioxus: `src/presentation/pages/settings_page.rs`
- Missing bindings: `theme`, `scan_trash`, `scan_cache`, `scan_logs`, `scan_thumbnails`, `schedule`, `sound`

**CSS Migration:** Angular uses `bg-page`, `card`, `btn` (no prefix). Dioxus uses Tailwind classes. CSS needs alignment.

---

## Pages Missing in Dioxus

---

### 6. Advanced Cleaner

| Aspect | Detail |
|--------|--------|
| **Angular** | `src/app/pages/advanced-cleaner/` — 5 junk categories (browser, thumbnails, applications, system, logs), expansion panels for item lists, per-category scan/clean, localStorage for last cleaned |
| **Dioxus** | ❌ No `advanced-cleaner` page. Backend handlers exist in `cleaner_handlers.rs` and `storage_handlers.rs` (stubs). |
| **Missing** | Page component, `JunkCleanerService`, `JunkItem`/`JunkCategorySummary` models, category icons, last cleaned tracking, per-category scan, expansion panels, localStorage persistence |
| **Evidence** | Angular: `master:src/app/pages/advanced-cleaner/advanced-cleaner.view.ts`, `advanced-cleaner.view.html` |

---

### 7. App Residue Cleaner

| Aspect | Detail |
|--------|--------|
| **Angular** | `src/app/pages/app-residue-cleaner/` — 4 tabs (configs, data, caches, orphaned), preview modal, backup warning, pagination, search, select-all with indeterminate |
| **Dioxus** | ❌ No page. Backend handlers `scan_app_residue`/`clean_app_residue` exist in `automation_handlers.rs` and `query_handlers.rs`. |
| **Missing** | `AppResidueService`, `ResidueTabComponent`, `OrphanedTabComponent`, 4-tab interface, backup warning banner, preview modal, pagination, search, select-all indeterminate state |
| **Evidence** | Angular: `src/app/pages/app-residue-cleaner/app-residue-cleaner.view.ts`, `app-residue-cleaner.view.html` |

---

### 8. Automation

| Aspect | Detail |
|--------|--------|
| **Angular** | `src/app/pages/automation/` — Quick actions grid, cron schedule config, directory exclude list, recipe builder via MatDialog, `AutomationService`, `ActionStep` union (CleanCategory/RunProfile/ExecuteCommand/Wait) |
| **Dioxus** | ❌ No page |
| **Missing** | `AutomationPage`, cron schedule UI, directory exclude UI, quick actions grid, recipe builder, `QuickAction`/`AutomationRecipe`/`ExecutionHistoryEntry` models |
| **Evidence** | Angular: `src/app/pages/automation/automation.view.ts`, `automation.view.html` |

---

### 9. Backup

| Aspect | Detail |
|--------|--------|
| **Angular** | ❌ No `backup/` directory found in `master:src/app/pages/` |
| **Dioxus** | ❌ No page |
| **Conclusion** | Page never existed or was removed before migration |

---

### 10. Clipboard

| Aspect | Detail |
|--------|--------|
| **Angular** | ❌ No `clipboard/` directory in any Angular project |
| **Dioxus** | ❌ No page, but `ClipboardService` exists in `dioxus-shared/src/services/clipboard/mod.rs` |
| **Conclusion** | Page never existed. Service properly implemented in shared layer. |

---

### 11. Container Cleaner

| Aspect | Detail |
|--------|--------|
| **Angular** | `src/app/pages/container-cleaner/` — Docker/Podman tabs, container summary, preview/dry-run, `ContainerService` |
| **Dioxus** | ❌ No page. Handler `get_container_summary` exists at `src/handlers/query_handlers.rs:662`. |
| **Missing** | Page, `ContainerSummary` model, tab UI (Docker/Podman), preview/dry-run, confirmation dialog |
| **Evidence** | Angular: `master:src/app/pages/container-cleaner/container-cleaner.view.ts`, `container-cleaner.view.html` |

---

### 12. Dev Cleaner

| Aspect | Detail |
|--------|--------|
| **Angular** | `src/app/pages/dev-cleaner/` — 6 dev tools (npm, pip, cargo, go, maven, gradle), grid of cards, clean per tool or batch, `DevCacheService` |
| **Dioxus** | ❌ No page. Backend `dev_cache_scanner.rs` fully implemented with all clean functions. |
| **Missing** | Page, `DevCacheSummary` Rust struct, format utility, confirm dialog, notification service, `LoadingErrorMixin` equivalent |
| **Evidence** | Angular: `master:src/app/pages/dev-cleaner/dev-cleaner.view.ts`, `dev-cleaner.view.html` |

---

### 13. Duplicate Finder

| Aspect | Detail |
|--------|--------|
| **Angular** | ❌ No `duplicate-finder/` found in any project |
| **Dioxus** | ❌ No page |
| **Conclusion** | Page never existed. |

---

### 14. Downloads Cleaner

| Aspect | Detail |
|--------|--------|
| **Angular** | ❌ No `downloads/` found in Cleanux Angular pages |
| **Dioxus** | ❌ No page |
| **Note** | Referenced in `TAURI_TO_DIOXUS_MIGRATION_REPORT.md:258` as migrated but no dedicated Dioxus page exists |

---

### 15. Kernel Cleaner

| Aspect | Detail |
|--------|--------|
| **Angular** | `src/app/pages/kernel-cleaner/` — Boot space info, current kernel display, kernel data table with pagination, old kernel removal, initramfs cleanup, GRUB update notice, result messages |
| **Dioxus** | ❌ No page. Bridge commands exist (`system_get_current_kernel`, `system_get_installed_kernels`, `system_get_old_kernels`, `system_remove_kernel`, `system_refresh_grub`). Handlers in `query_handlers.rs` (real) and `system_handlers.rs` (stub). |
| **Missing** | Page component, all UI sub-components (BootSpaceInfoCard, KernelDataTable, etc.), pagination state, GRUB notice |
| **Evidence** | Angular: `master:src/app/pages/kernel-cleaner/kernel-cleaner.view.ts`, `kernel-cleaner.view.html` |

---

### 16. Log Manager

| Aspect | Detail |
|--------|--------|
| **Angular** | ❌ No `log-manager/` found in Cleanux |
| **Dioxus** | ❌ No page (only game-related pages) |
| **Note** | Quest-log component exists in GhostGuardian but unrelated |

---

### 17. Media Cleaner

| Aspect | Detail |
|--------|--------|
| **Angular** | `src/app/pages/media-cleaner/` — mat-tab-group with Steam/Spotify/VLC/Thumbnails/Icons tabs, `MediaCacheService`, loading/error mixin |
| **Dioxus** | ❌ No page. Backend `media_cache_scanner.rs` fully implemented. |
| **Missing** | Page, tabbed interface, `MediaCacheSummary` Tauri command exposure |
| **Evidence** | Angular: `master:src/app/pages/media-cleaner/media-cleaner.view.ts`, `media-cleaner.view.html` |

---

### 18. Memory Optimizer

| Aspect | Detail |
|--------|--------|
| **Angular** | `src/app/pages/memory-optimizer/` — Circular SVG gauges for memory/swap, process list with pagination, optimize button, `MemoryOptimizerService` |
| **Dioxus** | ❌ No page. Dashboard shows basic bar-only memory stats. |
| **Missing** | Page, circular SVG gauges, process list, optimize functionality, swap details panel, auto-refresh, pagination |
| **Evidence** | Angular: `master:src/app/pages/memory-optimizer/memory-optimizer.view.ts`, `memory-optimizer.view.html` |

---

### 19. Package Deep Clean

| Aspect | Detail |
|--------|--------|
| **Angular** | `src/app/pages/package-deep-clean/` — Tabs per package manager (apt, pacman, dnf, zypper), orphaned packages, partial downloads, `PackageDeepCleanService` |
| **Dioxus** | ❌ No page. Tauri commands exist (`get_package_summary`, `deep_clean_all`, `apt_clean`, etc.). |
| **Missing** | Page, `PackageManagerSummary`/`OrphanedPackage` models, tabbed interface, double-confirm pattern |
| **Evidence** | Angular: `master:src/app/pages/package-deep-clean/package-deep-clean.view.ts`, `package-deep-clean.view.html` |

---

### 20. Profiles

| Aspect | Detail |
|--------|--------|
| **Angular** | `src/app/pages/profiles/` — List/create/edit/delete profiles, import/export, `ProfileService`, two-way ngModel binding, badge system for profile features |
| **Dioxus** | ❌ No page |
| **Missing** | Page, `CleaningProfile` model, CRUD UI, import/export, `ProfileService` equivalent |
| **Evidence** | Angular: `master:src/app/pages/profiles/profiles.view.ts`, `profiles.view.html` |

---

### 21. Reports

| Aspect | Detail |
|--------|--------|
| **Angular** | ❌ Angular source removed during Tauri→Dioxus migration |
| **Dioxus** | ❌ No dedicated page. Reports surfaced in Dashboard via `crud.find_cleaning_reports` data binding in `dashboard.yaml` |
| **Handler** | `src/application/handlers/crud_handlers.rs:140` — `find_cleaning_reports()` returns `Vec<Value>` with id/cleaned_bytes/timestamp |
| **Evidence** | Schema: `schemas/cleanux/pages/dashboard.yaml:197,205,212` |

---

### 22. Recent

| Aspect | Detail |
|--------|--------|
| **Angular** | ❌ No `recent/` found in Cleanux |
| **Dioxus** | ❌ No page |
| **Note** | Referenced as router link destination from Clean page but never existed as standalone |

---

### 23. Startup

| Aspect | Detail |
|--------|--------|
| **Angular** | ❌ Source removed (Tauri backend deleted) |
| **Dioxus** | ❌ No page. Backend handlers exist: `get_startup_items`, `set_startup_item_enabled`, `disable_startup_item` in `cleaner_handlers.rs:120+` and `command_handlers.rs:40+`. |
| **Missing** | Page with startup item list, enable/disable toggles |
| **Evidence** | Bridge wiring: `src/bridge.rs` pattern `clean_get_startup_items` / `clean_set_startup_enabled` |

---

### 24. System

| Aspect | Detail |
|--------|--------|
| **Angular** | ❌ No `system/` page in Cleanux (only in GhostGuardian as game page) |
| **Dioxus** | ❌ No page |
| **Note** | `ZenithDB/src/infrastructure/system/` contains Rust infra modules (dialogs, logging, memory, opener) — not a UI page |

---

### 25. System Repair

| Aspect | Detail |
|--------|--------|
| **Angular** | `src/app/pages/system-repair/` — 4 tabs (Symlinks, Packages, Cache, Permissions), broken symlink/orphaned package/permission repair, `RepairService` |
| **Dioxus** | ❌ No page. Domain service trait exists at `src/domain/services/repair_service.rs`, impl at `src/infrastructure/repair_service_impl.rs`. |
| **Missing** | Page, tab UI, repair actions (`findBrokenSymlinks`, `findOrphanedPackages`, `cleanFontCache`, `repairPermissions`) |
| **Evidence** | Angular: `master:src/app/pages/system-repair/system-repair.view.ts`, `system-repair.view.html` |

---

## CSS Migration Requirements

### Design Tokens (from Angular global styles)

| Token | Value | Usage |
|-------|-------|-------|
| `--accent` | `#06b6d4` | Primary cyan |
| `--accent-secondary` | `#8b5cf6` | Purple |
| `--accent-glow` | — | Glow effect |
| `--text-primary` | — | Main text |
| `--text-secondary` | — | Secondary text |
| `--bg-surface` | — | Card backgrounds |
| `--bg-elevated` | — | Elevated surfaces |
| `--border-color` | — | Borders |
| `--success` | — | Green |
| `--warning` | — | Yellow |
| `--error` | — | Red |
| `--info` | — | Blue |
| Border radius | 8/12/16/24/32px | sm/md/lg/xl/2xl |
| Fonts | Plus Jakarta Sans, JetBrains Mono | body + code |

### Component CSS Classes

| Class | Purpose |
|-------|---------|
| `view-layout max-w-4xl mx-auto` | Page centering wrapper |
| `glass-card matte-surface` | Frosted glass card |
| `stat-card matte-surface` | Statistics card |
| `glass-card-header / glass-card-body` | Card sections |
| `btn btn-primary / btn-secondary / btn-danger / btn-ghost` | Button variants |
| `btn-icon` | Icon-only button |
| `badge badge-primary / badge-success / badge-warning` | Status badges |
| `text-error / text-warning / text-success / text-info` | Status text colors |
| `bg-error/10 / bg-warning/10 / bg-success/10` | Status backgrounds with opacity |
| `border-trace` | Subtle border |
| `mat-expansion-panel` | Collapsible panel (no Dioxus equivalent) |
| `icon-sm / icon-xl / icon-2xl` | Icon sizing |
| `font-mono` | Monospace for paths/versions |
| `accent-[var(--accent-buttons)]` | Button accent color |

### Tailwind Classes Already Working

- Layout: `grid`, `flex`, `space-y-*`, `gap-*`, `p-*`, `m-*`
- Typography: `text-sm`, `text-xs`, `text-lg`, `font-semibold`, `font-bold`
- Colors: `text-zinc-*`, `bg-zinc-*`, `dark:` variants
- Radius: `rounded-xl`, `rounded-2xl`, `rounded-lg`
- Transitions: `transition-colors`, `transition-all`
- Dark mode: `dark:` prefix variants

### Icons

- **Angular:** Material Icons Outlined via Google Fonts + `<mat-icon fontIcon="...">`
- **Dioxus:** `material-symbols-rounded` — needs `@import url('https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200&display=swap')` in `app.css`

---

## Backend Coverage

Many pages have backend handlers already implemented but not wired to a Dioxus page:

| Handler | Location | Status |
|---------|----------|--------|
| `dev_cache_*` | `src/infrastructure/scanners/dev_cache_scanner.rs` | Full implementation, not wired |
| `media_cache_*` | `src/infrastructure/scanners/media_cache_scanner.rs` | Full implementation, not wired |
| `get_container_summary` | `src/handlers/query_handlers.rs:662` | Implemented, not wired |
| `system_get_*_kernel` | `src/handlers/query_handlers.rs` | Implemented, `system_handlers.rs` has stubs |
| `get_startup_items` | `src/application/handlers/cleaner_handlers.rs:120+` | Implemented, not wired |
| `findCleaningReports` | `src/application/handlers/crud_handlers.rs:140` | Implemented, wired via SDUI |
| `repair_service` | `src/infrastructure/repair_service_impl.rs` | Trait + impl exist |

---

## Priority Recommendations

1. **High Priority:** `automation`, `profiles`, `startup` — user-facing features with backend ready
2. **Medium Priority:** `advanced-cleaner`, `media-cleaner`, `memory-optimizer`, `kernel-cleaner`, `container-cleaner`, `dev-cleaner`, `package-deep-clean`, `system-repair`, `app-residue-cleaner` — cleaning features with partial backend
3. **Low Priority:** `reports` — already surfaced in dashboard, needs dedicated page for full history
