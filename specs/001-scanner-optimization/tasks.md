# Tasks: Scanner Optimization & Enhancement

**Input**: Design documents from `specs/001-scanner-optimization/`
**Prerequisites**: plan.md ✅ spec.md ✅ research.md ✅ data-model.md ✅ contracts/ ✅ quickstart.md ✅

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no incomplete task dependencies)
- **[Story]**: Which user story this task belongs to — [US1], [US2], [US3], [US4]
- No story label in Setup, Foundational, and Polish phases

---

## Phase 1: Setup

**Purpose**: Install new dependencies and create the benchmark infrastructure *before* the scanner is refactored. The pre-refactor baseline must be recorded here, before any Phase 3 changes alter scanner performance.

- [ ] T001 [P] Add `rayon` and `log` to `[dependencies]` in `src-tauri/Cargo.toml`
- [ ] T002 [P] Create `src-tauri/src/bin/gen_fixtures.rs` — binary that writes 1,000 dirs × 10 minimal ID3v2 MP3 files using `id3::Tag::write_to_path` (fields: TALB, TPE2, TLEN=180000, TRCK); add `required-features = ["dev-fixtures"]` to the `[[bin]]` entry in `Cargo.toml` so the binary is excluded from default/release builds
- [ ] T003 [P] Write `tests/bench_scanner.sh` — measures wall-clock time of `quick_scan` against `tests/fixtures/bench-library/`, compares to `tests/bench_baseline.txt`, exits 1 on regression
- [ ] T004 Run `cargo run --bin gen_fixtures` to populate `tests/fixtures/bench-library/` (depends on T002)
- [ ] T005 Record pre-refactor baseline: run `bash tests/bench_scanner.sh --record-baseline` → writes `tests/bench_baseline.txt` (depends on T003, T004)

**Checkpoint**: Benchmark infrastructure in place; baseline captured on the unmodified scanner. Safe to begin model changes.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Database migration and model sync. These changes compile-break the codebase intentionally and MUST be resolved before any user story work can begin.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete — the `Book` struct must compile with its new fields.

- [ ] T006 Generate Diesel migration directory by running `diesel migration generate scanner_fields` in `src-tauri/`
- [ ] T007 Write `src-tauri/migrations/<timestamp>_scanner_fields/up.sql` — four `ALTER TABLE books ADD COLUMN` statements: `duration_seconds INTEGER DEFAULT NULL`, `duration_is_estimated BOOLEAN DEFAULT NULL`, `file_count INTEGER DEFAULT NULL`, `orphaned BOOLEAN DEFAULT NULL`
- [ ] T008 [P] Write `src-tauri/migrations/<timestamp>_scanner_fields/down.sql` — four `ALTER TABLE books DROP COLUMN` statements (depends on T006 for directory; parallel with T007)
- [ ] T009 Run `diesel migration run` in `src-tauri/`; verify `src-tauri/src/schema.rs` auto-regenerates with the four new columns in the `books` table (depends on T007, T008)
- [ ] T010 Add `duration_seconds: Option<i32>`, `duration_is_estimated: Option<bool>`, `file_count: Option<i32>`, `orphaned: Option<bool>` to `src-tauri/src/models/book.rs`; initialize all four to `None` in existing `Book { ... }` construction sites in `src-tauri/src/scanner.rs` to fix compile errors (depends on T009)
- [ ] T011 [P] Update `src/app/models/books.ts` — add `duration_seconds: number | null`, `duration_is_estimated: boolean | null`, `file_count: number | null`, `orphaned: boolean | null`, `relative_file_path: string` to the `Book` interface; add `ScanReport` interface (Principle IV — parallel with T010, different file)

**Checkpoint**: `cargo check` ✅ `tsc --noEmit` ✅ `cargo test` ✅ (existing tests pass with new `None` fields)

---

## Phase 3: User Story 1 — Incremental Scan (Priority: P1) 🎯 MVP

**Goal**: Replace serial scanner with a two-phase parallel-collect → serial-write architecture. Existing books are skipped; only new directories produce DB writes. `quick_scan_command` returns `ScanReport`.

**Independent Test**: Run a scan on a library that is already fully indexed. Confirm `books_added = 0` and the command returns in under 2 seconds regardless of library size (SC-001).

