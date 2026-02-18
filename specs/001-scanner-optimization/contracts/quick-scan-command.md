# Contract: `quick_scan_command`

**Feature**: `001-scanner-optimization`  
**File**: `src-tauri/src/commands/settings_commands.rs` (current location) or new `scan_commands.rs`  
**Status**: Existing command — **return type changes**

---

## Summary of Change

The existing `quick_scan_command` currently returns `Result<(), String>`. This feature changes the return type to `Result<ScanReport, String>` to surface scan completion telemetry to the frontend.

This is the **only command signature change** in this feature. No new commands are added (per spec Out of Scope).

---

## Before (current)

```rust
#[tauri::command]
pub async fn quick_scan_command() -> Result<(), String>
```

**Angular call site** (current — returns `void`):
```typescript
await invoke<void>('quick_scan_command');
```

---

## After (this feature)

```rust
#[tauri::command]
pub async fn quick_scan_command() -> Result<ScanReport, String>
```

**Angular call site** (updated — returns `ScanReport`):
```typescript
const report = await invoke<ScanReport>('quick_scan_command');
console.log(`Added: ${report.books_added}, Skipped: ${report.books_skipped}, Orphaned: ${report.books_newly_orphaned}, Errors: ${report.errors}, Time: ${report.elapsed_ms}ms`);
```

---

## `ScanReport` — full schema

See [`scan-report.schema.json`](./scan-report.schema.json) for the JSON Schema.

| Field | Type | Description |
|-------|------|-------------|
| `books_added` | `number` | New records inserted |
| `books_skipped` | `number` | Directories skipped (already in DB) |
| `books_newly_orphaned` | `number` | Records newly marked orphaned |
| `errors` | `number` | Files/dirs that failed |
| `elapsed_ms` | `number` | Wall-clock time in ms |

---

## Breaking change assessment

**Severity: Non-breaking for existing UI** (the settings component that calls `quick_scan_command` currently ignores the return value — changing from `void` to `ScanReport` does not break any TypeScript call site that was using `await invoke<void>(...)`). However, the TypeScript call site should be updated to `invoke<ScanReport>` for type safety.

---

## `Book` model changes

See [`book.schema.json`](./book.schema.json) for the full updated `Book` JSON schema. The four new nullable fields (`duration_seconds`, `duration_is_estimated`, `file_count`, `orphaned`) are additive — no existing field is renamed, removed, or changes type. Existing Angular components that destructure `Book` objects continue to work without modification; they simply receive `null` for the new fields on pre-migration rows.

---

## Tauri capabilities

No changes to `src-tauri/capabilities/migrated.json` are required. The scanner reads the local filesystem (already permitted) and writes to the local SQLite database (already permitted).
