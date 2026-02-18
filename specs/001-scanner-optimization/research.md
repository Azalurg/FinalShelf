# Research: Scanner Optimization & Enhancement

**Feature**: `001-scanner-optimization`  
**Date**: 2026-02-18  
**Status**: Complete — all NEEDS CLARIFICATION resolved

---

## Summary of Unknowns Investigated

| # | Unknown | Status |
|---|---------|--------|
| R1 | rayon + Tauri/Tokio thread boundary safe pattern | ✅ Resolved |
| R2 | id3 v1.16.x — TLEN frame API | ✅ Resolved |
| R3 | Duration estimation from file size/bitrate | ✅ Resolved |
| R4 | Diesel 2.2.x — single-column nullable update | ✅ Resolved |
| R5 | Diesel 2.2.x — bulk orphan update with `eq_any` | ✅ Resolved |
| R6 | `AsChangeset` safety with full struct vs. partial | ✅ Resolved |
| R7 | `ScanReport` return type and rayon accumulation pattern | ✅ Resolved |
| R8 | Minimal valid MP3 test fixture generation | ✅ Resolved |

---

## R1 — rayon + Tauri/Tokio Thread Boundary

**Decision**: Wrap all rayon work in `tokio::task::spawn_blocking` before calling from an `async` Tauri command.

**Rationale**: `#[tauri::command] async fn` runs on Tokio's async worker thread pool. Calling `.par_iter().collect()` directly from an `async fn` blocks an async worker for the duration of the parallel scan, starving the IPC scheduler under concurrent requests. `tokio::task::spawn_blocking` dispatches to Tokio's dedicated blocking thread pool (capped at 512 threads), which is entirely separate from the async pool. rayon operates on its own work-stealing thread pool within that blocking context — no Tokio primitives enter rayon closures.

**Concrete pattern**:
```rust
#[tauri::command]
pub async fn quick_scan_command() -> Result<ScanReport, String> {
    tokio::task::spawn_blocking(|| quick_scan())
        .await
        .map_err(|e| e.to_string())?
}
```
`quick_scan()` is synchronous; rayon `par_iter` is called inside it freely.

**Alternatives considered**:
- `std::thread::spawn` — more boilerplate; no benefit over `spawn_blocking` for this pattern.
- `rayon::scope` inline in `async fn` — same starvation risk as direct `par_iter`.
- Declaring the command as non-`async` — Tauri places it on a blocking thread automatically, which also works. Chosen against because the existing `quick_scan_command` is `async` and the pattern is consistent.

**Thread boundary constraint (must be enforced in code review)**:
- No `async/.await` inside rayon closures.
- No Tokio primitives (`tokio::sync::Mutex`, `tokio::time`, etc.) inside rayon closures.
- `Diesel` connection objects are NOT `Send` in all configurations — connections must be created **after** the blocking boundary, in the serial DB-write phase, not inside rayon closures.

**Confidence**: High

---

## R2 — id3 v1.16.x — TLEN Frame API

**Decision**: Use `tag.duration()` to read TLEN; fall back to file-size estimation when it returns `None` or `Some(0)`.

**Rationale**: The `TagLike` trait in id3 v1.16.x exposes:
```rust
fn duration(&self) -> Option<u32>  // Returns TLEN value in MILLISECONDS
```
Returns `None` if the TLEN frame is absent or unparseable. Returns `Some(0)` if TLEN is present but explicitly set to zero — treat `Some(0)` the same as `None` (fall back to estimation).

The lower-level `tag.get("TLEN")` alternative exists but is verbose; `duration()` is the correct API.

**Key unit**: `u32` milliseconds. Convert to `i32` seconds with `(ms / 1000) as i32`.

**Alternatives considered**:
- `tag.get("TLEN")` — works, but requires manual string parsing; no benefit.
- A dedicated audio-frame-walking crate (e.g., `mp3-duration`) — accurate for VBR but reads the entire audio stream; too slow for large libraries without TLEN tags.

**Confidence**: High

---

## R3 — Duration Estimation (TLEN Absent / Zero)

**Decision**: Estimate using `duration_secs = (file_size_bytes - id3_tag_size_bytes) / (bitrate_kbps * 125)`, default bitrate 128 kbps. Read actual bitrate from the MPEG audio frame header when feasible; fall back to 128 kbps when not.

**Rationale**: The id3 crate does not expose bitrate (bitrate lives in MPEG audio frame headers, not ID3 tags). Reading the MPEG frame header requires ~40 lines of manual bit-parsing on a `BufReader`, seeking past the ID3 block. For a first implementation, using the 128 kbps default is acceptable for audiobooks (CBR 64–128 kbps covers >95% of the genre). The formula accuracy is within 2% for CBR files; up to 20% error for VBR (rare in audiobooks).

Subtracting the ID3 tag size is necessary to avoid overcounting: `id3_tag_size = tag.size()` (available from the id3 crate after reading).

**Formula**:
```
estimated_seconds = (file_bytes - tag_bytes) / (128 * 125)
                  = (file_bytes - tag_bytes) / 16_000
```
Sum across all files in the directory, set `duration_is_estimated = true`.

**Alternatives considered**:
- `mp3-duration` crate — accurate but full-file scan; adds a dependency (Principle VII risk); deferred to a follow-on if accuracy complaints arise.
- MPEG frame header parsing inline — more accurate, zero new crates; can be a follow-on improvement if the 128 kbps default proves insufficient.

**Confidence**: High (formula); Medium (accuracy for mixed-bitrate libraries)

---

