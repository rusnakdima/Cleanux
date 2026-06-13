# Cleanux Refactoring Plan

## Project Overview
Cleanux is a Tauri v2 desktop application (Rust backend + Angular frontend) for system cleanup and optimization on Linux. The codebase has significant code duplication, dead code, and architectural issues that need to be addressed.

---

## 1. DEAD CODE TO REMOVE

### Rust Backend

| File | Lines | Reason | Remove Action |
|------|-------|--------|---------------|
| `src-tauri/src/routes/log_analyzer.route.rs` | 22 | Route defined in `routes/mod.rs` but commands (`get_log_summary`, `get_log_entries`, `clean_old_logs`) are NOT registered in `lib.rs` invoke_handler (lines 107-254) | Delete file, remove from `routes/mod.rs` |
| `src-tauri/src/logging/macros.rs` | 68 | Macros `log_operation`, `log_operation_simple`, `log_if_enabled` defined but never used anywhere | Delete file, remove from `logging/mod.rs` |
| `src-tauri/src/logging/logger.rs` | 98 | `AppLogger` struct duplicates `Logger` from `logger.rs` - not used | Delete file, remove from `logging/mod.rs` |
| `src-tauri/src/logger.rs` | 60 | `Logger` struct (lines 5-54) duplicates `AppLogger` - `Logger` is used in `lib.rs:85` but `AppLogger` is more feature-rich | Consolidate into `logging/logger.rs` as `AppLogger` |
| `src-tauri/src/state.rs` | 21 | `AppState` struct (lines 3-6) with Mutex fields for `automation_service` and `backup_service` is never registered with Tauri | Delete file entirely |
| `src-tauri/src/middleware.rs` | 51 | `LoggingMiddleware`, `ErrorHandler`, `MetricsCollector` defined but never used | Delete file entirely |
| `src-tauri/src/dto.rs` | 109 | `ApiResponse` and DTOs (`CacheFilesRequest`, `ClearFilesRequest`, etc.) not used - codebase uses `ResponseModel` | Delete file entirely |
| `src-tauri/src/routes/scheduler.route.rs` | 41 | Commands not registered in `lib.rs` invoke_handler | Delete file, remove from `routes/mod.rs` |
| `src-tauri/src/services/scheduler.service.rs` | 209 | `SchedulerService` with systemd timer setup not used (commands not registered) | Delete file, remove from `services/mod.rs` |
| `src-tauri/src/routes/log_analyzer.route.rs` | 22 | Duplicate - already listed above | Confirm deletion |
| `src-tauri/src/helpers/singleton_macro.rs` | 13 | `define_singleton_service` macro never used | Delete file, remove from `helpers/mod.rs` |
| `src-tauri/src/logging/mod.rs` | 5 | References deleted `macros.rs` and `logger.rs` | Update to remove dead exports |

### Frontend

| File | Lines | Reason | Remove Action |
|------|-------|--------|---------------|
| `src/app/services/log-analyzer.service.ts` | 92 | Service calls `get_log_summary`, `get_log_entries`, `clean_old_logs` which are NOT registered in Rust backend | Delete file |
| `src/app/models/log-analyzer.model.ts` | ~50 | Model for log analyzer data - only used by dead log-analyzer service | Delete file |
| `src/app/services/logger.service.ts` | 315 | Comprehensive client-side logging - extensive but only used by dead services | Review usage, likely delete |

---

## 2. UNUSED DEPENDENCIES TO REMOVE

### Cargo.toml

| Crate | Version | Reason |
|-------|---------|--------|
| `tar` | 0.4 | Archive handling - not used anywhere in services/routes |
| `flate2` | 1.0 | Compression - not used anywhere (tar uses it but tar is unused) |
| `rusqlite` | 0.32 | SQLite for health_history - `HealthHistoryService` exists but database commands not exposed via Tauri |
| `tauri-plugin-mcp-bridge` | 0.8 | Optional plugin, only used when `mcp-bridge` feature enabled |
| `parking_lot` | 0.12 | Not used - standard `std::sync` used instead |

### package.json

| Package | Version | Reason |
|---------|---------|--------|
| `flyonui` | 2.4.1 | UI library - not found in any component templates |
| `flowbite` | 4.0.1 | UI library - not found in any component templates |
| `jsdom` | 27.1.0 | Testing dependency only - acceptable in devDependencies |

---

## 3. DUPLICATED CODE TO CONSOLIDATE

### Rust

