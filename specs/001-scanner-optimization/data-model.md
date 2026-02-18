# Data Model: Scanner Optimization & Enhancement

**Feature**: `001-scanner-optimization`  
**Date**: 2026-02-18  
**Spec ref**: `../spec.md` §Requirements → FR-007, FR-008

---

## Overview

This feature extends the existing `Book` model with four new nullable fields and introduces two ephemeral in-memory types (`DirectoryMetadata`, `ScanResult`) that are internal to the scanner. A new `ScanReport` type carries scan completion telemetry to the command layer.

All four new DB columns are added via a Diesel migration. All four default to `NULL`, preserving existing rows. The `schema.rs` file is regenerated automatically by Diesel after migration.

---

## 1. `Book` — Extended (Persistent)

**Location**: `src-tauri/src/models/book.rs`  
**Table**: `books`  
**Storage**: SQLite (via Diesel migration)

### Current fields (unchanged)

| Field | Rust type | SQL type | Notes |
|-------|-----------|----------|-------|
| `title` | `String` | `TEXT NOT NULL` | Primary key |
| `relative_cover_path` | `Option<String>` | `TEXT` | Relative to active root |
| `author_name` | `String` | `TEXT NOT NULL` | FK → `authors.name` |
| `genre` | `Option<String>` | `TEXT` | Free-text |
| `lector` | `Option<String>` | `TEXT` | Free-text |
| `create_date` | `Option<NaiveDateTime>` | `TIMESTAMP` | From file metadata |
| `read` | `Option<bool>` | `BOOLEAN` | User-set flag |
| `score` | `Option<i32>` | `INTEGER` | 1–10 user rating |
| `relative_file_path` | `String` | `TEXT NOT NULL` | Relative to active root |

### New fields (this feature)

| Field | Rust type | SQL type | Default | Write rule |
|-------|-----------|----------|---------|-----------|
| `duration_seconds` | `Option<i32>` | `INTEGER NULL` | `NULL` | Set on first scan; never overwritten by re-scan |
| `duration_is_estimated` | `Option<bool>` | `BOOLEAN NULL` | `NULL` | `false` = all TLEN tags present; `true` = estimation used |
| `file_count` | `Option<i32>` | `INTEGER NULL` | `NULL` | Count of `.mp3` files in directory; set on first scan |
| `orphaned` | `Option<bool>` | `BOOLEAN NULL` | `NULL` | Set each scan cycle: `true` = path missing; `false` = path confirmed; `NULL` = never checked |

### Validation rules

- `duration_seconds = 0` is valid only when all files are unreadable or zero-length (in which case `duration_is_estimated` must be `true`).
- `duration_is_estimated = false` requires that every file in the directory returned a valid non-zero TLEN.
- `file_count >= 1` for all newly scanned books; `NULL` only for rows created before this feature.
- `orphaned` may only be set by the scanner's orphan-check phase. User-editable commands (`update_book_command`) MUST NOT touch this field.

### State transitions for `orphaned`

```
NULL (pre-feature row)
  │
  └─► false  (path exists on first scan after migration)
        │
        ├─► true   (path deleted before next scan)
        │     │
        │     └─► false  (path restored)
        │
        └─► true   ...
```

### Migration SQL

File: `src-tauri/migrations/<timestamp>_scanner_fields/up.sql`

```sql
ALTER TABLE books ADD COLUMN duration_seconds   INTEGER  DEFAULT NULL;
ALTER TABLE books ADD COLUMN duration_is_estimated BOOLEAN DEFAULT NULL;
ALTER TABLE books ADD COLUMN file_count         INTEGER  DEFAULT NULL;
ALTER TABLE books ADD COLUMN orphaned           BOOLEAN  DEFAULT NULL;
```

File: `src-tauri/migrations/<timestamp>_scanner_fields/down.sql`

```sql
-- SQLite does not support DROP COLUMN in older versions.
-- If using SQLite >= 3.35, these work directly.
ALTER TABLE books DROP COLUMN duration_seconds;
ALTER TABLE books DROP COLUMN duration_is_estimated;
ALTER TABLE books DROP COLUMN file_count;
ALTER TABLE books DROP COLUMN orphaned;
```

---

## 2. `DirectoryMetadata` — In-Memory (Parallel Phase Output)

