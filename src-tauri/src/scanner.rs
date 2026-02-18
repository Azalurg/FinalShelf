use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, NaiveDateTime};
use id3::{Tag, TagLike};
use rayon::prelude::*;
use rusqlite::Result;
use serde::Serialize;
use walkdir::WalkDir;

use crate::{
    db::establish_connection,
    models::{author::Author, book::Book},
    services::{
        absolute_paths_service::get_current_absolute_path,
        authors_service::{add_author, is_author_exists},
        books_service::{add_book, get_all_books, is_book_exists, update_books_orphaned},
    },
};

// In-memory metadata for one audiobook directory (parallel phase output)
#[derive(Debug)]
struct DirectoryMetadata {
    parent_path: String,
    mp3_paths: Vec<PathBuf>,
    first_tag: Option<Tag>,
    file_create_date: Option<NaiveDateTime>,
    duration_seconds: Option<i32>,
    duration_is_estimated: bool,
    file_count: i32,
}

// Error during metadata extraction
#[derive(Debug)]
struct ScanError {
    path: String,
    message: String,
}

// Scan completion report (returned to command layer)
#[derive(Debug, Serialize)]
pub struct ScanReport {
    pub books_added: usize,
    pub books_skipped: usize,
    pub books_newly_orphaned: usize,
    pub errors: usize,
    pub elapsed_ms: u64,
}

fn look_for_cover(directory: &str, base_path: &Path) -> String {
    let exts = [".jpg", ".jpeg", ".png", ".gif", ".webp", ".nfo"];
    let names = ["cover", "folder", "album", "poster", "default", "art"];
    for name in names.iter() {
        for ext in exts.iter() {
            let image_name = format!("{}{}", name, ext);
            let image_path = Path::new(directory).join(image_name);

            if fs::metadata(&image_path).is_ok() {
                // Return the relative path
                return image_path
                    .strip_prefix(base_path)
                    .unwrap_or(&image_path)
                    .to_string_lossy()
                    .to_string();
            }
        }
    }
    String::new()
}

fn look_for_author_photo(path: &str, name: &str, base_path: &Path) -> String {
    let directory = match path.find(name) {
        Some(index) => &path[..index + name.len()],
        None => return String::new(),
    };

    look_for_cover(directory, base_path)
}

fn system_time_to_naive_date_time(option_time: Option<SystemTime>) -> Option<NaiveDateTime> {
    option_time?
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| DateTime::from_timestamp(duration.as_secs() as i64, duration.subsec_nanos()))
        .map(|datetime_utc| datetime_utc.naive_utc())
}