| Pattern | Locations | Solution |
|---------|----------|----------|
| **Package Manager Services** | `services/package.service.rs` (702 lines) and `services/package_deep_clean.service.rs` (458 lines) | These are nearly identical duplicates. Consolidate into single `package.service.rs`. Both routes registered in `lib.rs` cause command shadowing. |
| **Package Manager Routes** | `routes/package.route.rs` (120 lines) and `routes/package_deep_clean.route.rs` (88 lines) | Both define identical commands. Delete `package_deep_clean.route.rs`, keep only `package.route.rs`. |
| **Response Building** | `helpers/response.helper.rs` lines 6-98 | `ResponseBuilder` struct and helper functions (`success_response`, `info_response`, `error_response`, `data_string`, `data_empty_string`) overlap. Keep builder pattern, consolidate helpers. |
| **Command Execution** | `helpers/process.helper.rs` lines 6-99 | `stderr_message` (line 6) and `stderr_string` (line 10) are identical. `run_command` (line 18) and `get_command_output` (line 32) overlap. `pkexec` (line 74) and `pkexec_with_args` (line 90) overlap. Consolidate. |
| **Path Validation** | `helpers/validation.helper.rs` lines 6-30 and `security/allowlist.rs` lines 69-71 | Both expose `validate_path` and `is_allowed_path`. Consolidate into single location. |
| **Installed Apps Detection** | `services/app_residue.service.rs` lines 47-179 and `services/repair.service.rs` lines 79-152 | Both have `get_installed_apps()` using `dpkg -l`. Extract to shared helper. |
| **Logging Structs** | `logger.rs` lines 5-54 and `logging/logger.rs` lines 6-98 | Both have logging structs with similar functionality. Consolidate into `logging/logger.rs` as `AppLogger`. |
| **CommonPath Helper** | `helpers/common_paths.rs` lines 55-61 | `exists()` (line 55) and `to_path_buf()` (line 59) methods on `CommonPath` enum are never called. Remove if truly unused. |

### Frontend

| Pattern | Locations | Solution |
|---------|----------|----------|
| **Logging in Services** | Multiple services use verbose try/catch with logging | Consider creating base service class with centralized error handling and logging |
| **API Error Handling** | `api.service.ts` and individual services | Services like `dev-cache.service.ts`, `media-cache.service.ts` have identical error handling patterns |

---

## 4. ARCHITECTURAL IMPROVEMENTS

### Rust Backend

- **Remove dual logging system**: `logger.rs` and `logging/logger.rs` both provide logging. Consolidate to `logging/logger.rs` as `AppLogger`.
  - File: `src-tauri/src/logger.rs` (entire file)
  - File: `src-tauri/src/logging/logger.rs` (keep this, rename `AppLogger` to `Logger`)

- **Consolidate package management into single service**: `package.service.rs` and `package_deep_clean.service.rs` are duplicates
  - File: `src-tauri/src/services/package_deep_clean.service.rs` (delete)
  - File: `src-tauri/src/routes/package_deep_clean.route.rs` (delete)
  - File: `src-tauri/src/services/mod.rs` (remove deep_clean reference)
  - File: `src-tauri/src/routes/mod.rs` (remove deep_clean reference)

- **Command registration mismatch**: `log_analyzer.route.rs` and `scheduler.route.rs` are imported but their commands not registered
  - File: `src-tauri/src/lib.rs` lines 107-254 (invoke_handler)
  - Either register the commands or remove the imports

- **AppState unused**: `state.rs` defines state but never registered with Tauri builder
  - File: `src-tauri/src/state.rs` (delete if not used)
  - File: `src-tauri/src/lib.rs` (remove state registration if any)

### Frontend

- **Dead frontend services**: `log-analyzer.service.ts` calls non-existent backend commands
  - File: `src/app/services/log-analyzer.service.ts` (delete)
  - File: `src/app/models/log-analyzer.model.ts` (delete)

- **Logger service may be overkill**: `logger.service.ts` (315 lines) with signal-based log buffer may be unnecessary for desktop app
  - File: `src/app/services/logger.service.ts` (review and potentially simplify or delete)

- **Inconsistent service patterns**: Services like `dev-cache.service.ts` use verbose logging while others use simpler patterns
  - Consider creating `BaseService` class with standard error handling

---

## 5. STEP-BY-STEP REFACTORING GUIDE

### Phase 1: Cleanup (Week 1)

1. **Remove dead route files**:
   ```bash
   rm src-tauri/src/routes/log_analyzer.route.rs
   rm src-tauri/src/routes/scheduler.route.rs
   ```

2. **Remove dead service files**:
   ```bash
   rm src-tauri/src/services/scheduler.service.rs
   ```

3. **Remove dead helper/macro files**:
   ```bash
   rm src-tauri/src/helpers/singleton_macro.rs
   rm src-tauri/src/logging/macros.rs
   rm src-tauri/src/logging/logger.rs
   ```

