# Feature Specification: Scanner Optimization & Enhancement

**Feature Branch**: `001-scanner-optimization`
**Created**: 2026-02-18
**Status**: Draft
**Input**: User description: "Scanner Optimization & Enhancement — improve the audiobook scanning process for better performance, reliability, and user experience. Add book duration and file count. Incremental scanning. Error handling. Parallel processing."

## Clarifications

### Session 2026-02-18

- Q: When `TLEN` is missing, should duration be estimated (with a tolerance) or dropped entirely? → A: Estimate from file size/bitrate as a fallback, but add a `duration_is_estimated: Option<bool>` field to the `Book` model so callers can always distinguish real tag data (`false`) from a calculated value (`true`).
- Q: What should happen when a book directory already exists in the DB but its files have changed on disk? → A: Skip by default (existing records are never modified by the regular scan); a separate `force_rescan_command` that refreshes metadata-only fields without touching user fields (`score`, `read`) is planned but deferred to a follow-on feature.
- Q: What should happen when a previously scanned book's directory no longer exists on disk? → A: Log a warning identifying the missing path and mark the book as `orphaned = true` in the database. The record is preserved; no data is deleted.
- Q: How should the 2× performance improvement target be validated? → A: Include a reproducible benchmark script alongside a synthetic test library fixture committed to the repo; measure both the pre-refactor and post-refactor binary on the same machine against that fixture.
- Q: Should `rayon` be the required parallelism mechanism, or left open to implementation? → A: Require `rayon` explicitly. The Principle VII (YAGNI) justification is satisfied — no stdlib alternative achieves the same ergonomics without significant boilerplate. The Tauri async/rayon thread boundary constraint (rayon must never block the Tokio IPC thread) must be documented in code and respected in implementation.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Incremental Scan (New Books Only) (Priority: P1)

A user with an existing library triggers a scan after adding new audiobooks to their collection. The scanner identifies only the new directories and adds them to the database, leaving existing books untouched. The scan completes noticeably faster than a full rescan would.

**Why this priority**: This is the most impactful reliability and performance improvement. Without it, every scan re-processes the entire library, wastes time, and risks overwriting user edits (score, read status). All other improvements are secondary to correctness here.

**Independent Test**: Run a scan on a library that is already fully indexed. Confirm zero new books are added and the command returns in under 2 seconds regardless of library size.

**Acceptance Scenarios**:

1. **Given** a library with 500 books already in the database, **When** `quick_scan_command` is invoked with no new directories added, **Then** zero new book records are created and the scan finishes in under 2 seconds.
2. **Given** a library with 500 existing books and 10 new book directories added on disk, **When** `quick_scan_command` is invoked, **Then** exactly 10 new book records are inserted and the 500 existing records are unchanged.
3. **Given** the database contains a book record for a directory, **When** the scan runs again without any file changes in that directory, **Then** the existing book's `score`, `read`, and other user-editable fields are preserved exactly as stored.

---

### User Story 2 — Book Duration Stored and Accessible (Priority: P2)

A user browses their library and wants to see how long each audiobook is. The scanner extracts or estimates the total listening duration for each book and stores it. The value appears as a populated `duration_seconds` field when a book is retrieved via the existing `get_book_command`.

**Why this priority**: Duration is one of the most useful metadata fields for audiobook listeners. It can be derived purely from scanning logic (no UI or new command needed) and is independently deliverable.

**Independent Test**: Scan a directory containing a multi-file audiobook. Retrieve the book record and assert that `duration_seconds` is greater than zero, and that `duration_is_estimated` is `false` when `TLEN` tags were present, or `true` when duration was calculated from file size and bitrate.

**Acceptance Scenarios**:

1. **Given** a book directory with 3 MP3 files each containing a valid `TLEN` (length) ID3 tag, **When** the scanner processes that directory, **Then** the stored `duration_seconds` equals the sum of the three TLEN values (in seconds) and `duration_is_estimated` is `false`.
2. **Given** a book directory with MP3 files that have no `TLEN` tag, **When** the scanner processes that directory, **Then** `duration_seconds` contains a non-zero estimated value derived from file size and average bitrate, and `duration_is_estimated` is `true`.
3. **Given** a book directory with entirely unreadable or zero-length MP3 files, **When** the scanner processes that directory, **Then** `duration_seconds` is stored as `0` (zero) and the book record is still created successfully without a panic or crash.