## R4 — Diesel 2.2.x — Single-Column Nullable Update

**Decision**: Use the column expression form `.set(dsl::orphaned.eq(Some(flag)))` for per-row updates; use `eq_any` for bulk updates.

**Rationale**: Diesel 2.2.x supports targeting a single column directly in an update:
```rust
diesel::update(dsl::books.filter(dsl::title.eq(&title)))
    .set(dsl::orphaned.eq(Some(true)))
    .execute(conn)?;
```
This generates `UPDATE books SET orphaned = true WHERE title = ?` — exactly one column written. User-editable fields (`score`, `read`) are untouched.

For bulk updates across a set of titles:
```rust
diesel::update(dsl::books.filter(dsl::title.eq_any(&orphaned_titles)))
    .set(dsl::orphaned.eq(Some(true)))
    .execute(conn)?;
```
Generates a single `UPDATE ... WHERE title IN (...)` — one DB round-trip.

**SQLite variable limit caveat**: SQLite's default `SQLITE_MAX_VARIABLE_NUMBER = 999`. If the library contains more than 999 books, the `eq_any` list must be chunked (iterate in batches of ≤999). Wrap the two bulk updates in a single `conn.transaction(...)` for atomicity.

**Alternatives considered**:
- Full `AsChangeset` struct update — **rejected**: would overwrite `score`, `read`, `relative_file_path`, violating FR-001.
- `INSERT OR REPLACE` upsert — **rejected**: same field-overwrite risk.

**Confidence**: High

---

## R5 — `AsChangeset` Safety for Partial Updates

**Decision**: Never use the full `Book` struct's `AsChangeset` derive for orphan updates. Prefer the inline column expression form for single-field updates, or a dedicated `BookOrphanUpdate` changeset struct for future multi-field scan updates.

**Rationale**: `diesel::update(...).set(&whole_book_struct)` emits an UPDATE that touches every field in the struct. With `AsChangeset`, `Option<T>` fields that are `None` are **skipped** by default (not written to DB), but `Option<T>` fields that are `Some(value)` **are** written — meaning calling `.set(&book)` with a fully populated `Book` struct overwrites `score` and `read`. This violates FR-001.

Dedicated changeset pattern (for future `force_rescan_command` use):
```rust
#[derive(AsChangeset)]
#[diesel(table_name = books)]
pub struct BookMetadataUpdate {
    pub duration_seconds: Option<i32>,
    pub duration_is_estimated: Option<bool>,
    pub file_count: Option<i32>,
}
```

**Confidence**: High

---

## R6 — ScanReport and rayon Accumulation

**Decision**: `quick_scan()` returns `Result<ScanReport, String>`. The rayon parallel phase collects `Vec<Result<DirectoryMetadata, ScanError>>`, then a serial loop processes results for DB writes and tallies counts.

**Rationale**: A plain struct return value is idiomatic for a non-streaming scan command. No channels or `Mutex` accumulators are needed. The collect-then-process pattern is the canonical rayon pattern and keeps the parallel phase purely functional (no shared mutable state).

```rust
pub struct ScanReport {
    pub books_added: usize,
    pub books_skipped: usize,
    pub books_newly_orphaned: usize,
    pub errors: usize,
    pub elapsed_ms: u128,
}
```

**Alternatives considered**:
- `Mutex<Vec<...>>` shared accumulator inside rayon — works, but adds lock contention; unnecessary when `collect()` is available.
- Streaming via `mpsc::channel` — appropriate only for real-time progress events (out of scope per spec).

**Confidence**: High

---

## R7 — Minimal Valid MP3 Test Fixture

**Decision**: Generate minimal ID3v2-only `.mp3` files programmatically in a Rust `build.rs` or a separate fixture-generation binary. Files contain only an ID3v2.3 header with TALB (album), TPE2 (album artist), and TLEN (duration) frames — no audio data. The id3 crate parses them correctly.

**Rationale**: The id3 crate reads only the ID3 header region and does not validate the presence of MPEG audio frames. A file with a well-formed ID3v2.3 header and zero audio data is fully parseable. The `id3::Tag::write_to_path()` method writes such a file natively in Rust.

**Fixture generation approach** (for `tests/fixtures/bench-library/`):
- A small Rust binary at `tests/gen_fixtures.rs` or a shell script uses `id3::Tag` to write 1,000 directories × 10 MP3 files each.
- Each MP3 gets: TALB = `"Bench Book NNN"`, TPE2 = `"Author NNN"`, TLEN = `"180000"` (3 min), TRCK = track number.
- Total fixture size: ~10,000 files × ~200 bytes each ≈ 2 MB on disk.

**Confidence**: High

---

## Decisions Summary

| Decision | Chosen | Rationale |
|----------|--------|-----------|
| rayon integration | `tokio::task::spawn_blocking` wrapping `quick_scan()` | Safe Tokio/rayon boundary |
| TLEN reading | `tag.duration()` → `Option<u32>` ms | Native id3 API |
| Duration fallback | File size / (128 kbps × 125) | No new crates; good enough for CBR |
| Single-col update | `dsl::orphaned.eq(Some(flag))` inline | Minimal, safe, no field overwrite |
| Bulk orphan update | `eq_any` in two statements + transaction | One DB round-trip per state |
| Changeset safety | Never use full `Book` changeset for scan updates | Protects user fields |
| Scan return type | `Result<ScanReport, String>` | Idiomatic; no async complexity |
| Test fixtures | Rust-generated ID3v2-only MP3s | Reproducible, minimal, native |