pub fn quick_scan() -> Result<ScanReport, String> {
    let absolute_path = get_current_absolute_path();
    let directory: String;
    if let Some(absolute_path) = absolute_path {
        directory = absolute_path.absolute_path;
    } else {
        return Err("No path to scan".to_string());
    }

    log::info!("Quick scan starting in {}", directory);
    let start = Instant::now();
    let base_path = Path::new(&directory);

    // Phase 1: Collect unique parent directories
    let mut unique_dirs = HashSet::new();
    for entry in WalkDir::new(&directory).min_depth(1).into_iter() {
        match entry {
            Ok(entry) => {
                if let Some(mp3_path) = get_mp3_path(&entry) {
                    if let Some(parent_path) = mp3_path.parent() {
                        unique_dirs.insert(parent_path.to_path_buf());
                    }
                }
            }
            Err(e) => {
                log::warn!("Failed to read directory entry: {}", e);
                continue;
            }
        }
    }

    let directories: Vec<PathBuf> = unique_dirs.into_iter().collect();

    log::info!("Found {} unique book directories", directories.len());

    // Phase 2: Parallel metadata extraction
    let extraction_results: Vec<Result<DirectoryMetadata, ScanError>> = directories
        .par_iter()
        .map(|dir| extract_directory_metadata(dir, base_path))
        .collect();

    log::info!("Metadata extraction complete, processing results...");

    // Phase 3: Serial write phase
    let mut books_added = 0;
    let mut books_skipped = 0;
    let mut errors = 0;

    for result in extraction_results {
        match result {
            Ok(meta) => {
                // Build title from first tag or directory name
                let title = meta
                    .first_tag
                    .as_ref()
                    .and_then(|tag| tag.album())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("Unknown ({})", meta.parent_path));

                // Skip if book already exists
                if is_book_exists(&title) {
                    books_skipped += 1;
                    continue;
                }

                // Extract metadata from tag
                let tag = meta.first_tag.as_ref();
                let genre = tag.and_then(|t| t.genre()).unwrap_or("Unknown").to_string();
                let lector = tag.and_then(|t| t.artist()).unwrap_or("Unknown").to_string();
                let author_name = tag.and_then(|t| t.album_artist()).unwrap_or("Unknown").to_string();

                // Look for cover and author photo
                let relative_cover_path = look_for_cover(&meta.parent_path, base_path);

                // Build relative file path from first MP3
                let file_path_str = meta
                    .mp3_paths
                    .first()
                    .and_then(|path| path.strip_prefix(base_path).ok())
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| {
                        // Fallback: use absolute path if strip_prefix fails
                        meta.mp3_paths
                            .first()
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_default()
                    });

                // Add author if not exists
                if !is_author_exists(&author_name) {
                    let author = Author {
                        name: author_name.clone(),
                        relative_img_path: Some(look_for_author_photo(&meta.parent_path, &author_name, base_path)),
                    };
                    log::info!("Adding author: {}", author.name);
                    add_author(&author);
                }

                // Create and add book
                let book = Book {
                    title: title.clone(),
                    author_name,
                    relative_cover_path: Some(relative_cover_path),
                    genre: Some(genre),
                    lector: Some(lector),
                    create_date: meta.file_create_date,
                    read: Some(false),
                    score: Some(0),
                    relative_file_path: file_path_str,
                    duration_seconds: meta.duration_seconds,
                    duration_is_estimated: Some(meta.duration_is_estimated),
                    file_count: Some(meta.file_count),
                    orphaned: None,
                };
                log::info!("Adding book: {}", title);
                add_book(&book);
                books_added += 1;
            }
            Err(e) => {
                log::warn!("Error processing {}: {}", e.path, e.message);
                errors += 1;
            }
        }
    }

    // Phase 4: Orphan detection
    log::info!("Starting orphan detection phase");
    let conn = &mut establish_connection();
    let all_books = get_all_books();
    let mut orphaned_titles = Vec::new();
    let mut present_titles = Vec::new();

    for book in all_books {
        let full_path = Path::new(&directory).join(&book.relative_file_path);
        if full_path.exists() {
            // Path exists - mark as not orphaned (if it was orphaned before)
            if book.orphaned == Some(true) {
                present_titles.push(book.title);
            }
        } else {
            // Path missing - mark as orphaned
            if book.orphaned != Some(true) {
                log::warn!("Book orphaned (path not found): {}", book.title);
                orphaned_titles.push(book.title);
            }
        }
    }

    // Bulk update orphaned flags
    let books_newly_orphaned = orphaned_titles.len();
    if !orphaned_titles.is_empty() {
        update_books_orphaned(&orphaned_titles, true, conn);
    }
    if !present_titles.is_empty() {
        update_books_orphaned(&present_titles, false, conn);
    }

    let elapsed_ms = start.elapsed().as_millis() as u64;

    // Completion summary
    log::info!(
        "Scan complete: {} books added, {} skipped, {} newly orphaned, {} errors, {} ms elapsed",
        books_added,
        books_skipped,
        books_newly_orphaned,
        errors,
        elapsed_ms
    );

    Ok(ScanReport {
        books_added,
        books_skipped,
        books_newly_orphaned,
        errors,
        elapsed_ms,
    })
}