**Location**: `src-tauri/src/scanner.rs` (private)  
**Lifetime**: Ephemeral — lives only during one `quick_scan()` invocation; never serialized or stored.

Produced by the parallel phase (one instance per audiobook directory). Passed to the sequential DB-write phase.

| Field | Rust type | Description |
|-------|-----------|-------------|
| `parent_path` | `String` | Absolute path of the book directory |
| `mp3_paths` | `Vec<PathBuf>` | All `.mp3` files found in the directory |
| `first_tag` | `Option<id3::Tag>` | ID3 tag from the first MP3 in the directory; `None` if no readable tag found |
| `file_create_date` | `Option<NaiveDateTime>` | Creation timestamp from the first MP3's filesystem metadata |
| `duration_seconds` | `Option<i32>` | Computed total duration (TLEN sum or file-size estimate) |
| `duration_is_estimated` | `bool` | `true` if any file used size-based estimation |
| `file_count` | i32 | Number of `.mp3` files found |

**Derivation notes**:
- `duration_seconds` is computed entirely in the parallel phase from `mp3_paths`.
- `file_count = mp3_paths.len() as i32`.
- `first_tag` is used in the sequential phase to extract `TALB`, `TPE2`, `TCON`, `TPE1`.

---

## 3. `ScanError` — In-Memory (Error Transport)

**Location**: `src-tauri/src/scanner.rs` (private)  
**Lifetime**: Ephemeral — lives only during one scan invocation; logged, not persisted.

| Field | Rust type | Description |
|-------|-----------|-------------|
| `path` | `String` | File or directory path that caused the error |
| `message` | `String` | Human-readable error description |

Used as the `Err` variant in `Vec<Result<DirectoryMetadata, ScanError>>` from the parallel phase.

---

## 4. `ScanReport` — Scan Completion Summary (Serialized to Frontend)

**Location**: `src-tauri/src/scanner.rs` (public, returned from `quick_scan()`)  
**Serialization**: `serde::Serialize` — returned via `quick_scan_command` as JSON to Angular.

| Field | Rust type | JSON key | Description |
|-------|-----------|----------|-------------|
| `books_added` | `usize` | `books_added` | New book records inserted this run |
| `books_skipped` | `usize` | `books_skipped` | Directories skipped (title already in DB) |
| `books_newly_orphaned` | `usize` | `books_newly_orphaned` | Records newly set to `orphaned = true` |
| `errors` | `usize` | `errors` | Files/directories that failed processing |
| `elapsed_ms` | `u128` | `elapsed_ms` | Wall-clock time for the full scan |

**Angular interface** (`src/app/models/books.ts` or a new `src/app/models/scan.ts`):
```typescript
interface ScanReport {
  books_added: number;
  books_skipped: number;
  books_newly_orphaned: number;
  errors: number;
  elapsed_ms: number;
}
```

---

## 5. TypeScript `Book` Interface — Delta

**Location**: `src/app/models/books.ts`

New fields to add (Principle IV — same PR as Rust model changes):

```typescript
duration_seconds: number | null;
duration_is_estimated: boolean | null;
file_count: number | null;
orphaned: boolean | null;
relative_file_path: string;  // already exists in Rust model but missing from TS interface — fix in same PR
```

---

## Entity Relationship Summary

```
books (existing + 4 new columns)
  │
  ├── duration_seconds         INTEGER NULL
  ├── duration_is_estimated    BOOLEAN NULL
  ├── file_count               INTEGER NULL
  └── orphaned                 BOOLEAN NULL

DirectoryMetadata  ──(parallel phase)──►  Book insert
ScanError          ──(parallel phase)──►  logged + counted
ScanReport         ◄──(quick_scan return)──  command layer
```

---

## Constraints & Invariants

1. **FR-001 invariant**: The scanner NEVER calls `update_book` on an existing record during the scan phase. The only write to an existing `Book` row is `orphaned` field via the dedicated orphan-check phase.
2. **Migration invariant**: All four columns default to `NULL`. No backfill is run. Existing rows remain valid.
3. **Type sync invariant (Principle IV)**: The Rust `Book` struct, Diesel schema, and TypeScript `Book` interface must all be updated in the same commit/PR.
4. **`file_count` floor**: Must be `>= 1` for any newly inserted book. A directory with zero MP3 files is silently skipped (no record created).