- [ ] T012 [US1] Define `DirectoryMetadata`, `ScanError`, and `ScanReport` structs in `src-tauri/src/scanner.rs` (see data-model.md §2–4 for field definitions)
- [ ] T013 [US1] Rewrite the `WalkDir` loop in `quick_scan()` in `src-tauri/src/scanner.rs` to collect a deduplicated `Vec<PathBuf>` of unique parent directories before any processing (replaces the current `HashSet<processed_dirs>` approach)
- [ ] T014 [US1] Extract `extract_directory_metadata(dir: &Path, base_path: &Path) -> Result<DirectoryMetadata, ScanError>` function in `src-tauri/src/scanner.rs` — wraps existing `get_mp3_path`, `Tag::read_from_path`, cover/author-photo logic; new fields (`duration_seconds`, `file_count`) left as `None`/`0` placeholders for Phase 5–6
- [ ] T015 [US1] Add `rayon::par_iter()` parallel phase to `quick_scan()` in `src-tauri/src/scanner.rs` — calls `extract_directory_metadata` per directory, collects `Vec<Result<DirectoryMetadata, ScanError>>`
- [ ] T016 [US1] Implement serial write phase in `quick_scan()` in `src-tauri/src/scanner.rs` — for each `Ok(meta)`: call `is_book_exists`, build `Book` (new fields all `None`), call `add_book`, tally `books_added` / `books_skipped`; for each `Err(e)`: tally `errors`; change return type to `Result<ScanReport, String>`
- [ ] T017 [US1] Update `quick_scan_command` in `src-tauri/src/commands/settings_commands.rs` — wrap call with `tokio::task::spawn_blocking(|| quick_scan()).await`, change return type to `Result<ScanReport, String>`
- [ ] T018 [US1] Add `#[cfg(test)]` unit tests in `src-tauri/src/scanner.rs`: `test_no_op_scan_returns_zero_added` (existing book skipped), `test_scan_adds_only_new_directories` (new dirs inserted, existing untouched), and `test_relative_path_strips_absolute_root` (assert stored `relative_file_path` does not start with the absolute root prefix — Principle VI)

**Checkpoint**: US1 fully functional — incremental scan works, `ScanReport` returned, no regression on existing functionality.

---

## Phase 4: User Story 4 — Resilient Scan: No Crash on Bad Data (Priority: P2)

**Goal**: Eliminate all `unwrap`/`panic` from `scanner.rs`, replace `println!` with structured logging, handle corrupt/permission/empty cases gracefully, add orphan-detection phase, emit FR-010 completion report.

**Independent Test**: Place a zero-byte file named `broken.mp3` in a test directory alongside valid books. Run scanner. Assert valid books are added, `errors = 1`, function returns `Ok(ScanReport)` — no panic.

- [ ] T019 [P] [US4] Add `get_all_books() -> Vec<Book>` and `update_books_orphaned(titles: &[String], flag: bool, conn: &mut SqliteConnection)` helper functions to `src-tauri/src/services/books_service.rs` — use `dsl::title.eq_any(chunk)` bulk update, chunk at 999 (parallel: different file from scanner.rs)
- [ ] T020 [US4] Replace all `unwrap()` and `panic!()` calls in `src-tauri/src/scanner.rs` with `?`, `match`, or `if let` — ensure every error path returns a `ScanError` or propagates via `Result` (FR-005)
- [ ] T021 [US4] Replace all `println!` macros with `log::warn!` / `log::info!` throughout `src-tauri/src/scanner.rs` (FR-006)
- [ ] T022 [US4] Handle tag parse failure (`Tag::read_from_path` returns `Err`) in `extract_directory_metadata` in `src-tauri/src/scanner.rs` — return `Err(ScanError { path, message })` instead of silently dropping
- [ ] T023 [US4] Handle permission-denied `WalkDir` entries in the directory-collection pass in `src-tauri/src/scanner.rs` — match on `WalkDir` errors, call `log::warn!`, continue iteration
- [ ] T024 [US4] Silently skip directories that contain zero `.mp3` files in `src-tauri/src/scanner.rs` — no `Book` record created, no error counted, no log entry
- [ ] T025 [US4] Implement orphan-check phase in `quick_scan()` in `src-tauri/src/scanner.rs` after the serial write loop — load all books via `get_all_books`, resolve `absolute_root/relative_file_path`, collect `orphaned_titles` and `present_titles`, call `update_books_orphaned` in a single `conn.transaction` (chunked at 999), `log::warn!` per orphaned title, set `ScanReport.books_newly_orphaned` (depends on T019)
- [ ] T026 [US4] Add `log::info!` completion summary at end of `quick_scan()` in `src-tauri/src/scanner.rs` — logs `books_added`, `books_skipped`, `books_newly_orphaned`, `errors`, `elapsed_ms` (FR-010)
- [ ] T027 [US4] Add `#[cfg(test)]` unit tests in `src-tauri/src/scanner.rs`: `test_corrupt_mp3_scan_continues` (error counted, valid books added), `test_zero_mp3_dir_silently_skipped` (no record, no error), `test_orphan_marks_missing_path` (orphaned=true + warning), `test_orphan_clears_restored_path` (orphaned=false)

**Checkpoint**: US4 fully functional — scanner never panics, all error paths log and continue, orphan detection runs each cycle.

---

