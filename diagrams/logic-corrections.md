# Logic Corrections — Cleanux Architecture

## Project Type

**Native Tauri Project** — Flat command layer with services, NOT migrated to DDD/Dioxus.

---

## AS-IS State Architecture

### AppState Structure (lib.rs)

```rust
pub struct AppState {
  pub data: DataState,
}

pub struct DataState {
  pub repository_service: Arc<repositories::service::RepositoryService>,
  pub crud_service: Arc<services::crud_service::CrudService>,
}
```

**Correction from AGENTS.md**: The AGENTS.md documents `AppState` with `data: Arc<Data>` but the actual code uses `data: DataState` (not wrapped in Arc). The `repository_service` and `crud_service` are wrapped in `Arc<>` at initialization.

---

## Entity Layer

### Entities (src-tauri/src/entities/)

| Entity | Table | Index | Notes |
|--------|-------|-------|-------|
| `CleaningProfileEntity` | `cleaning_profiles` | `name` | Has `From<CleaningProfile>` conversion |
| `AutomationRecipeEntity` | `automation_recipes` | `name` | Has enums `ActionStep`, `RecipeTrigger` |
| `CleaningReportEntity` | `cleaning_reports` | `date` | Has nested `ReportCategories`, `SnapshotComparison` |
| `ExecutionHistoryEntity` | `execution_history` | `started_at` | Tracks recipe/script execution |
| `HealthSnapshotEntity` | `health_snapshots` | `timestamp` | Has `HealthTrendEntity` as separate type |

---

## Command Layer

### Generated vs Direct Commands

Most entity commands are generated via macros:

| Macro | Generates | Used By |
|-------|-----------|---------|
| `crud_get_command!` | `get_<entity>` | profile, automation |
| `crud_get_all_command!` | `get_<entities>` | profile, automation |
| `crud_create_command!` | `create_<entity>` | profile, automation |
| `crud_update_command!` | `update_<entity>` | profile, automation |
| `crud_delete_command!` | `delete_<entity>` | profile, automation |

### Direct Commands (non-macro)

| Module | Commands | Purpose |
|--------|----------|---------|
| `system_command` | 30+ commands | Memory, kernel, services, power management |
| `storage_command` | 35+ commands | Backup, directory, junk, media cache |
| `cleaner_command` | 35+ commands | Repair, startup, containers, app residue |
| `dashboard_command` | 5 commands | System summaries |
| `health_command` | 3 commands | Health history and trends |
| `report_command` | 3 commands | Report generation and comparison |
| `monitor_command` | 7 commands | Temperature and system stats |
| `package_command` | 2 commands | Package cache management |
| `log_command` | 13 commands | Journal and log rotation management |

---

## Service Layer (Business Logic)

### Service Groups

| Category | Services |
|----------|----------|
| **Cleanup** | `cache_cleaning`, `junk_cleaner`, `log_cleaning`, `trash_cleaning`, `large_file_cleaning` |
| **System** | `system`, `process`, `memory`, `power`, `temperature`, `kernel_cleaner` |
| **Containers** | `container_service` (Docker/Podman) |
| **Automation** | `automation_service`, `startup_service`, `repair_service` |
| **Data** | `directory_service`, `scanner_service`, `backup_service` |
| **Persistence** | `crud_service`, `report_service`, `health_history_service` |
| **Media/Dev** | `media_cache_service`, `dev_cache_service` |
| **App State** | `monitor_service`, `dashboard_service`, `profile_service` |
| **Logs** | `log_manager_service` |
| **Residue** | `app_residue_service` |
| **Package** | `package_service` (dnf, pacman, zypper, apt) |

---

## TO-BE Migration Notes

The AGENTS.md documents the migration target pattern:

1. **Frontend**: Angular → Dioxus with DDD layers
2. **Backend**: Tauri becomes thin IPC bridge only
3. **Shared**: `dioxus-shared` crate for themes, schemas, algorithms
4. **State**: AppState remains but refactored for KAS pattern

### Key Migration Transformations

| AS-IS | TO-BE |
|-------|-------|
| `#[tauri::command]` | `#[kas_handler]` with type annotation |
| `commands/*.rs` | `application/kas/` handlers |
| `services/*.rs` | `domain/services/` + `application/services/` |
| `entities/*.rs` | `domain/entities/` |
| Flat `models/` | `domain/value_objects/` |
| `AppState` with `repository_service` | Infrastructure persistence adapters |
| `CrudService` | Repository pattern with nosql_orm |

---

## Notes

- The project uses `nosql_orm` with JSON file storage via `JsonProvider`
- `tauri-plugin-mcp-bridge` is optional (feature-gated as `mcp-bridge`)
- Most commands return `Result<Response<T>, Response<T>>` for structured errors
- CRUD macros generate 5 commands per entity with consistent signatures
