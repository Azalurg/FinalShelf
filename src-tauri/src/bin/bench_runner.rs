//! Pre-refactor scan baseline benchmark.
//!
//! Walks the fixture directory and reads ALL ID3 tags serially — one per MP3 file.
//! This represents the full metadata-extraction workload (TLEN from every file)
//! that the refactored scanner performs. After the Phase 3 refactor, this binary
//! will be updated to use `rayon::par_iter` for the same workload; comparing the
//! two timings demonstrates the ≥2× speedup required by SC-002.
//!
//! Why read all tags (not just one per directory)?
//!   The pre-refactor scanner reads only the first MP3 tag per directory.
//!   The refactored scanner reads all tags for TLEN-based duration extraction.
//!   To measure the parallelism speedup fairly, the baseline must reflect the
//!   same workload: 10,000 tag reads for a 1,000-book × 10-file fixture.
//!
//! Usage:
//!   bench_runner <scan_directory>
//!
//! Output (one key=value per line, consumed by bench_scanner.sh):
//!   elapsed_ms=<wall-clock ms>
//!   dirs_processed=<unique parent directories encountered>
//!   files_read=<total ID3 reads attempted>

use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Instant;

use id3::Tag;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = match args.get(1) {
        Some(d) => d.clone(),
        None => {
            eprintln!("Usage: bench_runner <scan_directory>");
            std::process::exit(1);
        },
    };

    if !std::path::Path::new(&dir).exists() {
        eprintln!("ERROR: directory not found: {}", dir);
        std::process::exit(1);
    }

    let start = Instant::now();

    let mut seen_dirs: HashSet<PathBuf> = HashSet::new();
    let mut files_read: usize = 0;

    for entry in WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("mp3") {
            // Read every MP3 tag (simulates TLEN extraction across all files per directory).
            let _ = Tag::read_from_path(path);
            files_read += 1;

            if let Some(parent) = path.parent() {
                seen_dirs.insert(parent.to_path_buf());
            }
        }
    }

    let elapsed_ms = start.elapsed().as_millis();

    // Output consumed by bench_scanner.sh — one key=value per line.
    println!("elapsed_ms={}", elapsed_ms);
    println!("dirs_processed={}", seen_dirs.len());
    println!("files_read={}", files_read);
}