## Phase 5: User Story 2 — Book Duration Stored and Accessible (Priority: P2)

**Goal**: Populate `duration_seconds` and `duration_is_estimated` on every newly scanned `Book`. `false` when all files had valid TLEN tags; `true` when any file required file-size estimation.

**Independent Test**: Scan a directory with 3 MP3s each having `TLEN=180000`. Assert `duration_seconds = 540`, `duration_is_estimated = false`. Repeat with TLEN absent; assert estimated value > 0 and `duration_is_estimated = true`.

- [ ] T028 [US2] Add TLEN extraction to `extract_directory_metadata()` in `src-tauri/src/scanner.rs` — for each MP3 path call `tag.duration()` (returns `Option<u32>` ms); if `Some(ms)` and `ms > 0` add `ms / 1000` to TLEN sum
- [ ] T029 [US2] Add fallback duration estimation to `extract_directory_metadata()` in `src-tauri/src/scanner.rs` — when `tag.duration()` returns `None` or `Some(0)`, estimate `(file_bytes - tag_size_bytes) / 16_000` seconds (128 kbps default); set `duration_is_estimated = true`; document the 128 kbps assumption in a comment
- [ ] T030 [US2] Propagate `duration_seconds: Some(total_secs)` and `duration_is_estimated: Some(flag)` from `DirectoryMetadata` into `Book` construction in the serial write phase of `src-tauri/src/scanner.rs`
- [ ] T031 [US2] Add `#[cfg(test)]` unit tests in `src-tauri/src/scanner.rs`: `test_duration_from_tlen_tags` (all TLEN present → estimated=false, correct sum), `test_duration_fallback_estimation` (no TLEN → estimated=true, value > 0), `test_duration_zero_byte_files` (zero-byte MP3 → duration_seconds=0, no panic)

**Checkpoint**: US2 fully functional — every new book record has non-null `duration_seconds` and `duration_is_estimated` (SC-003).

---

## Phase 6: User Story 3 — File Count Stored per Book (Priority: P3)

**Goal**: Populate `file_count` on every newly scanned `Book` with the exact count of `.mp3` files in its directory. Non-MP3 files are ignored.

**Independent Test**: Scan a directory with exactly 12 `.mp3` files and 3 `.png` files. Assert `file_count = 12`.

- [ ] T032 [US3] Add `.mp3`-only file counting to `extract_directory_metadata()` in `src-tauri/src/scanner.rs` — count paths whose extension (lowercased) equals `"mp3"`; store as `file_count: i32` in `DirectoryMetadata`
- [ ] T033 [US3] Propagate `file_count: Some(count)` from `DirectoryMetadata` into `Book` construction in the serial write phase of `src-tauri/src/scanner.rs`
- [ ] T034 [US3] Add `#[cfg(test)]` unit tests in `src-tauri/src/scanner.rs`: `test_file_count_mp3_only` (mixed file types → only .mp3 counted) and `test_file_count_single_file` (one MP3 → file_count=1)

**Checkpoint**: US3 fully functional — every new book record has `file_count ≥ 1` (SC-004). All four user stories independently deliverable.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final quality gates — coverage, lint, format, and performance validation against the benchmark baseline.

- [ ] T035 [P] Run `cargo tarpaulin --include-files src/scanner.rs --out Stdout` in `src-tauri/`; verify ≥ 80% line coverage on new functions (SC-006); fix any gaps
- [ ] T036 [P] Run `cargo clippy -- -D warnings` and `cargo fmt --check` in `src-tauri/`; fix all reported issues (SC-007)
- [ ] T037 [P] Run `npm run lint` and `tsc --noEmit` from repo root; fix all issues (SC-007, Principle IV)
- [ ] T038 Run `bash tests/bench_scanner.sh` against `tests/fixtures/bench-library/`; confirm SC-001 (no-op scan ≤ 2 s) and SC-002 (full scan ≤ 30 s, ≥ 2× faster than baseline in `tests/bench_baseline.txt`)

**Checkpoint**: All acceptance checklist items from plan.md satisfied. Branch ready for PR.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately
- **Phase 2 (Foundational)**: Can begin independently of Phase 1 — migration, model, and TS interface tasks have no rayon/log dependency. Phase 3 is the first phase that requires rayon/log to be present in `Cargo.toml`
- **Phase 3 (US1)**: Depends on Phase 2 completion — `Book` struct must compile with new fields
- **Phase 4 (US4)**: Depends on Phase 3 — builds on top of `extract_directory_metadata` and two-phase architecture
- **Phase 5 (US2)**: Depends on Phase 3 — adds duration logic to `extract_directory_metadata`; can run in parallel with Phase 4
- **Phase 6 (US3)**: Depends on Phase 3 — adds file_count to `extract_directory_metadata`; can run in parallel with Phase 4 and 5
- **Phase 7 (Polish)**: Depends on Phases 3–6 complete

