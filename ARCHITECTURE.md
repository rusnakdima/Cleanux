# Cleanux Architecture

## Overview

Cleanux is a system cleanup utility built with Tauri (Rust backend + Angular frontend). The architecture follows a layered approach with clear separation between UI, business logic, and system interaction.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Angular Frontend                     │
│  (Components, Services, Routes, State Management)       │
└─────────────────────────────────────────────────────────┘
                            │
                    Tauri IPC (invoke)
                            │
┌─────────────────────────────────────────────────────────┐
│                    Rust Backend                          │
│  (Tauri Commands, Services, Business Logic)              │
└─────────────────────────────────────────────────────────┘
                            │
                    System APIs
                            │
         ┌──────────────────┼──────────────────┐
         ▼                  ▼                  ▼
    File System        System Services      Process Mgmt
    (fs, walkdir)      (systemctl)         (sysinfo)
```

## Frontend (Angular)

Located in `src/` directory.

### Structure

```
src/
├── app/                    # Main application code
│   ├── components/         # Reusable UI components
│   ├── pages/             # Route-based page components
│   ├── services/          # Angular services for API calls
│   └── models/            # TypeScript interfaces
├── assets/                 # Static assets (images, icons)
├── environments/            # Environment-specific configs
└── main.ts                 # Application entry point
```

### Key Technologies

- **Angular 21** - UI framework
- **RxJS** - Reactive programming
- **Material/CDK** - UI components
- **Tailwind CSS** - Styling

## Backend (Rust)

Located in `src-tauri/src/` directory.

### Module Structure

```
src-tauri/src/
├── lib.rs              # Library root, Tauri builder setup
├── main.rs             # Binary entry point
├── commands/           # Tauri command handlers (deprecated)
├── dto.rs              # Data Transfer Objects
├── errors/             # Error type definitions
├── helpers/            # Utility functions
├── middleware/         # Request/response middleware
├── models/             # Data models and types
├── routes/             # Tauri command route definitions
├── security/           # Security utilities (path validation, allowlists)
├── services/           # Business logic services
└── state.rs            # Application state management
```

### Key Patterns

#### Error Handling

Uses `thiserror` for error types defined in `errors/mod.rs`:

```rust
pub enum AppError {
    Io(#[from] std::io::Error),
    InvalidPath(String),
    PermissionDenied(String),
    // ...
}
```

Errors are converted to `ResponseModel` via the `Into<ResponseModel>` trait.

#### Response Model

All Tauri commands return `Result<ResponseModel, ResponseModel>`:

```rust
pub struct ResponseModel {
    pub status: ResponseStatus,  // Success, Info, Warning, Error
    pub message: String,
    pub data: DataValue,         // String, Number, Bool, Array, Object
}
```

#### Path Security

The `security/` module provides path validation:

- `allowlist.rs` - Defines allowed paths (`/home`, `/tmp`, `/var/cache`, etc.)
- `is_path_allowed()` - Checks if a path is within allowed directories
- `validate_path()` - Full path validation with security checks

## Services Layer

Located in `src-tauri/src/services/`.

### Key Services

| Service                     | Purpose                                 |
| --------------------------- | --------------------------------------- |
| `cleaner.service.rs`        | Orchestrates cache, trash, log cleaning |
| `cache_cleaning.service.rs` | Cache file detection and cleaning       |
| `trash_cleaning.service.rs` | Trash management                        |
| `log_cleaning.service.rs`   | System log analysis and cleaning        |
| `system.service.rs`         | Systemd service management              |
| `process.service.rs`        | Process listing and killing             |
| `scanner.service.rs`        | File scanning with duplicate detection  |
| `directory.service.rs`      | Directory size analysis                 |

### Service Pattern

Services are typically structs with `impl` blocks containing methods:

```rust
pub struct CleanerService;

impl CleanerService {
    pub fn getCacheFiles(&self, limit: Option<usize>, offset: Option<usize>)
        -> Result<ResponseModel, ResponseModel> { ... }
}
```

## Routes (Tauri Commands)

Located in `src-tauri/src/routes/`. Each route file exposes Tauri commands that wrap service calls.

Example pattern:

```rust
#[tauri::command]
#[allow(non_snake_case)]
pub fn getCacheFiles(
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<ResponseModel, ResponseModel> {
    CleanerService.getCacheFiles(limit, offset)
}
```

Commands are registered in `lib.rs` via `tauri::generate_handler![]`.

## Security Model

### Path Allowlist

Allowed paths:

- `/home` (and subdirectories)
- `/tmp`
- `/var/cache`
- `/var/tmp`
- `/var/log`
- `/snap`
- `/srv`
- `/opt`
- User's Trash directory

Blocked paths:

- `/proc` (except `/proc/self`, `/proc/curproc`)
- `/sys`
- `/dev`

### Path Validation

`validate_path()` in `security/allowlist.rs`:

1. Checks for null bytes
2. Checks for path traversal (`..`)
3. Canonicalizes and validates existence
4. Checks against allowlist
5. Rejects symlinks

## State Management

Application state is managed via Tauri state's `AppState` in `state.rs`.

```rust
pub struct AppState {
    pub scanning: bool,
    pub current_scan_path: Option<PathBuf>,
    // ...
}
```

State is registered with: `app.manage(state::AppState::new())`

## CI/CD Pipeline

GitHub Actions workflows in `.github/workflows/`:

### CI Workflow (`ci.yml`)

- Rust formatting check (`cargo fmt`)
- Rust linting (`cargo clippy`)
- Rust unit tests (`cargo test`)
- Frontend build check (`ng build`)

### Release Workflow (`release.yml`)

- Builds on Ubuntu with all dependencies
- Creates `.deb` and `.AppImage` packages
- Creates draft GitHub release

## Dependencies

### Rust Dependencies

Key crates:

- `tauri` - Application framework
- `serde` / `serde_json` - Serialization
- `sysinfo` - System information
- `walkdir` - Directory traversal
- `rayon` - Parallel processing
- `thiserror` - Error handling
- `chrono` - Date/time

### Frontend Dependencies

Key packages:

- `@angular/*` - Angular framework
- `@tauri-apps/api` - Tauri frontend API
- `rxjs` - Reactive extensions
- `@angular/material` - UI components
- `tailwindcss` - Styling

## Testing Strategy

### Rust Tests

- Unit tests within each module (`.rs` files with `#[cfg(test)]`)
- Integration tests in `src-tauri/tests/`
- Run with `cargo test`

### Frontend Testing

- Build validation serves as basic check
- Angular testing infrastructure not fully configured

## Build Process

1. Frontend is built with Angular (`ng build`)
2. Tauri packages the frontend with the Rust binary
3. Platform-specific bundles are created (`.deb`, `.AppImage`)