4. **Remove dead infrastructure files**:
   ```bash
   rm src-tauri/src/state.rs
   rm src-tauri/src/middleware.rs
   rm src-tauri/src/dto.rs
   ```

5. **Update mod.rs files** to remove dead module references:
   - Edit `src-tauri/src/routes/mod.rs` - remove `log_analyzer_route` and `scheduler_route`
   - Edit `src-tauri/src/services/mod.rs` - remove `scheduler_service`
   - Edit `src-tauri/src/helpers/mod.rs` - remove `singleton_macro`
   - Edit `src-tauri/src/logging/mod.rs` - remove `macros` and `logger`

6. **Remove duplicate package route/service**:
   ```bash
   rm src-tauri/src/routes/package_deep_clean.route.rs
   rm src-tauri/src/services/package_deep_clean.service.rs
   ```
   - Edit `src-tauri/src/routes/mod.rs` - remove `package_deep_clean_route`
   - Edit `src-tauri/src/services/mod.rs` - remove `package_deep_clean_service`

7. **Consolidate logging** - merge `logger.rs` into `logging/logger.rs`:
   - Copy content from `logger.rs` to `logging/logger.rs`
   - Rename `AppLogger` to `Logger`
   - Update `lib.rs:85` to use consolidated logger
   - Delete `logger.rs`

8. **Remove dead frontend files**:
   ```bash
   rm src/app/services/log-analyzer.service.ts
   rm src/app/models/log-analyzer.model.ts
   ```

### Phase 2: Deduplication (Week 2)

1. **Consolidate process helper functions** in `helpers/process.helper.rs`:
   - Merge `stderr_message` and `stderr_string` into single function
   - Merge `run_command` and `get_command_output` 
   - Merge `pkexec` and `pkexec_with_args`

2. **Consolidate response helpers** in `helpers/response.helper.rs`:
   - Keep `ResponseBuilder` struct
   - Ensure helper functions use consistent patterns

3. **Extract shared installed apps detection**:
   - Create `helpers/installed_apps.rs` with `get_installed_apps()` function
   - Update `app_residue.service.rs` and `repair.service.rs` to use shared helper

4. **Consolidate path validation**:
   - Keep validation in `security/allowlist.rs`
   - Remove duplicate `validate_path` from `validation.helper.rs`

### Phase 3: Architecture (Week 3)

1. **Create base service pattern** for frontend:
   - Create `BaseService` class with standard error handling and logging
   - Update services to extend base class

2. **Review and clean up Cargo.toml**:
   - Remove unused dependencies: `tar`, `flate2`, `rusqlite`, `parking_lot`
   - Keep `tauri-plugin-mcp-bridge` only if MCP bridge feature is needed

3. **Review and clean up package.json**:
   - Remove unused UI dependencies: `flyonui`, `flowbite`

4. **Fix command registration in lib.rs**:
   - Verify all imported routes have commands registered
   - Remove unused imports from `lib.rs` lines 11-80

5. **Create verification script** to ensure no dead commands:
   - Compare routes/mod.rs imports with lib.rs invoke_handler registrations

---

## 6. VERIFICATION COMMANDS

After each phase, run:

```bash
# Rust verification
cd src-tauri && cargo check

# Check for unused imports
cargo clippy -- -D warnings

# Run tests if any exist
cargo test

# Frontend verification
cd .. && bun run ng build

# Check for TypeScript errors
bun run build:frontend:check
```

---

## 7. RISK ASSESSMENT

| Change | Risk Level | Mitigation |
|--------|------------|------------|
| Delete `log_analyzer.route.rs` | Medium | Ensure frontend doesn't call these commands - verified dead code |
| Delete `scheduler.service.rs` | Medium | Verify scheduler functionality not needed in current release |
| Delete `package_deep_clean.*` | High | These are exact duplicates - removal is safe but verify both routes registered |
| Consolidate logging | Medium | Create backup before merging, test logging still works |
| Remove `rusqlite` dependency | High | `HealthHistoryService` uses it - verify health history feature not needed |
| Remove `tar`/`flate2` | Low | Low risk - unlikely to be used |
| Remove frontend services | Medium | Verify no components import these services |
| Consolidate process helpers | Low | Only internal helper functions - low impact |

---

## Summary Statistics

- **Dead Rust files to delete**: 11 files
- **Dead frontend files to delete**: 2 files  
- **Duplicate Rust code**: ~1,200 lines (package services + routes)
- **Duplicate frontend code**: ~300 lines (logging patterns)
- **Unused Cargo dependencies**: 5 crates
- **Unused npm dependencies**: 2 packages
- **Estimated refactoring time**: 3 weeks