---

### User Story 3 — File Count Stored per Book (Priority: P3)

A user wants to know how many chapter files a book contains (e.g., "24 files" indicates a long multi-part audiobook). The scanner counts the MP3 files in each book directory and stores the count alongside the book record.

**Why this priority**: Low implementation cost (already traversing the directory) and delivers browsing context. Dependent on the same directory-walk logic as duration.

**Independent Test**: Scan a directory with a known number of MP3 files. Retrieve the book record and assert `file_count` equals the exact number of MP3 files in that directory.

**Acceptance Scenarios**:

1. **Given** a book directory containing exactly 12 MP3 files, **When** the scanner processes that directory, **Then** the stored `file_count` equals 12.
2. **Given** a book directory containing MP3 files and also PNG/NFO/other files, **When** the scanner processes that directory, **Then** `file_count` counts only the `.mp3` files and ignores non-MP3 files.
3. **Given** a directory that contains a single MP3 file, **When** the scanner processes it, **Then** `file_count` is stored as 1.

---

### User Story 4 — Resilient Scan: No Crash on Bad Data (Priority: P2)

A user triggers a scan over a library that contains corrupt MP3 files, permission-denied directories, or directories with no MP3 files. The scanner logs errors for each problematic entry and continues scanning all remaining directories. The user receives a completion report indicating how many books were added and how many directories were skipped due to errors.

**Why this priority**: Tied with P2 for duration because a single crash in a large library renders the entire feature unusable. Reliability is a hard requirement from the constitution (Principle V — no panics, no unwrap outside tests).

**Independent Test**: Place a zero-byte file named `broken.mp3` alongside valid MP3s in a test directory. Run the scanner. Confirm that the valid books are added, the broken file is skipped with an error logged, and the function returns `Ok(())` (not an error or panic).

**Acceptance Scenarios**:

1. **Given** a library directory containing one corrupt MP3 (unreadable ID3 tags) alongside 10 valid book directories, **When** `quick_scan_command` is invoked, **Then** the 10 valid books are added, the corrupt file generates a logged warning, and the command returns successfully.
2. **Given** a directory that is inaccessible due to filesystem permissions, **When** the scanner traverses past it, **Then** the scanner logs the permission error and continues to the next directory without aborting.
3. **Given** a directory that contains no MP3 files at all (only images or text files), **When** the scanner encounters it, **Then** the directory is silently skipped with no book record created and no error returned.

---

### Edge Cases

