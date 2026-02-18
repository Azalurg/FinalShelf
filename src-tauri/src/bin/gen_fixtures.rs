//! Benchmark fixture generator.
//!
//! Generates 1,000 book directories × 10 minimal ID3v2 MP3 stub files.
//! Output: `<repo_root>/tests/fixtures/bench-library/`
//!
//! Each file contains only an ID3v2.3 tag (no audio data); this is sufficient
//! for scanner benchmarks that only read metadata.
//!
//! Tag fields written per file:
//!   TALB  → "Book XXXX"     (album title — used as book identifier)
//!   TPE2  → "Author XXXX"   (album artist)
//!   TPE1  → "Narrator XXXX" (track artist / lector)
//!   TLEN  → "180000"        (duration in ms = 3 min; used by TLEN extraction path)
//!   TRCK  → track number
//!
//! Usage (from repo root or src-tauri/):
//!   cargo run --features dev-fixtures --bin gen_fixtures

use std::path::Path;

use id3::{Frame, Tag, TagLike, Version};

fn main() {
    // CARGO_MANIFEST_DIR is set at compile time to the directory containing Cargo.toml
    // (i.e. src-tauri/). Its parent is the repo root.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let base = Path::new(manifest_dir)
        .parent()
        .expect("CARGO_MANIFEST_DIR has no parent directory")
        .join("tests/fixtures/bench-library");

    std::fs::create_dir_all(&base).unwrap_or_else(|e| panic!("Failed to create bench-library at {:?}: {}", base, e));

    println!("Generating fixtures at: {}", base.display());

    for book_i in 0..1000_usize {
        let book_dir = base.join(format!("book_{:04}", book_i + 1));
        std::fs::create_dir_all(&book_dir)
            .unwrap_or_else(|e| panic!("Failed to create book dir {:?}: {}", book_dir, e));

        for track_i in 0..10_usize {
            let file_path = book_dir.join(format!("track_{:02}.mp3", track_i + 1));

            // Create an empty stub file first; write_to_path will prepend the ID3 tag.
            std::fs::write(&file_path, b"").unwrap_or_else(|e| panic!("Failed to create stub {:?}: {}", file_path, e));

            let mut tag = Tag::new();
            // TALB — album title (book identifier)
            tag.set_album(format!("Book {:04}", book_i + 1));
            // TPE2 — album artist
            tag.add_frame(Frame::text("TPE2", format!("Author {:04}", book_i + 1)));
            // TPE1 — track artist / lector
            tag.set_artist(format!("Narrator {:04}", book_i + 1));
            // TLEN — duration in milliseconds (3 min = 180 000 ms)
            // tag.duration() in id3 1.16.x reads this frame and returns Option<u32> ms.
            tag.add_frame(Frame::text("TLEN", "180000"));
            // TRCK — track number
            tag.set_track((track_i + 1) as u32);

            tag.write_to_path(&file_path, Version::Id3v23)
                .unwrap_or_else(|e| panic!("Failed to write tag to {:?}: {}", file_path, e));
        }

        if (book_i + 1) % 100 == 0 {
            println!("  {}/1000 books generated...", book_i + 1);
        }
    }

    println!(
        "Done. Generated 1,000 × 10 = 10,000 MP3 stubs at:\n  {}",
        base.display()
    );
}
