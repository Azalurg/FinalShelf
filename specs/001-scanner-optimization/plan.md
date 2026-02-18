# Implementation Plan: Scanner Optimization & Enhancement

**Branch**: `001-scanner-optimization` | **Date**: 2026-02-18 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/001-scanner-optimization/spec.md`

---

## Summary

Refactor `src-tauri/src/scanner.rs` to replace serial `WalkDir` processing with a two-phase architecture: a parallel rayon metadata-extraction phase followed by a sequential Diesel write phase. Add four new nullable columns to the `books` table (`duration_seconds`, `duration_is_estimated`, `file_count`, `orphaned`) via a Diesel migration, update the Rust `Book` struct and TypeScript `Book` interface accordingly, and implement an orphan-detection phase that marks missing books without deleting data. Replace all `println!` calls with structured `log::warn!` / `log::info!` logging. Return a `ScanReport` from `quick_scan_command` instead of `()`.

---

## Technical Context

**Language/Version**: Rust 2021 edition (Tauri 2.0); TypeScript ~5.2.2 (Angular 17)
**Primary Dependencies**:
- `rayon` (new) — parallel metadata extraction; Principle VII satisfied (no stdlib alternative at equivalent ergonomics)
- `log` (new) — structured logging façade; replaces all `println!` calls
- `id3 1.16.x` (existing) — `tag.duration()` returns `Option<u32>` ms for TLEN
- `diesel 2.2.x` (existing) — `diesel::update(...).set(dsl::orphaned.eq(Some(flag)))` for column-level updates; `eq_any` for bulk orphan updates
- `walkdir 2.5.x` (existing) — directory traversal (unchanged API)
- `tokio::task::spawn_blocking` (existing, from Tauri) — bridges the Tokio async context to the rayon blocking context

**Storage**: SQLite (via Diesel); one Diesel migration adds 4 nullable columns
**Testing**: `cargo test` (unit tests in `scanner.rs` under `#[cfg(test)]`); `cargo tarpaulin` for coverage; `tests/bench_scanner.sh` for performance
**Target Platform**: Linux (primary); cross-platform (macOS/Windows) via Tauri — no platform-specific code added
**Project Type**: Single Tauri desktop app (Rust backend + Angular frontend)
**Performance Goals**:
- No-op scan (all books already in DB): ≤ 2 s regardless of library size (SC-001)
- Full scan of 10,000 files (1,000 books × 10 MP3s): ≤ 30 s on ≥4-core SSD machine (SC-002)
- ≥ 2× faster than pre-refactor binary on the same benchmark fixture (SC-002)

**Constraints**:
- rayon closures MUST NOT call `async/.await` or Tokio primitives (thread boundary rule)
- Diesel connection objects MUST be created in the sequential phase, not inside rayon closures
- User-editable fields (`score`, `read`) MUST NEVER be overwritten by the scanner (FR-001)
- `unwrap()` / `panic!()` forbidden in `scanner.rs` outside `#[cfg(test)]` (FR-005)
- `SQLITE_MAX_VARIABLE_NUMBER = 999`: `eq_any` bulk updates must be chunked for libraries > 999 books

**Scale/Scope**: Libraries of 100–10,000 books; benchmark fixture is 1,000 books × 10 files

---

## Constitution Check

*GATE: Pre-design (now) and post-design (re-checked below after Phase 1).*

| Principle | Requirement | Status | Notes |
|-----------|-------------|--------|-------|
| **I. Local-First** | No external network calls; all data local | PASS | Scanner reads local filesystem and writes local SQLite only |
| **II. Layered Architecture** | Commands are thin; business logic in services | WATCH | `quick_scan()` lives in `scanner.rs` which is called by `settings_commands.rs`. The scanner is a service-layer concern. Ensure the command handler only calls `quick_scan()` and returns the result — no logic leaks into the command layer. |
| **III. Performance by Default** | Performance gates as acceptance criteria | PASS | SC-001, SC-002 are explicit, measurable, and verified by the benchmark script |
| **IV. Type Safety & Shared Contracts** | TS interface sync in same PR | PASS | `src/app/models/books.ts` must be updated in the same commit. `ScanReport` TS interface required. `tsc --noEmit` and `cargo check` must pass. |
| **V. Incremental, Testable Delivery** | No panics; unit tests; `#[cfg(test)]` in same file | PASS | FR-005 bans `unwrap`; SC-006 requires ≥80% unit coverage; each US is independently testable |
| **VI. Portability via Relative Paths** | Never persist absolute paths | PASS | Scanner continues to call `strip_prefix(base_path)` before storing paths; no change to this behavior |
| **VII. Simplicity & Sustainable Maintainability** | YAGNI; no speculative abstractions | PASS (justified) | `rayon` is the only new non-trivial dependency. Justification documented in research.md R1. `log` is a standard façade with zero maintenance burden. |

