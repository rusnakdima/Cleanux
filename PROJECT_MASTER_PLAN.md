# PROJECT MASTER PLAN: Cleanux

**Project:** Cleanux
**Document Version:** 2.0
**Date:** 2026-06-14
**Status:** ⚠️ CRITICAL - Duplicate LoggerService

---

## Part 1: Project Overview

### Purpose

System optimization and cleaning utility with junk removal, backup, automation.

### Tech Stack

- **Frontend:** Angular v19
- **Backend:** Rust (Tauri v2)

### Build Status

- TypeScript: ✅ PASS
- Rust: ✅ PASS
- File Logging: ⚠️ Has own log manager

---

## Part 2: CRITICAL ISSUE: Duplicate LoggerService

### Problem

Cleanux has TWO logging services:

1. **`src/app/services/logger.service.ts`** (315 lines) - **DUPLICATE**
2. **`src/app/shared/services/logging.service.ts`** (600+ lines) - **UNIFIED**

The project is using the DUPLICATE, not the unified one!

### Evidence

```typescript
// src/app/services/logger.service.ts - WRONG TO USE
export class LoggerService {
  private config = environment.logging; // Uses custom format
  // Different API than unified LoggingService
}
```

### Required Actions

1. [ ] DELETE `src/app/services/logger.service.ts`
2. [ ] Update `api/tauri-api.service.ts` to import from `_shared/logging.service.ts`
3. [ ] Update ALL imports from `LoggerService` to `LoggingService`
4. [ ] Verify no `console.*` calls remain

### Verification

```bash
# Should find 0 results (only shared/services/logging.service.ts allowed)
grep -rn "class LoggerService" src/app --include="*.ts" | grep -v "shared/services"

# Should find only logging.service.ts
ls src/app/shared/services/
```

---

## Part 3: File Logging

### Current Status

Cleanux has its own log manager system:

- `services/log-manager.service.ts`
- `services/log-storage.service.ts`

### Recommendation

Keep existing log manager OR integrate with unified logging.

---

## Part 4: Implementation Tasks

### Phase 1: CRITICAL - Remove Duplicate

- [ ] DELETE `src/app/services/logger.service.ts`
- [ ] Update all imports
- [ ] Verify no console.\* calls

### Phase 2: File Logging

- [ ] Evaluate existing log-manager vs unified logging
- [ ] Decision: Keep or migrate

---

## Part 5: Key Files

### Files to DELETE

| File                                 | Reason    |
| ------------------------------------ | --------- |
| `src/app/services/logger.service.ts` | DUPLICATE |

### Files to MODIFY

| File                             | Action                     | Priority |
| -------------------------------- | -------------------------- | -------- |
| `api/tauri-api.service.ts`       | Use unified LoggingService | 🔴 HIGH  |
| Any file importing LoggerService | Update imports             | 🔴 HIGH  |

---

## Part 6: Verification

```bash
# Check for duplicate LoggerService
grep -rn "class LoggerService" src/app --include="*.ts" | grep -v "shared/services"
# Should return 0 results

# Check for console.*
grep -rn "console\." src/app --include="*.ts" | grep -v "logging.service.ts"
```

---

**Document Status:** v2.0 - CRITICAL: Duplicate LoggerService
**Last Updated:** 2026-06-14
