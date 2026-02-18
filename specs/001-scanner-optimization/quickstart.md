# Quickstart: Scanner Optimization & Enhancement

**Feature**: `001-scanner-optimization`  
**Branch**: `001-scanner-optimization`  
**Prerequisites**: Rust toolchain (2021 edition), `cargo`, `diesel_cli` (SQLite feature), Node.js ≥ 18, `npm`

---

## 1. Environment Setup

```bash
# Verify you are on the correct branch
git checkout 001-scanner-optimization

# Install Diesel CLI if not already present (SQLite only — avoids requiring libpq/libmysql)
cargo install diesel_cli --no-default-features --features sqlite

# Install frontend dependencies
npm install

# Copy example environment file and configure your library path
cp src-tauri/example_env src-tauri/.env
# Edit src-tauri/.env — set DATABASE_URL to an absolute path for your dev SQLite file
# e.g.: DATABASE_URL=/home/user/.config/finalshelf/dev.db
```

---

## 2. Run the Database Migration

The scanner-optimization migration adds four columns to the `books` table (`duration_seconds`, `duration_is_estimated`, `file_count`, `orphaned`).

```bash
cd src-tauri
diesel migration run
# Expected output:
#   Running migration 2026-02-18-000000_scanner_fields
```

Verify the schema regenerated correctly:

```bash
# schema.rs should now include the four new columns in the books table
grep -A 15 "books (title)" src/schema.rs
```

To roll back:

```bash
diesel migration revert
```

---

## 3. Add the `rayon` Dependency

```bash
cd src-tauri
cargo add rayon
# Verify it was added to Cargo.toml under [dependencies]
grep rayon Cargo.toml
```

---

## 4. Build & Run in Development

```bash
# From repo root — starts both the Angular dev server and the Tauri window
npm run tauri dev
```

To test the scan specifically:

1. Open the app → Settings → select a library root
2. Click "Scan" — the console will show the new `ScanReport` fields
3. Check the database: `sqlite3 <your-db-path> "SELECT title, duration_seconds, file_count, orphaned FROM books LIMIT 10;"`

---

## 5. Run Rust Unit Tests

```bash
cd src-tauri
cargo test
# To run only scanner tests:
cargo test scanner
# To run with output visible:
cargo test scanner -- --nocapture
```

Expected test coverage target: ≥ 80% line coverage on new functions in `scanner.rs` (SC-006).

To check coverage with `cargo-tarpaulin`:

```bash
cargo install cargo-tarpaulin  # one-time install
cargo tarpaulin --include-files src/scanner.rs --out Stdout
```

---

## 6. Lint & Format Checks

These must pass before any PR (SC-007):

```bash
cd src-tauri
cargo fmt --check       # zero formatting deviations
cargo clippy -- -D warnings  # zero warnings

# Frontend
cd ..
npm run lint            # ESLint + angular-eslint
```

---

## 7. Generate the Benchmark Fixture

The benchmark fixture (`tests/fixtures/bench-library/`) contains 1,000 simulated book directories, each with 10 minimal valid MP3 files. It must be generated once and committed to the repo.

```bash
cd src-tauri
# Run the fixture generator binary (to be created as part of implementation)
cargo run --bin gen_fixtures -- --output ../tests/fixtures/bench-library --books 1000 --files-per-book 10
# Expected output: "Generated 10000 fixture files in tests/fixtures/bench-library/"
# Approximate disk size: ~2 MB
```

Verify the fixture:

```bash
find tests/fixtures/bench-library -name "*.mp3" | wc -l
# Expected: 10000
```

---

## 8. Run the Performance Benchmark

```bash
# From repo root
bash tests/bench_scanner.sh
# This script:
#   1. Ensures tests/fixtures/bench-library/ exists (runs gen_fixtures if needed)
#   2. Runs quick_scan against the fixture library using a fresh in-memory DB
#   3. Prints wall-clock time
#   4. Compares against tests/bench_baseline.txt (if present)
#   5. Fails if result is worse than the baseline (i.e., regression)
```

To record a new baseline after confirming the implementation is correct:

```bash
bash tests/bench_scanner.sh --record-baseline
# Writes elapsed time to tests/bench_baseline.txt
```

**Target**: ≤ 30 s for 10,000 files; ≥ 2× faster than pre-refactor baseline (SC-002).

---

## 9. Verify Incremental Scan Behaviour

```bash
# 1. Run a full scan once to populate the DB
# 2. Run scan again with no changes
# 3. Confirm zero books added and completion in < 2 s (SC-001)

sqlite3 <your-db-path> "SELECT COUNT(*) FROM books;"  # note count
# trigger second scan via UI or direct Tauri invoke
sqlite3 <your-db-path> "SELECT COUNT(*) FROM books;"  # count must be identical
```

---

## 10. Verify Orphan Detection

```bash
# 1. After a scan, note a book's title
# 2. Rename or delete its directory on disk
# 3. Run scan again
# 4. Query the DB:
sqlite3 <your-db-path> "SELECT title, orphaned FROM books WHERE orphaned = 1;"
# The moved/deleted book should appear with orphaned = 1
```

---

## File Locations Reference

| Purpose | Path |
|---------|------|
| Scanner source | `src-tauri/src/scanner.rs` |
| Book model | `src-tauri/src/models/book.rs` |
| Books service | `src-tauri/src/services/books_service.rs` |
| DB schema (auto-gen) | `src-tauri/src/schema.rs` |
| Migration directory | `src-tauri/migrations/` |
| TypeScript Book interface | `src/app/models/books.ts` |
| Benchmark script | `tests/bench_scanner.sh` |
| Benchmark baseline | `tests/bench_baseline.txt` |
| Fixture generator | `src-tauri/src/bin/gen_fixtures.rs` |
| Fixture library | `tests/fixtures/bench-library/` |