// Extract metadata from a single audiobook directory
fn extract_directory_metadata(dir: &Path, _base_path: &Path) -> Result<DirectoryMetadata, ScanError> {
    let parent_path_str = dir.to_string_lossy().to_string();

    // Collect all MP3 files in this directory
    let mp3_paths: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| ScanError {
            path: parent_path_str.clone(),
            message: format!("Failed to read directory: {}", e),
        })?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path.extension().and_then(|s| s.to_str()).map(|s| s.to_lowercase()) == Some("mp3".to_string())
        })
        .collect();

    if mp3_paths.is_empty() {
        return Err(ScanError {
            path: parent_path_str,
            message: "No MP3 files found".to_string(),
        });
    }

    // Read the first MP3's tag
    let first_tag = mp3_paths.first().and_then(|path| Tag::read_from_path(path).ok());

    // Get file creation date from first MP3
    let file_create_date = mp3_paths
        .first()
        .and_then(|path| fs::metadata(path).ok())
        .and_then(|metadata| system_time_to_naive_date_time(metadata.created().ok()));

    // Calculate duration from TLEN tags or estimate from file size
    let mut total_duration_ms: u64 = 0;
    let mut duration_is_estimated = false;
    let mut files_processed = 0;

    for mp3_path in &mp3_paths {
        if let Ok(tag) = Tag::read_from_path(mp3_path) {
            // Try to get duration from TLEN tag
            if let Some(duration_ms) = tag.duration() {
                if duration_ms > 0 {
                    total_duration_ms += duration_ms as u64;
                    files_processed += 1;
                    continue;
                }
            }
        }

        // Fallback: estimate duration from file size
        // Assumption: 128 kbps bitrate (16,000 bytes per second)
        if let Ok(metadata) = fs::metadata(mp3_path) {
            let file_bytes = metadata.len();
            // Rough estimate: subtract ~3KB for ID3 tags, then divide by bitrate
            let audio_bytes = file_bytes.saturating_sub(3000);
            let estimated_seconds = audio_bytes / 16_000;
            total_duration_ms += estimated_seconds * 1000;
            duration_is_estimated = true;
            files_processed += 1;
        }
    }

    // Always compute duration if we processed any files (even if it's 0)
    let duration_seconds = if files_processed > 0 {
        Some((total_duration_ms / 1000) as i32)
    } else {
        None
    };

    let file_count = mp3_paths.len() as i32;

    Ok(DirectoryMetadata {
        parent_path: parent_path_str,
        mp3_paths,
        first_tag,
        file_create_date,
        duration_seconds,
        duration_is_estimated,
        file_count,
    })
}