- What happens when a book's directory is deleted from disk after a previous scan? During the next scan's orphan-check phase, the scanner resolves each existing book's `relative_file_path` against the active absolute root. If the resolved path no longer exists, the book is marked `orphaned = true` and a warning is logged. The record and all user data (`score`, `read`) are preserved intact.
- What happens when the active absolute root is switched to a different library root between scans? Orphan detection only checks books whose `relative_file_path` resolves under the current active root. Books belonging to an inactive root are not checked and are never marked orphaned as a result of a root switch.
- What happens when a book directory's files are replaced or re-encoded (same title, changed files)? The scanner skips the directory because the title already exists. `duration_seconds`, `file_count`, and `duration_is_estimated` retain their original scanned values. A future `force_rescan_command` (out of scope here) will provide a user-initiated path to refresh these metadata fields without touching `score` or `read`.
- What happens when two different directories produce the same album title (ID3 `TALB` tag)? The second directory is skipped by the existing `is_book_exists` guard. A warning is logged identifying the collision. This is a known limitation of the `title`-as-primary-key design (tracked as a long-term debt item in the constitution).
- What happens when the library root (`absolute_path`) does not exist on disk? The scanner returns `Err("No path to scan")` immediately without traversing anything.
- What happens when an MP3 file reports a `TLEN` duration of zero? The fallback file-size-based estimation is applied instead.
- What happens when `rayon` parallel workers encounter a race condition writing to the shared database? The `books_service::add_book` function must be called sequentially after parallel metadata collection — parallel work is scoped to I/O and metadata extraction only, not database writes.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The scanner MUST skip any book directory whose album title already exists in the `books` table, performing no database write and no further file I/O for that directory. The only permitted write to an existing record is setting `orphaned = true` during the orphan-check phase (FR-011). User-editable fields (`score`, `read`) MUST never be modified by the scanner under any circumstance.
- **FR-002**: The scanner MUST extract total book duration by summing the `TLEN` (length in milliseconds) ID3 frames across all MP3 files in the directory, then converting to whole seconds.
- **FR-003**: When `TLEN` is absent or zero for any file in a directory, the scanner MUST estimate that file's duration from its byte size and the bitrate read from the ID3 header (defaulting to 128 kbps if unavailable), sum estimated durations across all files, and set `duration_is_estimated` to `true`. When every file in the directory has a valid, non-zero `TLEN` tag, `duration_is_estimated` MUST be set to `false`.
- **FR-004**: The scanner MUST count the number of files with the `.mp3` extension in a book's directory and store that count as `file_count`.
- **FR-005**: The scanner MUST NOT call `unwrap()` or `panic!()` anywhere in `scanner.rs`; all fallible operations MUST propagate errors via `Result` or be explicitly handled with `match`/`if let`.
- **FR-006**: The scanner MUST log a warning (via the `log` crate's `warn!` macro) for each directory or file that cannot be processed, and then continue scanning remaining entries.
- **FR-007**: The `Book` model MUST include four new optional fields: `duration_seconds: Option<i32>`, `duration_is_estimated: Option<bool>`, `file_count: Option<i32>`, and `orphaned: Option<bool>`.
- **FR-008**: A Diesel migration MUST add `duration_seconds INTEGER`, `duration_is_estimated BOOLEAN`, `file_count INTEGER`, and `orphaned BOOLEAN` columns to the `books` table with `NULL` as the default for all four, preserving all existing rows.
- **FR-009**: The scanner MUST use `rayon` for parallel metadata extraction (directory traversal, ID3 tag reading, duration calculation, file counting). `rayon` work MUST be confined to a `rayon::scope` or `par_iter` call that completes before any database write begins — it MUST NOT hold, await, or block the Tauri Tokio IPC thread. Database writes (via `books_service` and `authors_service`) MUST remain sequential and execute after the parallel phase returns its collected results.
- **FR-010**: The scan completion log MUST report: total books added, total books skipped (already existed), total books newly marked orphaned, total errors encountered, and elapsed time.
- **FR-011**: After directory traversal completes, the scanner MUST perform an orphan-check phase: for every book record in the database whose `relative_file_path` resolves (via the active absolute root) to a path that no longer exists on disk, the scanner MUST set `orphaned = true` on that record and log a warning with the resolved path. Books whose paths still exist MUST have `orphaned` set to `false` (clearing any previous orphan flag). Books belonging to an inactive absolute root MUST be skipped during orphan-check.
- **FR-012**: A synthetic benchmark fixture MUST be created at `tests/fixtures/bench-library/` containing a reproducible structure of 1,000 simulated book directories, each with 10 minimal valid MP3 files. A benchmark shell script at `tests/bench_scanner.sh` MUST record wall-clock time for a full scan of this fixture and compare it against a pre-recorded baseline (the pre-refactor binary time, stored in `tests/bench_baseline.txt`).

### Key Entities

- **Book** (extended): Represents an audiobook in the database. Gains four new fields: `duration_seconds` (total listening time in whole seconds, nullable), `duration_is_estimated` (boolean; `true` when duration was calculated from file size/bitrate rather than read from a `TLEN` tag, nullable), `file_count` (number of MP3 chapter files, nullable), and `orphaned` (boolean; `true` when the book's resolved file path no longer exists on disk, `false` when confirmed present, `NULL` for rows created before this feature). All four default to `NULL` for existing rows post-migration.
- **ScanResult** (new, in-memory only): An ephemeral value type returned by the per-directory scanning logic, carrying extracted metadata before it is committed to the database. Contains: `book: Book`, `errors: Vec<String>`. Never persisted directly — it is a transport type internal to `scanner.rs`.
- **DirectoryMetadata** (new, in-memory only): Collected per directory during the parallel phase. Carries: `parent_path: String`, `mp3_paths: Vec<PathBuf>`, `first_tag: Option<Tag>`, `file_create_date: Option<NaiveDateTime>`. Passed to the sequential commit phase.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Scanning a library with 1,000 existing books (no new additions) completes in under 2 seconds on a modern laptop (≥ 4 cores, SSD).
- **SC-002**: Scanning a fresh synthetic library of 1,000 books (10,000 MP3 files, committed as a test fixture under `tests/fixtures/bench-library/`) completes in under 30 seconds on a modern laptop (≥ 4 cores, SSD) and is at least 2× faster than the pre-refactor binary measured on the same machine using the benchmark script at `tests/bench_scanner.sh`.
- **SC-003**: Every newly scanned book record has non-null `duration_seconds` and `duration_is_estimated` values. `duration_seconds = 0` is acceptable only when all files are unreadable or empty (in which case `duration_is_estimated` is `true`). `duration_is_estimated` is `false` only when every file in the directory provided a valid, non-zero `TLEN` tag.
- **SC-004**: Every newly scanned book record has a non-null `file_count` value of at least `1`.
- **SC-005**: A scan over a library containing 5% corrupt or permission-denied files completes successfully (`Ok(())`) with a warning logged per bad file and all valid books correctly inserted.
- **SC-006**: Unit tests in `scanner.rs` under `#[cfg(test)]` achieve coverage of at least 80% of new functions (measured by line coverage with `cargo tarpaulin` or equivalent).
- **SC-007**: `cargo clippy -- -D warnings` and `cargo fmt --check` both pass with zero errors on `scanner.rs` after the refactor.
- **SC-008**: All existing book records in the database remain valid and unmodified after running the migration (`duration_seconds = NULL`, `duration_is_estimated = NULL`, `file_count = NULL`, `orphaned = NULL` for pre-existing rows).
- **SC-009**: After a scan over a library from which 5 book directories have been deleted since the previous scan, exactly those 5 records have `orphaned = true`, a warning was logged for each, and all other records have `orphaned = false`.

---

## Assumptions

- One directory = one audiobook (current scanner invariant). Multi-disc sets stored in subdirectories are treated as separate books (no change to this behaviour in this feature).
- The `rayon` crate MUST be added as a direct dependency. This satisfies Principle VII: `rayon`'s `par_iter` is the only ergonomic data-parallel iterator in stable Rust; the alternative (`std::thread` + manual scoped threads + channels) would require ~150 lines of infrastructure for equivalent behaviour. **Thread boundary rule**: `rayon` operates on its own thread pool entirely outside Tauri's Tokio runtime. This is safe as long as no `async`/`.await` or Tokio primitives are used inside `rayon` closures — a constraint that must be enforced in code review.
- The `log` crate (or a compatible façade) is used for structured logging; the concrete logger implementation is out of scope for this feature.
- Average MP3 bitrate is assumed to be 128 kbps when no bitrate information is available in the ID3 header and `TLEN` is absent. This assumption is documented in the code.
- The `id3` crate version `1.16.x` exposes `Tag::duration()` or the `TLEN` frame via `Tag::get("TLEN")`; if neither is available, the fallback estimation path applies.
- The TypeScript `Book` interface in `src/app/models/books.ts` MUST be updated in the same PR to add `duration_seconds: number | null` and `file_count: number | null`, per constitution Principle IV.

---

## Out of Scope

- UI display of `duration_seconds` or `file_count` (planned for Phase 2).
- Support for non-MP3 audio formats (M4B, FLAC, Opus) — tracked separately as a future improvement.
- New Tauri commands or changes to existing command signatures (including `force_rescan_command`, which is explicitly deferred to a follow-on feature).
- A dedicated scan-progress event stream (real-time progress reporting to the UI).
- Changes to the cover art or author photo discovery logic.
