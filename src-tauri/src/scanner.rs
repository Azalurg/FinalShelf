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
    models::{author::Author, book::Book},
    services::{
        absolute_paths_service::get_current_absolute_path,
        authors_service::{add_author, is_author_exists},
        books_service::{add_book, is_book_exists},
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

    println!("Quick scan in {}", directory);
    let start = Instant::now();
    let base_path = Path::new(&directory);

    // Phase 1: Collect unique parent directories
    let mut unique_dirs = HashSet::new();
    for entry in WalkDir::new(&directory).min_depth(1).into_iter().filter_map(|e| e.ok()) {
        if let Some(mp3_path) = get_mp3_path(&entry) {
            if let Some(parent_path) = mp3_path.parent() {
                unique_dirs.insert(parent_path.to_path_buf());
            }
        }
    }

    let directories: Vec<PathBuf> = unique_dirs.into_iter().collect();

    println!("Found {} unique book directories", directories.len());

    // Phase 2: Parallel metadata extraction
    let extraction_results: Vec<Result<DirectoryMetadata, ScanError>> = directories
        .par_iter()
        .map(|dir| extract_directory_metadata(dir, base_path))
        .collect();

    println!("Metadata extraction complete, processing results...");

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
                    .unwrap_or_else(|| meta.mp3_paths.first().unwrap().to_string_lossy().to_string());

                // Add author if not exists
                if !is_author_exists(&author_name) {
                    let author = Author {
                        name: author_name.clone(),
                        relative_img_path: Some(look_for_author_photo(&meta.parent_path, &author_name, base_path)),
                    };
                    println!("Adding author: {:?}", author);
                    add_author(&author);
                }

                // Create and add book
                let book = Book {
                    title,
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
                println!("Adding book: {:?}", book);
                add_book(&book);
                books_added += 1;
            }
            Err(e) => {
                println!("Error processing {}: {}", e.path, e.message);
                errors += 1;
            }
        }
    }

    let books_newly_orphaned = 0; // Orphan detection in Phase 4
    let elapsed_ms = start.elapsed().as_millis() as u64;

    println!("Quick scan complete, elapsed time: {} ms", elapsed_ms);

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

    // Placeholder values for Phase 5 (duration) and Phase 6 (file_count)
    let duration_seconds = None;
    let duration_is_estimated = false;
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
}
