# Agent Code Writing Guidelines

## Tauri + Angular v20 + TailwindCSS v4

---

## Abstract

This document defines rules for writing high-quality, maintainable, and scalable code in Tauri applications. These rules are derived from analysis of production-grade Tauri projects and ensure:

- **Code Quality**: Clean, readable, type-safe code without `any` or `console.error`
- **No Redundancy**: DRY principle - no duplicated code, extract to shared utilities
- **No Dead Code**: Remove unused imports, functions, services, components
- **Reusable Algorithms**: Common patterns extracted to services/utilities
- **Correct Architecture**: Separation of concerns, proper layering
- **No Problems**: Predictable behavior, proper error handling, no anti-patterns

---

## 1. Project Architecture

### 1.1 Directory Structure

```
project/
├── src/                              # Angular frontend
│   └── app/
│       ├── api/
│       │   └── tauri-api.service.ts # Tauri invoke wrapper
│       ├── features/                 # Feature modules (self-contained)
│       │   └── [feature]/
│       │       ├── components/      # UI only - NO business logic, NO CSS files
│       │       │   ├── name.component.ts
│       │       │   └── name.component.html
│       │       ├── services/        # Business logic only
│       │       │   └── name.service.ts
│       │       ├── models/           # TypeScript interfaces
│       │       │   └── name.model.ts
│       │       └── store/           # Signal-based state
│       │           └── name.store.ts
│       ├── shared/                   # Cross-feature shared
│       │   ├── components/          # Reusable UI components (NO CSS files)
│       │   ├── services/            # Common services
│       │   │   └── logging.service.ts
│       │   ├── models/             # Shared types
│       │   │   └── settings.model.ts
│       │   ├── pipes/
│       │   ├── directives/
│       │   └── utils/
│       ├── guards/
│       ├── interceptors/
│       ├── providers/
│       ├── app.config.ts
│       ├── app.routes.ts
│       └── app.ts
├── src-tauri/                        # Rust backend
│   └── src/
│       ├── commands/                # Tauri command handlers (thin wrappers)
│       │   ├── item.route.rs
│       │   └── mod.rs
│       ├── services/               # Business logic
│       │   └── name.service.rs
│       ├── models/                 # Rust types
│       │   └── name.model.rs
│       ├── errors/                  # Error types
│       │   └── mod.rs
│       ├── helpers/                 # Utility functions
│       │   └── name.helper.rs
│       ├── entities/               # Domain entities
│       │   └── name.entity.rs
│       ├── logger.rs               # Centralized logging with configurable levels
│       ├── state.rs                # AppState definition
│       ├── lib.rs                  # Module aggregation + entry
│       └── main.rs                 # Binary entry
└── styles.css                       # ONE global TailwindCSS file - NO component CSS files
```

### 1.2 Core Principle: Separation of Concerns

| Layer          | Responsibility                                 | Forbidden                                 |
| -------------- | ---------------------------------------------- | ----------------------------------------- |
| **Components** | UI rendering, template logic                   | API calls, business logic, state mutation |
| **Services**   | Business logic, API calls, data transformation | UI code, direct DOM manipulation          |
| **Stores**     | Reactive state via signals                     | Business logic, API calls                 |
| **Models**     | Data shapes (interfaces/structs)               | Logic                                     |
| **Commands**   | Thin wrappers delegating to services           | Business logic                            |

**Rule: Never mix layers in the same file.**

---

## 2. Angular Patterns

### 2.1 Signal-Based State (MANDATORY)

Use Angular Signals for ALL reactive state. Do not use `BehaviorSubject`, `Subject`, or `Observable` for new code.

```typescript
// ✅ CORRECT - Signal-based store
@Injectable()
export class ItemStore {
  // Mutable state
  readonly items = signal<Item[]>([]);
  readonly loading = signal(false);
  readonly selectedIds = signal<Set<string>>(new Set());

  // Derived state
  readonly itemCount = computed(() => this.items().length);
  readonly selectedItems = computed(() =>
    this.items().filter((item) => this.selectedIds().has(item.id))
  );
  readonly totalSize = computed(() => this.items().reduce((sum, item) => sum + item.size, 0));

  // State mutations return void, update signals
  loadItems(): void {
    this.loading.set(true);
    // ... fetch and update items signal
  }

  toggleSelection(id: string): void {
    this.selectedIds.update((set) => {
      const next = new Set(set);
      next.has(id) ? next.delete(id) : next.add(id);
      return next;
    });
  }
}
```

```typescript
// ❌ WRONG - BehaviorSubject (legacy pattern)
@Injectable()
export class BadStore {
  private items$ = new BehaviorSubject<Item[]>([]); // FORBIDDEN
  private loading$ = new BehaviorSubject(false); // FORBIDDEN
}
```

### 2.2 Component Pattern (MANDATORY)

Components are **dumb** - they only render UI and emit events.

**TypeScript file (.ts) - NO styling, NO business logic:**

```typescript
// ✅ CORRECT - name.component.ts
@Component({
  selector: "app-item-list",
  templateUrl: "./item-list.component.html",
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ItemListComponent {
  @Input({ required: true }) items = input<Item[]>();
  @Input() loading = input(false);
  @Output() itemSelect = output<Item>();

  trackByItem(index: number, item: Item): string {
    return item.id;
  }
}
```