### User Story Dependencies

- **US1 (P1)**: Can start after Phase 2 — no dependencies on other stories
- **US4 (P2)**: Depends on US1 (requires the `extract_directory_metadata` function and two-phase architecture)
- **US2 (P2)**: Depends on US1 (same function) — independent of US4
- **US3 (P3)**: Depends on US1 (same function) — independent of US2 and US4

### Critical Ordering Constraint

**T005 (record baseline) MUST execute before T015 (add rayon par_iter)**. The baseline must be measured on the serial scanner. Once Phase 3 work begins, the baseline is invalidated.

### Within Each User Story

- Types before functions (T012 before T013–T016)
- Service helpers before scanner consumers (T019 before T025)
- Implementation before unit tests (T016 before T018, T026 before T027, etc.)

---

## Parallel Execution Examples

### Phase 1

```
T001 (Cargo.toml)     ──┐
T002 (gen_fixtures.rs)──┤ can run in parallel
T003 (bench_scanner.sh) ┘
         ↓
T004 (generate fixtures) ← depends on T002
         ↓
T005 (record baseline) ← depends on T003 + T004
```

### Phase 2

```
T007 (up.sql) ──┐ can run in parallel
T008 (down.sql)─┘
       ↓
T009 (diesel migration run)
       ↓
T010 (book.rs update) ──┐ can run in parallel
T011 (books.ts update)──┘ (different files)
```

### Phase 4 (US4)

```
T019 (books_service.rs) ──┐ parallel: different file
                           ↓ (T019 complete)
T020 → T021 → T022 → T023 → T024 → T025 (depends on T019) → T026 → T027
       (all scanner.rs — sequential within file)
```

### Phase 5 + Phase 6 (parallel with each other, both after Phase 3)

```
Phase 5: T028 → T029 → T030 → T031  (scanner.rs duration logic)
Phase 6: T032 → T033 → T034          (scanner.rs file_count logic)
```
*Note: US2 and US3 both modify `scanner.rs`. If worked in parallel by different developers, coordinate on `extract_directory_metadata` to avoid merge conflicts.*

### Phase 7

```
T035 (tarpaulin) ──┐
T036 (clippy/fmt)──┤ can run in parallel
T037 (lint/tsc)  ──┘
         ↓
T038 (benchmark validation) ← run last
```

---

## Implementation Strategy

### MVP (User Story 1 Only)

1. Complete Phase 1: Setup (T001–T005)
2. Complete Phase 2: Foundational (T006–T011)
3. Complete Phase 3: US1 (T012–T018)
4. **STOP AND VALIDATE**: Confirm incremental scan works, no-op returns in ≤ 2 s, `ScanReport` returned
5. Partial Phase 7: run T036 (clippy/fmt) and T037 (lint/tsc) to verify code quality

MVP delivers: parallel scan architecture, incremental (skip-existing) behavior, `ScanReport` telemetry.

### Incremental Delivery

1. Setup + Foundational → base compiles with new fields ✅
2. US1 → incremental scan + rayon architecture (MVP!)
3. US4 → resilience + orphan detection (no crashes in production)
4. US2 → duration metadata (user-visible value)
5. US3 → file count (low cost, browsing context)
6. Polish → coverage + lint + benchmark confirms SC-001/SC-002

### Parallel Team Strategy

With two developers after Phase 2 complete:
- Developer A: US1 (Phase 3) → US4 (Phase 4) sequential
- Developer B: US2 (Phase 5) and US3 (Phase 6) after Phase 3 complete

---

## Task Summary

| Phase | Tasks | Story | Parallelizable |
|-------|-------|-------|---------------|
| Phase 1: Setup | T001–T005 | — | T001, T002, T003 |
| Phase 2: Foundational | T006–T011 | — | T008, T011 |
| Phase 3: US1 P1 | T012–T018 | [US1] | — |
| Phase 4: US4 P2 | T019–T027 | [US4] | T019 |
| Phase 5: US2 P2 | T028–T031 | [US2] | — |
| Phase 6: US3 P3 | T032–T034 | [US3] | — |
| Phase 7: Polish | T035–T038 | — | T035, T036, T037 |
| **Total** | **38 tasks** | | **8 [P] opportunities** |

| Story | Tasks | Independent Test |
|-------|-------|-----------------|
| US1 (P1) | T012–T018 (7) | No-op scan → `books_added=0` in ≤ 2 s |
| US4 (P2) | T019–T027 (9) | `broken.mp3` in valid library → `Ok(ScanReport)`, `errors=1` |
| US2 (P2) | T028–T031 (4) | 3-file book with TLEN → `duration_seconds=540`, `estimated=false` |
| US3 (P3) | T032–T034 (3) | 12 MP3s + 3 PNGs → `file_count=12` |