**Post-Design Re-check** (after data-model.md and contracts/ generated):

| Principle | Post-Design Status | Additional Notes |
|-----------|-------------------|-----------------|
| **II** | PASS | `DirectoryMetadata` and `ScanError` are private to `scanner.rs`. `ScanReport` is returned up to the command layer. No DB access in the command. |
| **IV** | PASS | `data-model.md` confirms the Rust `Book` struct, Diesel schema, and TypeScript interface all gain the same 4 fields in the same migration+PR. `ScanReport` TS interface documented. |
| **V** | PASS | The two-phase architecture (parallel collect then serial write) makes each phase unit-testable independently with in-memory fixtures. |

**Gate result: PASS — no blocking violations. Implementation may proceed.**

---

## Project Structure

### Documentation (this feature)

```text
specs/001-scanner-optimization/
├── plan.md              <- this file
├── research.md          <- Phase 0 (complete)
├── data-model.md        <- Phase 1 (complete)
├── quickstart.md        <- Phase 1 (complete)
├── contracts/
│   ├── book.schema.json           <- updated Book JSON schema with 4 new fields
│   ├── scan-report.schema.json    <- new ScanReport JSON schema
│   └── quick-scan-command.md      <- command contract (return type change)
├── checklists/
│   └── requirements.md    <- all items pass
└── tasks.md              <- Phase 2 (/speckit.tasks — NOT created here)
```

### Source Code (files touched by this feature)

```text
src-tauri/
├── Cargo.toml                              MODIFY  add rayon, log dependencies
├── src/
│   ├── scanner.rs                          REWRITE full refactor (largest change)
│   ├── models/
│   │   └── book.rs                         MODIFY  add 4 new Option<> fields
│   ├── services/
│   │   └── books_service.rs                MODIFY  add orphan-update helpers
│   └── commands/
│       └── settings_commands.rs            MODIFY  wrap quick_scan in spawn_blocking, change return type
└── migrations/
    └── <timestamp>_scanner_fields/
        ├── up.sql                          CREATE  4 ALTER TABLE ADD COLUMN statements
        └── down.sql                        CREATE  4 DROP COLUMN statements

src-tauri/src/schema.rs                     AUTO-GENERATED by diesel migration run

src/app/models/
└── books.ts                                MODIFY  add 4 new nullable fields + ScanReport interface

tests/
├── bench_scanner.sh                        CREATE  benchmark script
├── bench_baseline.txt                      CREATE  pre-refactor baseline time
└── fixtures/
    └── bench-library/                      CREATE  1000 dirs x 10 MP3s (generated)

src-tauri/src/bin/
└── gen_fixtures.rs                         CREATE  fixture generator binary (dev-only)
```

**Structure Decision**: Single Tauri project — no new modules, no new service files at the module level. The scanner is self-contained in `scanner.rs`. New helper functions in `books_service.rs` follow the existing layered pattern. The `gen_fixtures` binary is dev-only (excluded from production via Cargo features or separate `[[bin]]` with no auto-include).

---

## Complexity Tracking

No constitution violations requiring justification. Additions that needed evaluation:

| Addition | Why Needed | Simpler Alternative Rejected Because |
|----------|------------|-------------------------------------|
| `rayon` dependency | Data-parallel metadata extraction to meet 2x performance target | `std::thread` + manual scoped threads + channels = ~150 lines of infrastructure for equivalent behavior; rayon delivers this as a single `par_iter()` call |
| `tokio::task::spawn_blocking` wrapper | Safe Tokio/rayon thread boundary in existing async command | Calling `par_iter()` directly in an `async fn` blocks Tokio's async worker pool — IPC starvation risk |
| `gen_fixtures` binary | Reproducible benchmark fixture (FR-012 — cannot commit 10,000 audio files) | No alternative to code-generating them; committed output is ~2 MB of minimal ID3v2 files |

---

## Implementation Phases

### Phase A — Foundation (prerequisite for all other phases)

**Scope**: No behavioral change — pure setup. Validates the build still compiles after model changes.

