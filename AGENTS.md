# AGENTS.md - Development Guide for Cleanux

This document serves as the primary source of truth for agentic coding agents operating within the Cleanux repository. It outlines the project structure, coding standards, and operational commands to ensure consistency across the Tauri (Rust) backend and Angular frontend.

## 1. Project Overview

Cleanux is a desktop system cleanup application built with **Tauri v2**, using **Rust** for the backend and **Angular v21.1.0** for the frontend. It uses **Bun** as the package manager.

**Package Manager:** Bun v1.3.5

### Key Dependencies
- **Tauri:** v2.11.2
- **Angular:** v21.1.0
- **nosql_orm:** Custom ORM for data persistence
- **TailwindCSS:** v4.1.18

---

## 2. Operational Commands

### Development

- **Start Development Environment:** `bun run tauri:dev` (Runs both Angular and Tauri in dev mode)
- **Start Angular Only:** `bun run start` (available at http://localhost:1420)

### Build Commands (CORRECT)

- **Rust Check:** `cargo check --manifest-path src-tauri/Cargo.toml` (NEVER cargo build for verification)
- **Rust Build:** `cargo build --manifest-path src-tauri/Cargo.toml` (only for actual builds)
- **Rust Build Release:** `cargo build --release --manifest-path src-tauri/Cargo.toml`
- **Build Application:** `bun run tauri:build`
- **Build Angular Only:** `bun run build`
- **Build for Production:** `bun run build:prod`

### Verification (Lint/Test)

- **Rust Check:** `cargo check --manifest-path src-tauri/Cargo.toml`
- **Rust Lint:** `cargo clippy --manifest-path src-tauri/Cargo.toml`
- **Rust Test (All):** `cargo test --manifest-path src-tauri/Cargo.toml`
- **Rust Test (Single):** `cargo test --manifest-path src-tauri/Cargo.toml -- <test_name>`
- **Angular Lint:** `bun run lint`
- **E2E Tests:** `bun run test:e2e`

---

## 3. Backend (Rust) Standards

Located in `src-tauri/`.

### Directory Structure

Organize by type using plural folder names:

- `routes/` (or `commands/`): Command handlers and API endpoints
- `services/`: Business logic (split into modular sub-services where complex)
- `models/`: Data structures (structs/enums)
- `helpers/` or `utils/`: Utility functions
- `repositories/`: Data access layer via nosql_orm

### Naming Conventions

- **Files:** `<kebab-case-base-name>.<singular-folder-derivative>.rs`
  - Example: `models/user.model.rs`, `services/auth.service.rs`
- **Structs/Traits:** PascalCase with singular suffix
  - Example: `UserModel`, `AuthService`
- **Struct Fields:** **camelCase** (required for Angular frontend consistency)
- **Functions/Variables:** **snake_case**

### Response System - CRITICAL

All commands must return `Result<Response<serde_json::Value>, Response<serde_json::Value>>`.

#### Response Type Definition

```rust
// src-tauri/src/models/response.model.rs

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
  Success,
  Info,
  Warning,
  Error,
  Created,
  Updated,
  Deleted,
  ValidationError,
  NotFound,
  Unauthorized,
  Forbidden,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T = Value> {
  pub status: Status,
  pub message: String,
  pub data: T,
}
```

#### Response::success Signature

```rust
Response::success(message: impl Into<String>, data: T) -> Self
```

Example:
```rust
Ok(Response::success("Entity found", doc))
Ok(Response::success("Entities retrieved", docs))
```

#### Response::error Signature

```rust
Response::error(status: Status, message: impl Into<String>) -> Self
where T: Default
```

Example:
```rust
Err(Response::error(Status::Error, e.to_string()))
Err(Response::error(Status::NotFound, "Entity not found"))
```

#### Status Enum Variants

| Variant | Use Case |
|---------|----------|
| `Success` | Standard successful operation |
| `Created` | Resource created successfully |
| `Updated` | Resource updated successfully |
| `Deleted` | Resource deleted successfully |
| `Info` | Informational message |
| `Warning` | Warning condition |
| `Error` | General error |
| `ValidationError` | Input validation failed |
| `NotFound` | Resource not found |
| `Unauthorized` | Authentication required |
| `Forbidden` | Insufficient permissions |

#### ResponseModel Helper Methods

```rust
pub type ResponseModel = Response<Value>;
pub type ResponseStatus = Status;

// Convenience constructors
ResponseModel::created(data: Value) -> Self
ResponseModel::updated(data: Value) -> Self
ResponseModel::deleted(data: Value) -> Self
ResponseModel::validation_error(message: impl Into<String>) -> Self
ResponseModel::not_found(entity: &str) -> Self
ResponseModel::unauthorized() -> Self
ResponseModel::forbidden() -> Self
```

---

## 4. CRUD Macros

Cleanux uses generated macros for standard CRUD operations. These are defined in `src-tauri/src/commands/macros.rs` and invoked from command files.

### Available Macros

| Macro | Generates | Table Access |
|-------|-----------|--------------|
| `crud_get_command!` | `get_<entity>` | `find_by_id` |
| `crud_get_all_command!` | `get_<entities>` | `find_many` |
| `crud_create_command!` | `create_<entity>` | `insert` |
| `crud_update_command!` | `update_<entity>` | `update` |
| `crud_delete_command!` | `delete_<entity>` | `delete` |
| `crud_patch_command!` | `patch_<entity>` | `patch` |

### Macro Usage Examples

```rust
// In src-tauri/src/commands/profile.command.rs

use crate::crud_get_command;
use crate::crud_get_all_command;
use crate::crud_create_command;
use crate::crud_update_command;
use crate::crud_delete_command;

crud_get_command!(get_cleaning_profile, "cleaning_profiles");
crud_get_all_command!(get_cleaning_profiles, "cleaning_profiles");
crud_create_command!(create_cleaning_profile, "cleaning_profiles");
crud_update_command!(update_cleaning_profile, "cleaning_profiles");
crud_delete_command!(delete_cleaning_profile, "cleaning_profiles");
```

### Generated Command Signature

Each macro generates a `#[tauri::command(rename_all = "camelCase")]` async function that takes:
- `state: tauri::State<'_, AppState>` - Application state with repository access
- Operation-specific parameters (e.g., `id: Option<String>`, `data: serde_json::Value`)

### State Access Pattern

Commands access data through `state.data.repository_service`:

```rust
let doc = state
  .data
  .repository_service
  .find_by_id($table, &id)
  .await
  .map_err(|e| Response::error(Status::Error, e.to_string()))?
  .ok_or_else(|| Response::error(Status::NotFound, "Entity not found"))?;
```

---

## 5. Frontend (Angular) Standards

Located in `src/app/`.

### Directory Structure

Organize by type using plural folder names:

- `components/`: Reusable UI elements
- `views/` or `pages/`: Page-level structures
- `entities/` or `models/`: Interfaces and data classes
- `services/`: Business logic and API interaction
- `shared/`: Shared utilities, components, and services
- `api/`: API service wrappers (e.g., `TauriApiService`)
- `core/`: Core application services

### Component Structure

- **Subfolders:** Every component MUST have its own subfolder: `components/<base-name>/`
- **Separation:** Logic in `<base-name>.component.ts`, template in `<base-name>.component.html`
- **Standalone:** All components should be standalone

### Naming Conventions

- **Files:** `<kebab-case-base-name>.<singular-folder-derivative>.ts`
- **Classes:** PascalCase with singular suffix (e.g., `UserComponent`, `ProfileService`)

### API Service Pattern

Cleanux uses `TauriApiService` which wraps `invoke` and handles response unwrapping:

```typescript
// src/app/api/tauri-api.service.ts
@Injectable({ providedIn: 'root' })
export class TauriApiService {
  async invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
    const response = await invoke<Response>(command, args);
    if (response.status === 'success') {
      return getData<T>(response) as T;
    } else {
      throw new ApiException(response.message || `Operation failed: ${command}`, command);
    }
  }
}
```

### CrudService Pattern

Use `CrudService` for standard entity operations:

```typescript
// src/app/services/crud.service.ts
@Injectable({ providedIn: 'root' })
export class CrudService {
  constructor(private api: TauriApiService) {}

  async execute<T>(operation: string, entity: string, params: CrudParams = {}): Promise<T> {
    return this.api.invoke<T>('crud_execute', { operation, entity, ...params });
  }

  async get<T>(entity: string, id: string): Promise<T> { /* ... */ }
  async getAll<T>(entity: string, filter?: unknown): Promise<T[]> { /* ... */ }
  async create<T>(entity: string, data: unknown): Promise<T> { /* ... */ }
  async update<T>(entity: string, id: string, data: unknown): Promise<T> { /* ... */ }
  async delete(entity: string, id: string): Promise<void> { /* ... */ }
}
```

### Response Model (Frontend)

```typescript
// src/app/entities/response.model.ts
export type ResponseStatus =
  | 'success' | 'info' | 'warning' | 'error'
  | 'created' | 'updated' | 'deleted'
  | 'validationError' | 'notFound' | 'unauthorized' | 'forbidden';

export interface Response<T = unknown> {
  status: ResponseStatus;
  message: string;
  data: T;
}

export function isSuccess<T>(r: Response<T>): boolean {
  return ['success', 'created', 'updated', 'deleted'].includes(r.status);
}

export function getData<T>(response: Response<unknown>): T | null {
  return response.data as T ?? null;
}
```

### Coding Practices

- **Modern Angular:** Use Signals for state management and Control Flow (`@if`, `@for`, `@switch`)
- **Type Safety:** Avoid `any`. Use interfaces for all API responses
- **Standalone:** All components should be standalone

---

## 6. tauri-mcp Tool Usage (CRITICAL)

**CONNECTING TO RUNNING APPS ONLY**:
- `tauri-mcp_driver_session`: Connect to ALREADY RUNNING Tauri app
- **NEVER** kill a Tauri process with this tool
- **NEVER** try to start/run the app - frontend won't be running
- The app must be started separately (e.g., by user or another process)
- After connecting, you can use `tauri-mcp_webview_*` and `tauri-mcp_ipc_*` tools

**Workflow**:
1. User starts the Tauri app separately (`bun run tauri:dev` or similar)
2. Agent connects via `tauri-mcp_driver_session` with action: "start"
3. Agent uses webview/ipc tools to interact
4. **NEVER** use driver_session to stop/kill the app process

**Available Tools**:
- `tauri-mcp_driver_session` - Connect to running app (action: "start")
- `tauri-mcp_webview_dom_snapshot` - Get UI structure
- `tauri-mcp_webview_find_element` - Find elements
- `tauri-mcp_webview_interact` - Click, type, scroll
- `tauri-mcp_ipc_execute_command` - Call Rust backend commands
- `tauri-mcp_ipc_monitor` - Monitor IPC calls
- `tauri-mcp_manage_window` - Window management

### Connection

```javascript
// Start session before any other tools
tauri-mcp_driver_session({ action: "start" })
```

### Webview Tools

| Tool | Purpose |
|------|---------|
| `tauri-mcp_webview_dom_snapshot` | Get DOM structure (accessibility/structure) |
| `tauri-mcp_webview_find_element` | Find element by CSS/XPath/text |
| `tauri-mcp_webview_interact` | Click, scroll, swipe, type |
| `tauri-mcp_webview_screenshot` | Capture viewport screenshot |
| `tauri-mcp_webview_keyboard` | Type text or send key events |
| `tauri-mcp_webview_wait_for` | Wait for element/text/ipc-event |

### IPC Tools

| Tool | Purpose |
|------|---------|
| `tauri-mcp_ipc_execute_command` | Invoke Rust backend command |
| `tauri-mcp_ipc_monitor` | Capture invoke() calls and responses |
| `tauri-mcp_ipc_emit_event` | Emit Tauri event to frontend |
| `tauri-mcp_ipc_get_backend_state` | Get app metadata and Tauri version |

### Window Management

| Tool | Purpose |
|------|---------|
| `tauri-mcp_manage_window` | list/info/resize windows |

### Logs

| Tool | Purpose |
|------|---------|
| `tauri-mcp_read_logs` | Read console/android/ios/system logs |

### Example Usage

```javascript
// Interact with webview
tauri-mcp_webview_find_element({ selector: "#submit-button", strategy: "css" })
tauri-mcp_webview_interact({ action: "click", selector: "#submit-button" })

// Invoke backend command
tauri-mcp_ipc_execute_command({ command: "get_version" })

// Monitor IPC
tauri-mcp_ipc_monitor({ action: "start" })
// ... perform actions ...
tauri-mcp_ipc_get_captured({ filter: "get_" })
```

---

## 7. Project-Specific Patterns

### Data Persistence

Cleanux uses `nosql_orm` with JSON file storage. The `RepositoryService` provides CRUD operations:

```rust
// Available repository methods
repository_service.find_by_id(table: &str, id: &str) -> Result<Option<Value>, Error>
repository_service.find_many(table: &str, filter: Option<Value>, page: Option<u64>, limit: Option<u64>, ...) -> Result<Vec<Value>, Error>
repository_service.insert(table: &str, data: Value) -> Result<Value, Error>
repository_service.update(table: &str, id: &str, data: Value) -> Result<Value, Error>
repository_service.patch(table: &str, id: &str, patch: Value) -> Result<Value, Error>
repository_service.delete(table: &str, id: &str) -> Result<(), Error>
```

### AppState Structure

```rust
pub struct AppState {
  pub data: Arc<Data>,
  pub crud_service: Arc<services::crud_service::CrudService>,
  pub repository_service: Arc<repositories::service::RepositoryService>,
}
```

### Custom Commands

For non-CRUD operations (e.g., system cleaning, file operations), create dedicated command files:

```rust
// Example: src-tauri/src/commands/kernel_cleaner.command.rs
#[tauri::command(rename_all = "camelCase")]
pub async fn get_current_kernel() -> Result<Response, String> {
  // Implementation
  Ok(Response::success("Current kernel retrieved", kernel_version))
}
```

### Error Handling

Return `Result<Response, String>` for simple errors or `Result<Response, Response>` when you need to return structured error responses:

```rust
// Simple error (String)
Result<Response, String>

// Structured error (Response)
Result<Response<serde_json::Value>, Response<serde_json::Value>>
```

---

## 8. General Rules

- **Manual Creation:** Do not use `ng generate` or automated scaffolding; create files manually
- **Path Aliases:** Use `@components/*`, `@services/*`, `@entities/*`, etc.
- **Indentation:** 2-space indent
- **No Test Files:** Do not create `.spec.ts` or `_test.rs` files unless explicitly requested
