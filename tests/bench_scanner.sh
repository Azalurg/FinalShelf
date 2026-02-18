#!/usr/bin/env bash
# =============================================================================
# bench_scanner.sh — Scanner performance benchmark
#
# Measures wall-clock time for the metadata-extraction workload against the
# 1,000-book × 10-file synthetic fixture, compares against a recorded baseline,
# and validates the SC-001 / SC-002 performance gates.
#
# Usage:
#   bash tests/bench_scanner.sh --record-baseline
#       Build bench_runner (serial, pre-refactor), run against fixture, and
#       write elapsed_ms to tests/bench_baseline.txt.  Must be run BEFORE
#       the Phase 3 rayon refactor.
#
#   bash tests/bench_scanner.sh
#       Build bench_runner (current code), run against fixture, compare result
#       against the stored baseline, and exit 1 if any SC gate fails.
#
# SC gates checked:
#   SC-002a  full-scan wall time ≤ 30,000 ms
#   SC-002b  current elapsed ≤ baseline / 2  (≥ 2× speedup over pre-refactor)
#
# Note: SC-001 (no-op scan ≤ 2 s) is validated by the unit test
#   test_no_op_scan_returns_zero_added in scanner.rs, not here.
# =============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FIXTURE_DIR="$REPO_ROOT/tests/fixtures/bench-library"
BASELINE_FILE="$REPO_ROOT/tests/bench_baseline.txt"
TAURI_DIR="$REPO_ROOT/src-tauri"
BENCH_BIN="$TAURI_DIR/target/release/bench_runner"

# ── Helpers ──────────────────────────────────────────────────────────────────

die() { echo "ERROR: $*" >&2; exit 1; }

check_fixture() {
    [[ -d "$FIXTURE_DIR" ]] || die \
        "Fixture directory not found: $FIXTURE_DIR
       Run: cd src-tauri && cargo run --features dev-fixtures --bin gen_fixtures"
}

build_bench_runner() {
    echo "Building bench_runner (release) ..."
    (cd "$TAURI_DIR" && \
        cargo build --features dev-fixtures --bin bench_runner --release --quiet)
    [[ -x "$BENCH_BIN" ]] || die "bench_runner binary not found after build: $BENCH_BIN"
}

run_bench() {
    "$BENCH_BIN" "$FIXTURE_DIR" | grep '^elapsed_ms=' | cut -d= -f2
}

print_stats() {
    echo ""
    echo "Benchmark run details:"
    "$BENCH_BIN" "$FIXTURE_DIR"
}

# ── Record baseline ───────────────────────────────────────────────────────────

if [[ "${1:-}" == "--record-baseline" ]]; then
    check_fixture
    build_bench_runner
    echo "Running serial pre-refactor scan ..."
    print_stats
    ELAPSED=$(run_bench)
    echo ""
    echo "$ELAPSED" > "$BASELINE_FILE"
    echo "Baseline recorded: ${ELAPSED} ms → $BASELINE_FILE"
    echo ""
    echo "IMPORTANT: This baseline must be committed to the branch."
    echo "           Do NOT re-record after the Phase 3 rayon refactor."
    exit 0
fi

# ── Compare against baseline ──────────────────────────────────────────────────

check_fixture
[[ -f "$BASELINE_FILE" ]] || die \
    "Baseline file not found: $BASELINE_FILE
   Run: bash tests/bench_scanner.sh --record-baseline  (on the pre-refactor code)"

BASELINE=$(tr -d '[:space:]' < "$BASELINE_FILE")

build_bench_runner
echo "Running post-refactor scan ..."
print_stats
ELAPSED=$(run_bench)

echo ""
echo "┌──────────────────────────────────────────┐"
echo "│  Benchmark Results                       │"
echo "├──────────────────────────────────────────┤"
printf "│  Baseline (pre-refactor) : %8s ms     │\n" "$BASELINE"
printf "│  Current  (post-refactor): %8s ms     │\n" "$ELAPSED"
THRESHOLD=$(( BASELINE / 2 ))
printf "│  2× threshold            : %8s ms     │\n" "$THRESHOLD"
echo "└──────────────────────────────────────────┘"
echo ""

PASS=true

# SC-002a: full scan ≤ 30 s
if [[ "$ELAPSED" -le 30000 ]]; then
    echo "✓ SC-002a  wall time  : ${ELAPSED}ms ≤ 30,000ms"
else
    echo "✗ SC-002a  wall time  : ${ELAPSED}ms > 30,000ms  [FAIL]"
    PASS=false
fi

# SC-002b: ≥ 2× faster than baseline
if [[ "$ELAPSED" -le "$THRESHOLD" ]]; then
    echo "✓ SC-002b  speedup    : ${ELAPSED}ms ≤ baseline/2 (${THRESHOLD}ms) — ≥2× faster"
else
    echo "✗ SC-002b  speedup    : ${ELAPSED}ms > baseline/2 (${THRESHOLD}ms)  [FAIL]"
    PASS=false
fi

echo ""
if [[ "$PASS" == "true" ]]; then
    echo "All benchmark gates PASSED ✓"
    exit 0
else
    echo "Benchmark FAILED ✗ — performance regression detected"
    exit 1
fi