**HTML file (.html) - ALL TailwindCSS classes go here:**

```html
<!-- ✅ CORRECT - TailwindCSS classes ONLY in HTML -->
<div class="p-4 bg-card rounded-xl border border-border">
  <h2 class="text-lg font-semibold text-text-primary">Items</h2>

  @if (loading()) {
  <div class="flex justify-center py-8">
    <span class="text-text-secondary">Loading...</span>
  </div>
  } @else { @for (item of items(); track trackByItem($index, item)) {
  <app-item-row [item]="item" class="mt-2 block" (select)="itemSelect.emit($event)" />
  } @empty {
  <div class="py-8 text-center text-text-secondary">No items found</div>
  } }
</div>
```

```typescript
// ❌ WRONG - Business logic in component
@Component({
  template: `<div class="bg-accent">{{ getData() }}</div>`, // FORBIDDEN
})
export class BadComponent {
  async getData() {
    // FORBIDDEN - business logic in component
    await this.api.invoke("get_data"); // FORBIDDEN
  }
}

// ❌ WRONG - Inline template instead of templateUrl
@Component({
  selector: "app-bad",
  template: '<div class="p-4">Content</div>', // FORBIDDEN - use templateUrl
  styles: [".class { color: red }"], // FORBIDDEN - no component CSS
})
export class BadComponent {}
```

### 2.3 Service Pattern (MANDATORY)

Services handle ALL business logic and API communication.

```typescript
// ✅ CORRECT - name.service.ts
@Injectable({ providedIn: "root" })
export class ItemService {
  private api = inject(TauriApiService);
  private store = inject(ItemStore);
  private errorHandler = inject(ErrorHandlerService);

  async loadItems(): Promise<void> {
    try {
      const data = await this.api.invoke<Item[]>("get_items");
      this.store.items.set(data);
    } catch (error) {
      this.errorHandler.handleError(error, "ItemService.loadItems");
    }
  }

  async deleteItem(id: string): Promise<void> {
    try {
      await this.api.invoke("delete_item", { id });
      this.store.items.update((items) => items.filter((i) => i.id !== id));
    } catch (error) {
      this.errorHandler.handleError(error, "ItemService.deleteItem");
      throw error;
    }
  }
}
```

### 2.4 name.store.ts Pattern

```typescript
// ✅ CORRECT - name.store.ts
@Injectable()
export class ItemStore {
  // Mutable state
  readonly items = signal<Item[]>([]);
  readonly loading = signal(false);
  readonly selectedIds = signal<Set<string>>(new Set());

  // Derived state
  readonly itemCount = computed(() => this.items().length);
  readonly selectedItems = computed(() =>
    this.items().filter((item) => this.selectedIds().has(item.id))
  );
  readonly totalSize = computed(() => this.items().reduce((sum, item) => sum + item.size, 0));

  // State mutations return void, update signals
  loadItems(): void {
    this.loading.set(true);
    // ... fetch and update items signal
  }

  toggleSelection(id: string): void {
    this.selectedIds.update((set) => {
      const next = new Set(set);
      next.has(id) ? next.delete(id) : next.add(id);
      return next;
    });
  }
}
```

### 2.4 Dependency Injection

**Preferred: inject() function**

```typescript
@Injectable({ providedIn: "root" })
export class MyService {
  private api = inject(TauriApiService); // ✅ PREFERRED
  private errorHandler = inject(ErrorHandlerService);
}
```

**Acceptable: Constructor injection (for testing)**

```typescript
@Injectable({ providedIn: "root" })
export class MyService {
  constructor(
    private api: TauriApiService, // ✅ ACCEPTABLE
    private errorHandler: ErrorHandlerService
  ) {}
}
```

---

## 3. Tauri/Rust Backend Patterns

### 3.1 AppState Organization

**Principle: Focused state, not monolithic. Group by domain.**

```rust
// ✅ CORRECT - Focused state by domain
pub struct AppState {
    pub logger: Logger,
    pub cache_service: Arc<CacheCleaningService>,
    pub dashboard_service: Arc<DashboardService>,
    pub app_handle: Arc<TokioMutex<AppHandle>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let logger = Logger::new();
            let cache_service = Arc::new(CacheCleaningService::new(logger.clone()));

            app.manage(AppState {
                logger,
                cache_service,
                app_handle: Arc::new(TokioMutex::new(app.handle().clone())),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_cache_summary,
            clean_cache
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

```rust
// ❌ WRONG - Monolithic state with 20+ services crammed together
pub struct AppState {
    pub service_a: Arc<ServiceA>,
    pub service_b: Arc<ServiceB>,
    pub service_c: Arc<ServiceC>,
    pub service_d: Arc<ServiceD>,
    // ... 20 more - FORBIDDEN
}
```

### 3.2 Command Handler Pattern

**Principle: Commands are thin wrappers. All business logic goes to services.**

```rust
// ✅ CORRECT - name.route.rs - Thin command wrapper
#[tauri::command]
pub async fn get_cache_summary(
    state: State<'_, AppState>,
) -> Result<ResponseModel, ResponseModel> {
    state.cache_service.get_summary().await
}
```

```rust
// ❌ WRONG - Business logic in command handler
#[tauri::command]
pub fn bad_command() -> Result<ResponseModel, ResponseModel> {
    // Complex logic here - FORBIDDEN, should be in service
    let data = fetch_data()?;
    let processed = process_data(data)?;
    Ok(ResponseModel::success(processed))
}
```

### 3.3 name.service.rs Pattern

**Principle: All business logic in services.**

```rust
// ✅ CORRECT - name.service.rs
pub struct CacheCleaningService {
    logger: Logger,
}