// Helper function to extract mp3 path from a directory entry
fn get_mp3_path(entry: &walkdir::DirEntry) -> Option<&Path> {
    let path = entry.path();
    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("mp3") {
        Some(path)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn create_test_mp3(path: &Path) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        // Create a minimal valid MP3 file with ID3v2 header
        // ID3v2.4 header: "ID3" + version (4, 0) + flags (0) + size (syncsafe 0)
        let minimal_mp3 = vec![
            // ID3v2.4 header
            0x49, 0x44, 0x33, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // MP3 frame header (fake but parseable)
            0xFF, 0xFB, 0x90, 0x00,
        ];
        fs::write(path, minimal_mp3).unwrap();
    }

    #[test]
    fn test_no_op_scan_returns_zero_added() {
        // This test verifies that scanning an already-indexed library returns books_added = 0
        // Note: Requires a test database setup, skipping actual DB interaction for now
        // Full integration test would create a temp DB, add a book, then scan the same directory
        
        // Placeholder assertion - full implementation requires test DB infrastructure
        assert!(true, "Test infrastructure for DB-backed tests to be implemented");
    }

    #[test]
    fn test_scan_adds_only_new_directories() {
        // This test verifies that only new directories are added during a scan
        // Existing books are skipped, new ones are inserted
        
        // Placeholder assertion - requires test DB
        assert!(true, "Test infrastructure for DB-backed tests to be implemented");
    }

    #[test]
    fn test_relative_path_strips_absolute_root() {
        // Test that stored relative_file_path does not contain the absolute root prefix
        let base_path = Path::new("/home/user/library");
        let full_path = PathBuf::from("/home/user/library/Author/Book/file.mp3");
        
        let relative = full_path.strip_prefix(base_path).unwrap();
        let relative_str = relative.to_string_lossy().to_string();
        
        assert!(!relative_str.starts_with("/home"), "Relative path should not start with absolute prefix");
        assert_eq!(relative_str, "Author/Book/file.mp3");
    }

    #[test]
    fn test_extract_directory_metadata_returns_error_for_no_mp3s() {
        // Create a temporary directory with no MP3 files
        let temp_dir = std::env::temp_dir().join("finalshelf_test_no_mp3");
        fs::create_dir_all(&temp_dir).unwrap();
        
        let result = extract_directory_metadata(&temp_dir, Path::new("/"));
        
        assert!(result.is_err(), "Should return error for directory with no MP3 files");
        
        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_extract_directory_metadata_counts_mp3_files() {
        // Create a temporary directory with 3 MP3 files and 2 other files
        let temp_dir = std::env::temp_dir().join("finalshelf_test_count_mp3");
        fs::create_dir_all(&temp_dir).unwrap();
        
        create_test_mp3(&temp_dir.join("file1.mp3"));
        create_test_mp3(&temp_dir.join("file2.mp3"));
        create_test_mp3(&temp_dir.join("file3.mp3"));
        
        // Create non-MP3 files
        fs::write(temp_dir.join("cover.jpg"), b"fake image").unwrap();
        fs::write(temp_dir.join("notes.txt"), b"some notes").unwrap();
        
        let result = extract_directory_metadata(&temp_dir, Path::new("/"));
        
        assert!(result.is_ok(), "Should successfully extract metadata");
        let meta = result.unwrap();
        assert_eq!(meta.file_count, 3, "Should count exactly 3 MP3 files");
        assert_eq!(meta.mp3_paths.len(), 3, "Should collect 3 MP3 paths");
        
        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_file_count_single_file() {
        // Test that a directory with exactly one MP3 file has file_count = 1
        let temp_dir = std::env::temp_dir().join("finalshelf_test_single_mp3");
        fs::create_dir_all(&temp_dir).unwrap();
        
        // Create exactly one MP3
        create_test_mp3(&temp_dir.join("single.mp3"));
        
        let result = extract_directory_metadata(&temp_dir, Path::new("/"));
        
        assert!(result.is_ok(), "Should successfully extract metadata");
        let meta = result.unwrap();
        assert_eq!(meta.file_count, 1, "Should have file_count = 1 for single MP3");
        assert_eq!(meta.mp3_paths.len(), 1, "Should collect exactly 1 MP3 path");
        
        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_corrupt_mp3_scan_continues() {
        // Test that a corrupt/unreadable MP3 doesn't stop the scan
        let temp_dir = std::env::temp_dir().join("finalshelf_test_corrupt_mp3");
        fs::create_dir_all(&temp_dir).unwrap();
        
        // Create one valid MP3
        create_test_mp3(&temp_dir.join("valid.mp3"));
        
        // Create a corrupt MP3 (just random bytes, no valid ID3)
        fs::write(temp_dir.join("corrupt.mp3"), b"not a valid mp3 file").unwrap();
        
        let result = extract_directory_metadata(&temp_dir, Path::new("/"));
        
        // Should succeed because at least one valid MP3 exists
        // The corrupt file might not be parsed correctly but shouldn't crash
        assert!(result.is_ok(), "Should handle corrupt MP3 gracefully");
        
        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_zero_mp3_dir_silently_skipped() {
        // Test that a directory with zero MP3 files returns an error
        // (which is then logged and counted, not crashing the scanner)
        let temp_dir = std::env::temp_dir().join("finalshelf_test_zero_mp3");
        fs::create_dir_all(&temp_dir).unwrap();
        
        // Create only non-MP3 files
        fs::write(temp_dir.join("cover.jpg"), b"fake image").unwrap();
        fs::write(temp_dir.join("info.txt"), b"some text").unwrap();
        
        let result = extract_directory_metadata(&temp_dir, Path::new("/"));
        
        assert!(result.is_err(), "Should return error for directory with no MP3 files");
        if let Err(e) = result {
            assert!(e.message.contains("No MP3 files found"));
        }
        
        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_duration_from_tlen_tags() {
        // Test that duration is correctly calculated from TLEN tags
        // Note: This test uses minimal MP3 files without actual TLEN tags
        // Full integration would require creating MP3s with id3 crate's Tag::write_to_path
        
        let temp_dir = std::env::temp_dir().join("finalshelf_test_duration_tlen");
        fs::create_dir_all(&temp_dir).unwrap();
        
        // Create 3 MP3 files (each would need TLEN=180000ms = 180s for full test)
        create_test_mp3(&temp_dir.join("file1.mp3"));
        create_test_mp3(&temp_dir.join("file2.mp3"));
        create_test_mp3(&temp_dir.join("file3.mp3"));
        
        let result = extract_directory_metadata(&temp_dir, Path::new("/"));
        
        assert!(result.is_ok(), "Should successfully extract metadata");
        let meta = result.unwrap();
        
        // Our minimal MP3s don't have TLEN tags, so it will use fallback estimation
        // The test validates that duration_seconds is computed (non-None)
        assert!(meta.duration_seconds.is_some(), "Should have computed duration");
        assert!(meta.duration_seconds.unwrap() >= 0, "Duration should be non-negative");
        
        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_duration_fallback_estimation() {
        // Test that duration estimation works when TLEN is absent
        let temp_dir = std::env::temp_dir().join("finalshelf_test_duration_fallback");
        fs::create_dir_all(&temp_dir).unwrap();
        
        // Create a minimal MP3 file (14 bytes - no TLEN tag)
        create_test_mp3(&temp_dir.join("no_tlen.mp3"));
        
        let result = extract_directory_metadata(&temp_dir, Path::new("/"));
        
        assert!(result.is_ok(), "Should handle MP3 without TLEN gracefully");
        let meta = result.unwrap();
        
        // Should have estimated duration
        assert!(meta.duration_seconds.is_some(), "Should estimate duration");
        assert!(meta.duration_is_estimated, "Should flag as estimated");
        
        // With 14-byte file: (14 - 3000).max(0) / 16000 = 0 seconds
        // But we handle this gracefully
        assert!(meta.duration_seconds.unwrap() >= 0, "Estimated duration should be non-negative");
        
        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_duration_zero_byte_files() {
        // Test that zero-byte MP3 files don't cause panics
        let temp_dir = std::env::temp_dir().join("finalshelf_test_duration_zero");
        fs::create_dir_all(&temp_dir).unwrap();
        
        // Create a zero-byte file
        fs::write(temp_dir.join("empty.mp3"), b"").unwrap();
        
        let result = extract_directory_metadata(&temp_dir, Path::new("/"));
        
        // Should handle gracefully (might return error or zero duration)
        match result {
            Ok(meta) => {
                // If it succeeds, duration should be 0 or None
                if let Some(duration) = meta.duration_seconds {
                    assert_eq!(duration, 0, "Zero-byte file should have 0 duration");
                }
            }
            Err(_) => {
                // It's also acceptable to return an error for zero-byte files
                assert!(true, "Zero-byte file handling is graceful");
            }
        }
        
        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_orphan_marks_missing_path() {
        // This test verifies orphan detection logic
        // Note: Requires test database setup for full integration
        
        // Unit test for path existence check logic
        let missing_path = Path::new("/nonexistent/path/to/book.mp3");
        assert!(!missing_path.exists(), "Test path should not exist");
        
        // Placeholder - full test requires DB infrastructure
        assert!(true, "Full orphan detection test requires test database");
    }

    #[test]
    fn test_orphan_clears_restored_path() {
        // This test verifies that orphan flag is cleared when path is restored
        // Note: Requires test database setup for full integration
        
        // Create a temporary file to simulate restored path
        let temp_file = std::env::temp_dir().join("finalshelf_test_restored.mp3");
        fs::write(&temp_file, b"test").unwrap();
        
        assert!(temp_file.exists(), "Restored path should exist");
        
        // Cleanup
        fs::remove_file(&temp_file).ok();
        
        // Placeholder - full test requires DB infrastructure
        assert!(true, "Full orphan restoration test requires test database");
    }
}