1. **Diesel migration**: run `diesel migration generate scanner_fields`, write `up.sql` and `down.sql` (see `data-model.md` §Migration SQL), run `diesel migration run`, verify `schema.rs` regenerates with 4 new columns.
2. **`Book` struct update**: add `duration_seconds: Option<i32>`, `duration_is_estimated: Option<bool>`, `file_count: Option<i32>`, `orphaned: Option<bool>` to `src-tauri/src/models/book.rs`. Fix all downstream compilation errors (existing `Book { ... }` literals in `scanner.rs` need the 4 new fields initialized to `None`).
3. **TypeScript interface update**: add 4 nullable fields + `relative_file_path` (currently missing from TS interface) + `ScanReport` interface to `src/app/models/books.ts`. Run `tsc --noEmit`.
4. **`Cargo.toml`**: add `rayon` and `log` under `[dependencies]`.
5. **Gate**: `cargo check` passes, `tsc --noEmit` passes, `cargo test` passes (existing tests unbroken).

### Phase B — Parallel Architecture (US1 + US4 core)

**Scope**: Refactor `scanner.rs` to the two-phase architecture. No new fields populated yet (all `None` for duration/file_count/orphaned).

1. Define private types `DirectoryMetadata`, `ScanError`, and public `ScanReport` in `scanner.rs` (see `data-model.md`).
2. Rewrite `quick_scan()` signature to `pub fn quick_scan() -> Result<ScanReport, String>`.
3. Parallel phase: collect unique parent directories from `WalkDir`, then `par_iter()` over them calling a new `extract_directory_metadata(dir_path, base_path)` function that returns `Result<DirectoryMetadata, ScanError>`. Collect into `Vec<Result<...>>`.
4. Serial phase: iterate results — for each `Ok(meta)`: skip if `is_book_exists`, build `Book` (new fields all `None`), call `add_book`, tally `books_added` / `books_skipped`. For each `Err(e)`: log warning, tally `errors`.
5. Replace all `unwrap()` / `panic!()` with `?` / `match` / `if let` (FR-005). *(May be deferred to the Phase D / US4 pass for cleaner commits — both phases touch `scanner.rs`.)*
6. Replace all `println!` with `log::warn!` / `log::info!` (FR-006). *(Same deferral note as step 5.)*
7. Update `quick_scan_command` in `settings_commands.rs` to use `tokio::task::spawn_blocking(|| quick_scan()).await` and return `Result<ScanReport, String>`.
8. **Gate**: `cargo test` passes, `cargo clippy -- -D warnings` passes, scan behavior preserved.

### Phase C — Duration & File Count (US2 + US3)

**Scope**: Populate `duration_seconds`, `duration_is_estimated`, `file_count` during the parallel phase.

1. In `extract_directory_metadata()`:
   - Walk all MP3 paths, count them → `file_count`.
   - For each MP3: call `tag.duration()`. If `Some(ms)` and `ms > 0`: add `ms / 1000` to TLEN sum. Otherwise: estimate `(file_size_bytes - tag_size_bytes) / 16_000` seconds, set `estimated = true`.
   - Sum durations → `duration_seconds`; set `duration_is_estimated` accordingly.
2. Serial phase: pass `duration_seconds`, `duration_is_estimated`, `file_count` into the `Book` constructor.
3. Add `#[cfg(test)]` unit tests:
   - `test_duration_from_tlen_tags` — fixture files with TLEN → `duration_is_estimated = false`, correct sum
   - `test_duration_fallback_estimation` — no TLEN → `duration_is_estimated = true`, non-zero value
   - `test_duration_zero_byte_files` — zero-byte MP3 → `duration_seconds = 0`, no panic
   - `test_file_count_mp3_only` — mixed file types → only `.mp3` counted
4. **Gate**: SC-003 and SC-004 satisfied; unit tests pass; `cargo tarpaulin` ≥ 80% on new functions.

### Phase D — Orphan Detection (US4 + FR-011)

**Scope**: Integrate orphan-check phase post main scan loop for efficient detection and bulk flagging.

1. Add to books_service.rs:

- bulk_update_books_orphaned(orphaned_titles: &[&str], flag: bool, conn: &mut SqliteConnection) — filters with title.eq_any(orphaned_titles) then .set(dsl::orphaned.eq(flag)).
- get_all_books(conn: &mut SqliteConnection) -> Vec<Book> — selects all records via books.load::<Book>(conn).

2. In quick_scan(), after serial write phase:

- Fetch all_books = get_all_books(conn)?.
- Iterate: construct PathBuf::from(absolute_root).join(relative_file_path); partition into orphaned_titles: Vec<&str> (missing) and present_titles: Vec<&str> (exists).
- Wrap in conn.transaction(|conn| { bulk_update_books_orphaned(&orphaned_titles, true, conn)?; bulk_update_books_orphaned(&present_titles, false, conn)?; Ok(()) })?;
- Chunk titles at 999 per call if >999 (loop over chunks(999)).
- For each new orphan: warn!("Orphaned book: {}", title);
- Populate ScanReport.books_newly_orphaned = orphaned_titles.len() as u32.