impl CacheCleaningService {
    pub fn new(logger: Logger) -> Self {
        Self { logger }
    }

    pub async fn get_summary(&self) -> Result<ResponseModel, ResponseModel> {
        self.logger.debug("Getting cache summary");
        // ... business logic
        Ok(ResponseModel::success(summary))
    }

    pub async fn clean(&self, paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
        self.logger.info("Cleaning cache files");
        // ... business logic
    }
}
```

### 3.4 response.model.rs Pattern

**Principle: Consistent response structure across ALL commands.**

```rust
// ✅ CORRECT - models/response.model.rs
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Error,
    Info,
    Warning,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum DataValue {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<serde_json::Value>),
    Object(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseModel {
    pub status: Status,
    pub message: String,
    pub data: DataValue,
}

impl<T> ResponseModel<T> {
    pub fn success(data: T) -> Self { ... }
    pub fn error(message: impl Into<String>) -> Self { ... }
}
```

### 3.5 state.rs Pattern

```rust
// ✅ CORRECT - state.rs
pub struct AppState {
    pub logger: Logger,
    pub cache_service: Arc<CacheCleaningService>,
    pub dashboard_service: Arc<DashboardService>,
    pub app_handle: Arc<TokioMutex<AppHandle>>,
}
```

---

## 4. API Layer Pattern

### 4.1 tauri-api.service.ts

**Principle: Single entry point for all Tauri invokes.**

```typescript
// ✅ CORRECT - api/tauri-api.service.ts
const DEFAULT_TIMEOUT_MS = 30000;

export interface InvokeOptions {
  timeoutMs?: number;
  suppressError?: boolean;
}

@Injectable({ providedIn: "root" })
export class TauriApiService {
  async invoke<T>(
    command: string,
    args?: Record<string, unknown>,
    options: InvokeOptions = {}
  ): Promise<T> {
    const timeoutMs = options.timeoutMs ?? DEFAULT_TIMEOUT_MS;

    try {
      const response = await Promise.race([
        invoke<Response<T>>(command, args),
        new Promise<never>((_, reject) =>
          setTimeout(
            () => reject(new Error(`Command "${command}" timed out after ${timeoutMs}ms`)),
            timeoutMs
          )
        ),
      ]);

      if (response.status === "success") {
        return response.data as T;
      } else {
        throw new ApiException(response.message || `Operation failed: ${command}`, command);
      }
    } catch (error: unknown) {
      if (!options.suppressError) {
        console.error(`Error invoking command "${command}":`, error);
      }
      if (error instanceof ApiException) {
        throw error;
      }
      throw new ApiException(
        error instanceof Error ? error.message : String(error),
        command,
        error
      );
    }
  }
}
```

**Rule: ALL `invoke()` calls go through TauriApiService. Never call `invoke()` directly in components or services.**

### 4.2 invoke-wrapper.util.ts (for cancellation)

```typescript
// ✅ CORRECT - shared/utils/invoke-wrapper.util.ts
export async function invokeWithAbortHandling<T>(
  invokeFn: () => Promise<T>,
  context: string,
  errorHandler: ErrorHandlerService
): Promise<T> {
  try {
    return await invokeFn();
  } catch (e) {
    if (e instanceof Error && e.name === "AbortError") {
      throw new Error("Operation cancelled");
    }
    errorHandler.handleError(e, context);
    throw e;
  }
}
```

---

## 5. Error Handling

### 5.1 error.model.ts - Typed Error System

**Principle: Errors are typed, categorized, and user-facing.**

```typescript
// ✅ CORRECT - models/error.model.ts
export enum ErrorCode {
  UNKNOWN = "UNKNOWN",
  NETWORK_ERROR = "NETWORK_ERROR",
  VALIDATION_ERROR = "VALIDATION_ERROR",
  NOT_FOUND = "NOT_FOUND",
  UNAUTHORIZED = "UNAUTHORIZED",
  TIMEOUT = "TIMEOUT",
  CONNECTION_FAILED = "CONNECTION_FAILED",
}

export interface AppError {
  code: ErrorCode;
  message: string; // Technical message for logging
  userMessage: string; // User-facing message
  originalError?: unknown;
  timestamp: Date;
  retryable: boolean;
}

export interface RetryConfig {
  maxAttempts: number;
  delayMs: number;
  backoffMultiplier: number;
}

export const DEFAULT_RETRY_CONFIG: RetryConfig = {
  maxAttempts: 3,
  delayMs: 1000,
  backoffMultiplier: 2,
};
```

### 5.2 error-handler.service.ts

**Principle: Centralized error processing with retry support.**

```typescript
// ✅ CORRECT - services/error-handler.service.ts
@Injectable({ providedIn: 'root' })
export class ErrorHandlerService {
  private toastService = inject(ToastService);
  private logsSignal = signal<ErrorLogEntry[]>([]);
  readonly logs = computed(() => this.logsSignal());

  handleError(error: unknown, context?: string): AppError {
    const appError = this.convertToAppError(error);
    this.logError(appError, context);

    if (!appError.retryable) {
      this.toastService.error(appError.userMessage);
    }

    return appError;
  }

  async retry<T>(
    operation: () => Promise<T>,
    config: Partial<RetryConfig> = {}
  ): Promise<T> {
    const { maxAttempts, delayMs, backoffMultiplier } = {
      ...DEFAULT_RETRY_CONFIG,
      ...config,
    };

    for (let attempt = 1; attempt <= maxAttempts; attempt++) {
      try {
        return await operation();
      } catch (error) {
        const lastError = this.handleError(error);
        if (!lastError.retryable || attempt === maxAttempts) {
          throw lastError;
        }
        await this.delay(delayMs * Math.pow(backoffMultiplier, attempt - 1));
      }
    }
    throw new Error('Unreachable');
  }

  private convertToAppError(error: unknown): AppError { ... }
  private logError(error: AppError, context?: string): void { ... }
  private delay(ms: number): Promise<void> { ... }
}
```

### 5.3 Error Propagation Rule

**NEVER silently catch errors. ALWAYS use ErrorHandlerService.**

```typescript
// ❌ WRONG - Silent catch
try {
  await doSomething();
} catch {
  // Silent failure - FORBIDDEN
}

// ❌ WRONG - Empty catch
try {
  await doSomething();
} catch (error) {
  // Do nothing - FORBIDDEN
}

// ❌ WRONG - console.error without handler
try {
  await doSomething();
} catch (error) {
  console.error("Error:", error); // FORBIDDEN - use ErrorHandlerService
}

// ✅ CORRECT - Handle and propagate
try {
  await doSomething();
} catch (error) {
  this.errorHandler.handleError(error, "Service.method");
  throw error; // Re-throw if caller needs to handle
}
```

---

## 6. Styling - TailwindCSS v4 Only

### 6.1 Core Rules

| Rule                         | Description                                            |
| ---------------------------- | ------------------------------------------------------ |
| **ONE global CSS file**      | `styles.css` - single source for all CSS               |
| **TailwindCSS in HTML only** | All utility classes in `.html` template files          |
| **No component CSS files**   | No `*.component.css`, no `styles: [...]` in @Component |
| **No inline styles**         | No `style="..."` attributes in HTML                    |
| **Pure TailwindCSS**         | Only TailwindCSS utility classes, no custom CSS        |

### 6.2 Global CSS File

```css
/* styles.css - ONLY CSS file in project */
@tailwind base;
@tailwind components;
@tailwind utilities;

@theme {
  --color-accent: #10b981;
  --color-accent-hover: #059669;
  --color-card: #1a1a2e;
  --color-border: #2d2d44;
  --color-text-primary: #f1f5f9;
  --color-text-secondary: #94a3b8;
  --color-success: #22c55e;
  --color-warning: #f97316;
  --color-error: #ef4444;
}
```

### 6.3 Dark/Light Mode via CSS Variables

```css
.dark {
  --bg-primary: #0a0a0f;
  --bg-secondary: #12121a;
  --bg-card: #1a1a2e;
  --text-primary: #f1f5f9;
  --text-secondary: #94a3b8;
}

.light {
  --bg-primary: #f8fafc;
  --bg-secondary: #f1f5f9;
  --bg-card: #ffffff;
  --text-primary: #0f172a;
  --text-secondary: #475569;
}
```

### 6.4 Component Templates - TailwindCSS Only

**TypeScript file (.ts) - NO styling:**

```typescript
// ✅ CORRECT - No styling in .ts file
@Component({
  selector: "app-item-card",
  templateUrl: "./item-card.component.html",
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ItemCardComponent {
  @Input({ required: true }) item = input<Item>();
  @Output() action = output<void>();
}
```

**HTML file (.html) - TailwindCSS utility classes ONLY:**

```html
<!-- ✅ CORRECT - Pure TailwindCSS utility classes -->
<div class="p-4 bg-card rounded-xl border border-border">
  <h3 class="text-lg font-semibold text-text-primary">{{ item().name }}</h3>
  <p class="mt-2 text-sm text-text-secondary">{{ item().description }}</p>
  <button
    class="mt-4 px-4 py-2 bg-accent text-white rounded-lg
           hover:bg-accent-hover transition-colors duration-200"
    (click)="action.emit()"
  >
    Action
  </button>
</div>
```

### 6.5 Forbidden Styling Patterns

```html
<!-- ❌ FORBIDDEN - Inline styles -->
<div style="background: red; padding: 16px;">Wrong</div>

<!-- ❌ FORBIDDEN - Arbitrary values not in theme -->
<div class="bg-[#123456]">Wrong</div>

<!-- ❌ FORBIDDEN - Custom component CSS files -->
<!-- styles: ['./item-card.component.css'] in @Component - WRONG -->
```

### 6.6 Semantic Color Usage

Always use semantic color names from theme:

```html
<!-- ✅ CORRECT - Semantic colors from theme -->
<div class="bg-accent text-text-primary border-border">Styled</div>

<!-- ❌ WRONG - Hardcoded colors -->
<div class="bg-[#10b981] text-[#f1f5f9]">Wrong</div>
```

---

## 7. Naming Conventions

### 7.1 TypeScript Files

| Type           | Pattern               | Example                                 |
| -------------- | --------------------- | --------------------------------------- |
| Services       | `name.service.ts`     | `item.service.ts`, `logging.service.ts` |
| Stores         | `name.store.ts`       | `item.store.ts`                         |
| Components     | `name.component.ts`   | `item-card.component.ts`                |
| HTML Templates | `name.component.html` | `item-card.component.html`              |
| Models         | `name.model.ts`       | `item.model.ts`, `settings.model.ts`    |
| Utils          | `name.util.ts`        | `format.util.ts`                        |
| Pipes          | `name.pipe.ts`        | `format-bytes.pipe.ts`                  |
| Directives     | `name.directive.ts`   | `auto-focus.directive.ts`               |
| Guards         | `name.guard.ts`       | `auth.guard.ts`                         |
| Interceptors   | `name.interceptor.ts` | `error.interceptor.ts`                  |
| Config         | `name.config.ts`      | `app.config.ts`                         |
| Routes         | `name.routes.ts`      | `app.routes.ts`                         |

### 7.2 Rust Files

| Type     | Pattern            | Example                |
| -------- | ------------------ | ---------------------- |
| Routes   | `name.route.rs`    | `item.route.rs`        |
| Services | `name.service.rs`  | `item.service.rs`      |
| Models   | `name.model.rs`    | `item.model.rs`        |
| Errors   | `mod.rs` (errors/) | `errors/mod.rs`        |
| Helpers  | `name.helper.rs`   | `validation.helper.rs` |
| Entities | `name.entity.rs`   | `item.entity.rs`       |
| Logger   | `logger.rs`        | `logger.rs`            |
| State    | `state.rs`         | `state.rs`             |
| Lib      | `lib.rs`           | `lib.rs`               |
| Main     | `main.rs`          | `main.rs`              |

### 7.3 Commands

| Language   | Case         | Example            |
| ---------- | ------------ | ------------------ |
| Rust       | `snake_case` | `get_item_summary` |
| TypeScript | `camelCase`  | `getItemSummary`   |

---

## 8. Code Quality Rules

### 8.1 Type Safety (MANDATORY)

```typescript
// ❌ FORBIDDEN - any type
function badFunction(value: any) { ... }

// ✅ CORRECT - explicit types
function goodFunction(value: Item): string { ... }

// ✅ CORRECT - unknown with type guard
function handleUnknown(value: unknown): Item {
  if (isItem(value)) {
    return value;
  }
  throw new Error('Invalid item');
}
```

### 8.2 Immutability

```typescript
// ✅ CORRECT - Using signal update
this.selectedIds.update((set) => {
  const next = new Set(set);
  next.add(id);
  return next;
});

// ❌ WRONG - Direct mutation
this.selectedIds().add(id); // Mutating signal value - FORBIDDEN
```

### 8.3 Async/Await

```typescript
// ✅ CORRECT - async/await with error handling
async loadData(): Promise<void> {
  try {
    const data = await this.api.invoke<Item[]>('get_items');
    this.store.items.set(data);
  } catch (error) {
    this.errorHandler.handleError(error, 'loadData');
  }
}

// ❌ WRONG - .then().catch() chain
loadData(): void {
  this.api.invoke<Item[]>('get_items')
    .then(data => this.store.items.set(data))
    .catch(error => console.error(error)); // FORBIDDEN
}
```

### 8.4 No Magic Numbers

```typescript
// ❌ WRONG - Magic number
setTimeout(() => doSomething(), 86400000);

// ✅ CORRECT - Named constant
const TWENTY_FOUR_HOURS_MS = 24 * 60 * 60 * 1000;
setTimeout(() => doSomething(), TWENTY_FOUR_HOURS_MS);
```

---

## 9. Anti-Patterns (NEVER DO)

### 9.1 Code Smells

| Anti-Pattern                | Problem                     | Solution                             |
| --------------------------- | --------------------------- | ------------------------------------ |
| `console.error()`           | Scattered logging           | Use `logging.service.ts`             |
| `any` type                  | No type safety              | Proper types or `unknown`            |
| Magic numbers               | Unreadable                  | Named constants                      |
| Duplicate code              | Maintenance burden          | Extract to shared utils              |
| Dead code                   | Confusion                   | Remove immediately                   |
| Business logic in component | Testability                 | Move to `name.service.ts`            |
| Monolithic AppState         | Coupling                    | Split by domain                      |
| Direct `invoke()` calls     | Inconsistent error handling | Use `tauri-api.service.ts`           |
| Component CSS files         | Scattered styles            | ONE global `styles.css`              |
| Inline styles               | Violates separation         | TailwindCSS in `name.component.html` |
| Inline templates            | Mixed concerns              | Use `templateUrl: '.component.html'` |
| Non-standard naming         | Inconsistency               | Use `name.type.ts/html/rs` pattern   |

### 9.2 Structural Anti-Patterns

```rust
// ❌ FORBIDDEN - Monolithic state
pub struct AppState {
    pub service_a: Arc<ServiceA>,
    pub service_b: Arc<ServiceB>,
    // ... 20 more crammed together
}

// ❌ FORBIDDEN - Business logic in command
#[tauri::command]
pub fn bad_command() -> Result<Value, String> {
    let data = complex_processing()?; // Should be in service
    Ok(data)
}

// ❌ FORBIDDEN - SKIP_FRONTEND environment hack
if std::env::var("SKIP_FRONTEND").is_ok() {
    return; // FORBIDDEN
}
```

### 9.3 Import Anti-Patterns

```typescript
// ❌ FORBIDDEN - Unused imports
import { Something } from "./types"; // Never used

// ❌ FORBIDDEN - Non-existent imports
import { Wrong } from "./wrong-path"; // File doesn't exist

// ❌ FORBIDDEN - Barrel imports when not needed
import { A, B, C } from "./index"; // Import only what you need
```

---

## 10. Pre-Commit Checklist

Before committing code, verify:

- [ ] **No `console.error` calls** - Use `logging.service.ts` instead
- [ ] **No `any` types** - Use proper types or `unknown`
- [ ] **No unused imports** - Clean up before commit
- [ ] **All errors handled** - Via `error-handler.service.ts`
- [ ] **Components are dumb** - No business logic in `name.component.ts`
- [ ] **Services handle logic** - API calls in `name.service.ts`
- [ ] **No magic numbers** - Use named constants
- [ ] **Types properly defined** - Interfaces in `name.model.ts`
- [ ] **Names follow conventions** - Use `name.type.ts/html/rs` pattern
- [ ] **No dead code** - Remove unused functions/services
- [ ] **TailwindCSS in .html only** - No styling in `name.component.ts`
- [ ] **Signals for state** - No BehaviorSubject, use `name.store.ts`
- [ ] **DRY principle** - No duplicated code
- [ ] **ONE global CSS file** - No component CSS files
- [ ] **Pure TailwindCSS** - No inline styles, no custom component CSS
- [ ] **Logging configured** - `logger.rs` and `logging.service.ts` with toggle support
- [ ] **Environment-based log levels** - `LOG_LEVEL` env var, settings service

---

## 12. TailwindCSS v4 Setup for Angular v20-21

### 12.1 Package Dependencies

```bash
npm install tailwindcss @tailwindcss/postcss postcss autoprefixer
```

### 12.2 Project Configuration

**1. `postcss.config.js` at project root:**

```javascript
export default {
  plugins: {
    "@tailwindcss/postcss": {},
    autoprefixer: {},
  },
};
```

**2. `tailwind.config.js` at project root:**

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,ts}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        accent: {
          DEFAULT: "var(--color-accent)",
          hover: "var(--color-accent-hover)",
        },
      },
    },
  },
  plugins: [],
};
```

**3. Update `angular.json` - add global styles:**

```json
{
  "projects": {
    "your-app": {
      "architect": {
        "build": {
          "options": {
            "styles": ["src/styles.css"],
            "scripts": []
          }
        }
      }
    }
  }
}
```

**4. `src/styles.css` - ONE global CSS file:**

```css
@import "tailwindcss";

@theme {
  --color-accent: #10b981;
  --color-accent-hover: #059669;
  --color-card: #1a1a2e;
  --color-border: #2d2d44;
  --color-text-primary: #f1f5f9;
  --color-text-secondary: #94a3b8;
  --color-success: #22c55e;
  --color-warning: #f97316;
  --color-error: #ef4444;
  --color-info: #3b82f6;
}

/* Dark mode */
.dark {
  --bg-primary: #0a0a0f;
  --bg-secondary: #12121a;
  --bg-card: #1a1a2e;
}

.light {
  --bg-primary: #f8fafc;
  --bg-secondary: #f1f5f9;
  --bg-card: #ffffff;
}

/* Base styles */
body {
  background: var(--bg-primary);
  color: var(--text-primary);
}
```

### 12.3 Component Usage

**`name.component.ts`:**

```typescript
@Component({
  selector: "app-button",
  templateUrl: "./button.component.html",
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ButtonComponent {
  @Input() variant = input<"primary" | "secondary">("primary");
  @Output() clicked = output<void>();
}
```

**`name.component.html`:**

```html
<button
  class="px-4 py-2 rounded-lg font-medium transition-colors"
  [class.bg-accent]="variant() === 'primary'"
  [class.bg-secondary]="variant() === 'secondary'"
  (click)="clicked.emit()"
>
  <ng-content></ng-content>
</button>
```

### 12.4 Key Rules

| Rule                    | Implementation                                       |
| ----------------------- | ---------------------------------------------------- |
| ONE global CSS          | `src/styles.css` only                                |
| TailwindCSS via @import | `@import "tailwindcss";`                             |
| Theme via @theme        | CSS custom properties in `:root` or `.dark`/`.light` |
| No component CSS files  | Components use TailwindCSS in HTML                   |
| Dark mode via class     | `darkMode: 'class'` in tailwind.config.js            |

---

## 13. Logging System

### 13.1 Rust Logger (Backend)

**Principle: Centralized logging with configurable log levels.**

**`src-tauri/src/logger.rs`:**

```rust
use log::{LevelFilter, Logger};
use env_logger::Builder;
use std::io::Write;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Logger {
    level: LevelFilter,
    inner: Arc<Mutex<Builder>>,
}

impl Logger {
    pub fn new() -> Self {
        let level = Self::parse_log_level();
        let mut builder = Builder::new();
        builder
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{}] {} - {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                )
            })
            .filter(None, level);

        Self {
            level,
            inner: Arc::new(Mutex::new(builder)),
        }
    }

    fn parse_log_level() -> LevelFilter {
        std::env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string())
            .to_lowercase()
            .as_str()
            .into()
    }

    pub fn init(&self) {
        let builder = self.inner.lock().unwrap();
        builder.init();
    }

    pub fn debug(&self, msg: &str) {
        log::debug!("{}", msg);
    }

    pub fn info(&self, msg: &str) {
        log::info!("{}", msg);
    }

    pub fn warn(&self, msg: &str) {
        log::warn!("{}", msg);
    }

    pub fn error(&self, msg: &str) {
        log::error!("{}", msg);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
```

**Usage in Rust:**

```rust
#[tauri::command]
pub fn get_data(state: State<AppState>) -> Result<ResponseModel, ResponseModel> {
    state.logger.info("Fetching data");
    // ... logic
    state.logger.debug("Data fetched successfully");
}
```

**Configuration via environment:**

```bash
# .env or environment variables
LOG_LEVEL=debug   # trace, debug, info, warn, error
```

### 13.2 logging.service.ts (Angular Frontend)

**Principle: Centralized logging with configurable levels and output.**

```typescript
// ✅ CORRECT - shared/services/logging.service.ts
export type LogLevel = "debug" | "info" | "warn" | "error";

export interface LogEntry {
  level: LogLevel;
  message: string;
  context?: string;
  timestamp: Date;
  data?: unknown;
}

@Injectable({ providedIn: "root" })
export class LoggingService {
  private config = inject(SettingsService);

  readonly logs = signal<LogEntry[]>([]);
  readonly isEnabled = computed(() => this.config.currentSettings.logging.enabled);
  readonly minLevel = computed(() => this.config.currentSettings.logging.minLevel);

  private shouldLog(level: LogLevel): boolean {
    if (!this.isEnabled()) return false;
    const levels: LogLevel[] = ["debug", "info", "warn", "error"];
    const currentIndex = levels.indexOf(this.minLevel());
    const messageIndex = levels.indexOf(level);
    return messageIndex >= currentIndex;
  }

  debug(message: string, context?: string, data?: unknown): void {
    if (!this.shouldLog("debug")) return;
    this.log("debug", message, context, data);
  }

  info(message: string, context?: string, data?: unknown): void {
    if (!this.shouldLog("info")) return;
    this.log("info", message, context, data);
  }

  warn(message: string, context?: string, data?: unknown): void {
    if (!this.shouldLog("warn")) return;
    this.log("warn", message, context, data);
  }

  error(message: string, context?: string, data?: unknown): void {
    if (!this.shouldLog("error")) return;
    this.log("error", message, context, data);
  }

  private log(level: LogLevel, message: string, context?: string, data?: unknown): void {
    const entry: LogEntry = {
      level,
      message,
      context,
      timestamp: new Date(),
      data,
    };

    // Store in memory
    this.logs.update((logs) => [entry, ...logs].slice(0, 1000));

    // Console output
    const prefix = context ? `[${context}]` : "";
    switch (level) {
      case "debug":
        console.debug(`[DEBUG]${prefix} ${message}`, data ?? "");
        break;
      case "info":
        console.info(`[INFO]${prefix} ${message}`, data ?? "");
        break;
      case "warn":
        console.warn(`[WARN]${prefix} ${message}`, data ?? "");
        break;
      case "error":
        console.error(`[ERROR]${prefix} ${message}`, data ?? "");
        break;
    }
  }

  clearLogs(): void {
    this.logs.set([]);
  }
}
```

### 13.3 settings.model.ts - Configuration Model

```typescript
// ✅ CORRECT - shared/models/settings.model.ts
export interface LoggingSettings {
  enabled: boolean;
  minLevel: LogLevel;
  consoleOutput: boolean;
  remoteLogging: boolean;
}

export interface AppSettings {
  logging: LoggingSettings;
  // ... other settings
}

export const DEFAULT_LOGGING_SETTINGS: LoggingSettings = {
  enabled: true,
  minLevel: "info", // debug, info, warn, error
  consoleOutput: true,
  remoteLogging: false,
};
```

### 13.4 Configuration Files

**`src/config/settings.json` (or via Tauri config):**

```json
{
  "logging": {
    "enabled": true,
    "minLevel": "info",
    "consoleOutput": true,
    "remoteLogging": false
  }
}
```

**Rust `.env`:**

```bash
LOG_LEVEL=info
RUST_LOG=info
```

### 13.5 logger.rs (Rust Backend)

```rust
// ✅ CORRECT - logger.rs
use log::{LevelFilter, Logger};
use env_logger::Builder;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Logger {
    level: LevelFilter,
}

impl Logger {
    pub fn new() -> Self {
        let level = std::env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string())
            .to_lowercase()
            .as_str()
            .into();
        Self { level }
    }

    pub fn init(&self) {
        Builder::new()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{}] {} - {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                )
            })
            .filter(None, self.level)
            .init();
    }

    pub fn debug(&self, msg: &str) { log::debug!("{}", msg); }
    pub fn info(&self, msg: &str) { log::info!("{}", msg); }
    pub fn warn(&self, msg: &str) { log::warn!("{}", msg); }
    pub fn error(&self, msg: &str) { log::error!("{}", msg); }
}

impl Default for Logger {
    fn default() -> Self { Self::new() }
}
```

### 13.6 Usage Examples

**Rust - Application startup (lib.rs):**

```rust
fn run() {
    let logger = Logger::new();
    logger.init();
    log::info!("Application starting...");

    tauri::Builder::default()
        .setup(|app| {
            log::info!("Tauri app setup complete");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Angular - Service logging:**

```typescript
// ✅ CORRECT - services/dashboard.service.ts
@Injectable({ providedIn: "root" })
export class DashboardService {
  private logger = inject(LoggingService);
  private api = inject(TauriApiService);

  async loadDashboard(): Promise<void> {
    this.logger.debug("DashboardService.loadDashboard", "Loading dashboard data");

    try {
      const data = await this.api.invoke<DashboardData>("get_dashboard_data");
      this.logger.info("DashboardService.loadDashboard", "Dashboard loaded successfully");
    } catch (error) {
      this.logger.error("DashboardService.loadDashboard", "Failed to load dashboard", { error });
      throw error;
    }
  }
}
```

### 13.7 Toggle Logging Summary

| Layer       | Configuration       | Toggle Method                          |
| ----------- | ------------------- | -------------------------------------- |
| **Rust**    | `LOG_LEVEL` env var | `RUST_LOG=debug` or `LOG_LEVEL=info`   |
| **Rust**    | Code at runtime     | `Logger::new()` reads env on init      |
| **Angular** | Settings service    | `settings.logging.enabled`             |
| **Angular** | Min level           | `settings.logging.minLevel`            |
| **Both**    | Build config        | Different `.env` files per environment |

---

## 14. Abstract Principles Summary

### Core Tenets

1. **Single Responsibility** - Each piece does one thing well
2. **Dependency Injection** - Services receive dependencies, don't create them
3. **Error Propagation** - Errors flow up, never silently caught
4. **Type Safety** - No `any`, no guessing
5. **DRY** - Extract shared logic, no duplication
6. **Pure UI** - Components render, services act

### Architectural Rules

| Layer          | Do                                                 | Never Do                                  |
| -------------- | -------------------------------------------------- | ----------------------------------------- |
| **Components** | Render UI, emit events, use `@Input()`/`@Output()` | API calls, business logic, state mutation |
| **Services**   | API calls, business logic, data transformation     | UI code, direct DOM manipulation          |
| **Stores**     | Signal state, `computed()` derived state           | API calls, business logic                 |
| **Models**     | Define data shapes                                 | Logic                                     |
| **Commands**   | Thin wrappers calling services                     | Business logic                            |

### Styling Rules

| Do                             | Never Do                                          |
| ------------------------------ | ------------------------------------------------- |
| ONE global styles.css file     | Component-specific CSS files                      |
| TailwindCSS classes in .html   | TailwindCSS classes in .ts                        |
| CSS variables for theme tokens | Hardcoded color values                            |
| Semantic color names           | Arbitrary values like `bg-[#123]`                 |
| `templateUrl: './file.html'`   | `template: '<div>...</div>'` or `styles: ['...']` |

### Backend Rules

| Do                                    | Never Do                     |
| ------------------------------------- | ---------------------------- |
| Focused AppState by domain            | Monolithic AppState          |
| Thin commands delegating to services  | Commands with business logic |
| Consistent ResponseModel<T>           | Raw values as response       |
| Error propagation via `?`             | `unwrap()` or `expect()`     |
| `Arc<TokioMutex<T>>` for shared state | Global mutable state         |

### Result of Following These Rules

- **No redundant code** - DRY principle enforced
- **No duplicated logic** - Shared utilities extracted (e.g., `invoke-wrapper.util.ts`)
- **No dead code** - Unused code removed
- **No type safety issues** - `any` forbidden
- **No error swallowing** - All errors handled via `error-handler.service.ts`
- **Correct architecture** - Proper separation of concerns
- **Maintainable codebase** - Predictable structure with `name.type.ts` naming
- **Testable code** - Dependencies injected, logic in services
- **Scalable** - Domain-driven state organization
- **Consistent naming** - All files follow `name.type.ts/html/rs` pattern