3. Add #[cfg(test)] unit tests (using mock conn, temp paths):

- test_orphan_marks_missing_path — insert book with fake path; scan detects missing → bulk update sets orphaned=true; verify log.warn! and DB state.
- test_orphan_clears_restored_path — insert orphaned book; create path → scan sets orphaned=false; verify update.

4. Gate: SC-009 met; cargo test passes; cargo clippy clean

### Phase E — Benchmark Fixture & Validation (FR-012)

**Scope**: Create the benchmark infrastructure; confirm SC-002.

> ⚠️ **Ordering note**: Steps 1–3 (fixture creation, gen_fixtures binary, and baseline recording) MUST be completed **before Phase B** — the baseline must be measured against the unmodified serial scanner. In tasks.md these steps appear in Phase 1 (Setup) for this reason. Only step 4 (final benchmark validation against the post-refactor binary) belongs at the end of the implementation.

1. Write `src-tauri/src/bin/gen_fixtures.rs` — generates 1,000 dirs × 10 minimal MP3s using `id3::Tag::write_to_path`. Each file: TALB, TPE2, TLEN=180000 (3 min), TRCK.
2. Write `tests/bench_scanner.sh` — runs `quick_scan` against the fixture with a fresh DB, records wall-clock time, compares to `tests/bench_baseline.txt`, exits 1 on regression.
3. Record pre-refactor baseline on the `main` branch binary: `bash tests/bench_scanner.sh --record-baseline` → writes `tests/bench_baseline.txt`.
4. After Phases B–D complete: run benchmark; validate ≤ 30 s and ≥ 2× speedup.
5. Commit the generator binary (`gen_fixtures.rs`), the benchmark script (`bench_scanner.sh`), and the baseline timing (`tests/bench_baseline.txt`) to the branch. The generated fixture directory (`tests/fixtures/bench-library/`) is excluded via `.gitignore` — it is regenerated on-demand by running `cargo run --features dev-fixtures --bin gen_fixtures`.
6. **Gate**: SC-002 satisfied — benchmark passes.

---

## Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| rayon `par_iter` races on the old `HashSet<processed_dirs>` | High if not removed | Crash / UB | Replace shared `HashSet` with pre-deduplication of parent dirs before `par_iter` (serial WalkDir pass collects dirs, parallel phase operates on the deduplicated list) |
| `SQLITE_MAX_VARIABLE_NUMBER` exceeded in `eq_any` | Medium (libraries > 999 books) | Diesel runtime error | Chunk `eq_any` lists at 999; implemented in Phase D |
| `tag.duration()` always returns `None` for a user's encoding | Low | SC-003 miss | Fallback estimation path is always active; `duration_seconds = 0` only for truly empty/unreadable files |
| Migration `DROP COLUMN` fails on SQLite < 3.35 | Medium | `diesel migration revert` fails | Document minimum SQLite version in quickstart.md; the `down.sql` is a dev/test convenience only — production never needs to revert |
| `gen_fixtures` binary bloating the release build | Low | Binary size increase | Declare as `[[bin]]` with `required-features = ["dev-fixtures"]` or place in `tests/` as an integration test binary |

---

## Acceptance Checklist (Pre-Merge Gate)

- [ ] `diesel migration run` succeeds; `schema.rs` contains all 4 new columns
- [ ] `cargo check` — zero errors
- [ ] `tsc --noEmit` — zero errors
- [ ] `cargo test` — all tests pass
- [ ] `cargo tarpaulin` — ≥ 80% line coverage on new `scanner.rs` functions (SC-006)
- [ ] `cargo clippy -- -D warnings` — zero warnings (SC-007)
- [ ] `cargo fmt --check` — zero deviations (SC-007)
- [ ] `npm run lint` — zero errors
- [ ] SC-001: no-op scan completes in ≤ 2 s on benchmark fixture
- [ ] SC-002: full scan ≤ 30 s and ≥ 2× vs baseline on benchmark fixture
- [ ] SC-003: all new book records have non-null `duration_seconds` and `duration_is_estimated`
- [ ] SC-004: all new book records have `file_count ≥ 1`
- [ ] SC-005: scan with 5% corrupt files returns `Ok(ScanReport)` with warnings logged
- [ ] SC-008: existing rows post-migration have all 4 new columns = NULL
- [ ] SC-009: 5 deleted directories → exactly 5 records `orphaned = true`; all others `orphaned = false`
- [ ] `tests/fixtures/bench-library/` and `tests/bench_baseline.txt` committed
- [ ] `src/app/models/books.ts` updated with 4 new fields + `ScanReport` interface
