use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::LazyLock,
    time::Instant,
    fs,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use lofty::file::TaggedFileExt;
use lofty::prelude::*;
use lofty::tag::ItemKey;
use walkdir::WalkDir;

use crate::{
    models::{author::Author, book::Book, series::NewSeries},
    services::{
        absolute_paths_service::get_current_absolute_path,
        authors_service::{add_author, is_author_exists},
        books_service::{add_book, get_all_book_paths, is_book_exists},
        series_service::{create_series, get_series_by_author},
    },
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const AUDIO_EXTENSIONS: &[&str] = &["mp3", "m4b", "m4a", "ogg", "flac", "aac"];
const COVER_NAMES: &[&str] = &["cover", "folder", "album", "poster", "default", "art"];
const COVER_EXTENSIONS: &[&str] = &[".jpg", ".jpeg", ".png", ".gif", ".webp"];

// Compiled once at first use — avoids repeated regex construction on hot scan paths.
static SERIES_TITLE_PATTERN1: LazyLock<regex_lite::Regex> = LazyLock::new(|| {
    regex_lite::Regex::new(
        r"^(.+?)\s*[-–—]\s*(?:(?:Part|Book|Vol\.?|Volume|Episode|Ep\.?|#)?\s*)?(\d+)\s*[-–—]\s*(.+)$",
    )
    .expect("Valid SERIES_TITLE_PATTERN1 regex")
});

static SERIES_TITLE_PATTERN2: LazyLock<regex_lite::Regex> = LazyLock::new(|| {
    regex_lite::Regex::new(r"^(.+?)\s+(\d+)\s*[-–—]\s*(.+)$")
        .expect("Valid SERIES_TITLE_PATTERN2 regex")
});

static SERIES_DIR_ORDER: LazyLock<regex_lite::Regex> = LazyLock::new(|| {
    regex_lite::Regex::new(r"^(\d+)\s*[-–—]\s*(.+)$").expect("Valid SERIES_DIR_ORDER regex")
});

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

#[derive(serde::Serialize, Clone, Debug)]
pub struct ScanResult {
    pub added: i64,
    pub skipped: i64,
    pub errors: Vec<String>,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct ScanProgress {
    pub phase: String,
    pub current: i64,
    pub total: i64,
    pub message: String,
}

/// Run a library scan with optional progress callback.
///
/// - `full = false` (quick scan): skips directories already present in the DB.
/// - `full = true`  (full scan):  examines every directory, adds missing books.
/// - `on_progress` is called during scanning to report progress.
pub fn scan_with_progress<F>(full: bool, mut on_progress: F) -> Result<ScanResult, String>
where
    F: FnMut(ScanProgress),
{
    let absolute_path_obj = get_current_absolute_path()
        .ok_or_else(|| "No library path configured. Add a path in Settings first.".to_string())?;

    let directory = absolute_path_obj.absolute_path;
    let base_path = Path::new(&directory);

    println!("Starting {} scan in {}", if full { "full" } else { "quick" }, directory);
    let start = Instant::now();

    // -- Phase 1: Discovery ---------------------------------------------------
    on_progress(ScanProgress {
        phase: "discovery".to_string(),
        current: 0,
        total: 0,
        message: "Discovering directories...".to_string(),
    });

    let skip = build_skip_set(full, base_path);
    let book_dirs = discover_book_dirs(base_path, &skip);
    let total_dirs = book_dirs.len() as i64;

    on_progress(ScanProgress {
        phase: "discovery".to_string(),
        current: total_dirs,
        total: total_dirs,
        message: format!("Discovered {} new book directories", total_dirs),
    });

    println!("Discovered {} new book directories", book_dirs.len());

    // -- Phase 2: Extraction --------------------------------------------------
    let mut candidates: Vec<BookCandidate> = Vec::new();
    for (idx, dir) in book_dirs.iter().enumerate() {
        on_progress(ScanProgress {
            phase: "extraction".to_string(),
            current: idx as i64 + 1,
            total: total_dirs,
            message: format!(
                "Extracting metadata from: {}",
                dir.file_name().unwrap_or_default().to_string_lossy()
            ),
        });

        match extract_metadata(dir, base_path) {
            Ok(candidate) => candidates.push(candidate),
            Err(e) => {
                eprintln!("Skipping {:?}: {}", dir, e);
            },
        }
    }
    println!("Extracted metadata for {} books", candidates.len());

    // -- Phase 3: Persist -----------------------------------------------------
    on_progress(ScanProgress {
        phase: "persist".to_string(),
        current: 0,
        total: candidates.len() as i64,
        message: "Saving to database...".to_string(),
    });

    let result = persist_batch_with_progress(candidates, &mut on_progress);

    on_progress(ScanProgress {
        phase: "complete".to_string(),
        current: result.added,
        total: result.added + result.skipped,
        message: format!(
            "Scan complete: {} added, {} skipped, {} errors",
            result.added,
            result.skipped,
            result.errors.len()
        ),
    });

    println!(
        "Scan complete: {} added, {} skipped, {} errors. Elapsed: {:?}",
        result.added,
        result.skipped,
        result.errors.len(),
        start.elapsed()
    );

    Ok(result)
}

// ---------------------------------------------------------------------------
// Internal data structures
// ---------------------------------------------------------------------------

struct BookCandidate {
    title: String,
    author_name: String,
    genre: Option<String>,
    lector: Option<String>,
    duration_seconds: i32,
    create_date: Option<NaiveDateTime>,
    relative_cover_path: Option<String>,
    relative_file_path: String,
    author_img_path: Option<String>,
    // Series detection fields
    detected_series_name: Option<String>,
    detected_series_order: Option<i32>,
}

/// Detected series information from title or directory structure.
struct SeriesDetection {
    series_name: String,
    series_order: Option<i32>,
    /// The cleaned title with series prefix removed.
    cleaned_title: String,
}

// ---------------------------------------------------------------------------
// Phase 1 – Discovery
// ---------------------------------------------------------------------------

fn build_skip_set(full: bool, base_path: &Path) -> HashSet<String> {
    if full {
        return HashSet::new();
    }

    match get_all_book_paths() {
        Ok(relative_paths) => relative_paths
            .into_iter()
            .filter_map(|rel| {
                Path::new(&rel)
                    .parent()
                    .map(|p| base_path.join(p).to_string_lossy().to_string())
            })
            .collect(),
        Err(e) => {
            eprintln!("Could not load known dirs from DB: {}. Starting fresh.", e);
            HashSet::new()
        },
    }
}

/// Walk the directory tree and return paths of directories that contain audio
/// files and are not in the `skip` set.
fn discover_book_dirs(root: &Path, skip: &HashSet<String>) -> Vec<PathBuf> {
    let mut book_dirs: HashSet<PathBuf> = HashSet::new();

    for entry in WalkDir::new(root).min_depth(1).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if is_audio_file(path) {
            if let Some(parent) = path.parent() {
                let parent_str = parent.to_string_lossy().to_string();
                if !skip.contains(&parent_str) {
                    book_dirs.insert(parent.to_path_buf());
                }
            }
        }
    }

    let mut dirs: Vec<PathBuf> = book_dirs.into_iter().collect();
    dirs.sort();
    dirs
}

fn is_audio_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| AUDIO_EXTENSIONS.contains(&e.to_lowercase().as_str()))
            .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Phase 2 – Extraction
// ---------------------------------------------------------------------------

/// Collect all audio files in a directory (non-recursive, sorted).
fn find_audio_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| is_audio_file(p))
        .collect();
    files.sort();
    files
}

/// Extract metadata for a single book directory.
fn extract_metadata(book_dir: &Path, base_path: &Path) -> Result<BookCandidate, String> {
    let audio_files = find_audio_files(book_dir);
    if audio_files.is_empty() {
        return Err(format!("No audio files in {:?}", book_dir));
    }

    let primary_file = &audio_files[0];

    // Read tags from the first file (alphabetically)
    let tagged_file = lofty::read_from_path(primary_file)
        .map_err(|e| format!("Failed to read tags from {:?}: {}", primary_file, e))?;

    let dir_fallback_name = book_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let (title, author_name, genre, lector) = if let Some(tag) = tagged_file.primary_tag() {
        (
            tag.album()
                .map(|s| s.to_string())
                .unwrap_or_else(|| dir_fallback_name.clone()),
            tag.get_string(&ItemKey::AlbumArtist)
                .map(|s| s.to_string())
                .or_else(|| tag.artist().map(|s| s.to_string()))
                .unwrap_or_else(|| "Unknown".to_string()),
            tag.genre().map(|s| s.to_string()),
            tag.artist().map(|s| s.to_string()),
        )
    } else {
        (dir_fallback_name, "Unknown".to_string(), None, None)
    };

    // Sum duration of all audio files in the directory
    let duration_seconds = sum_duration(&audio_files);

    // File creation / modification date
    let create_date = get_file_date(primary_file);

    // Cover image search
    let relative_cover_path = find_cover(book_dir, base_path);

    // Representative file path (relative)
    let relative_file_path = primary_file
        .strip_prefix(base_path)
        .unwrap_or(primary_file)
        .to_string_lossy()
        .to_string();

    // Author photo
    let author_img_path = find_author_photo(book_dir, &author_name, base_path);

    // Series detection: may also clean the title by removing series prefix
    let detection = detect_series(&title, book_dir, &author_name, base_path);
    let (clean_title, detected_series_name, detected_series_order) = match detection {
        Some(d) => (d.cleaned_title, Some(d.series_name), d.series_order),
        None => (title, None, None),
    };

    Ok(BookCandidate {
        title: clean_title,
        author_name,
        genre,
        lector,
        duration_seconds,
        create_date,
        relative_cover_path,
        relative_file_path,
        author_img_path,
        detected_series_name,
        detected_series_order,
    })
}

/// Sum the duration (in seconds) of all audio files in the list.
/// Uses lofty's audio properties which read the header only – fast even for
/// directories with many files.
fn sum_duration(audio_files: &[PathBuf]) -> i32 {
    audio_files
        .iter()
        .filter_map(|path| {
            lofty::read_from_path(path)
                .ok()
                .map(|f| f.properties().duration().as_secs() as i32)
        })
        .sum()
}

/// Try to find a cover image in the given directory.
fn find_cover(directory: &Path, base_path: &Path) -> Option<String> {
    for name in COVER_NAMES {
        for ext in COVER_EXTENSIONS {
            let filename = format!("{}{}", name, ext);
            let image_path = directory.join(&filename);
            if image_path.exists() {
                return Some(
                    image_path
                        .strip_prefix(base_path)
                        .unwrap_or(&image_path)
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }
    }
    None
}

/// Navigate up to the author-level directory and look for a cover image there.
fn find_author_photo(book_dir: &Path, author_name: &str, base_path: &Path) -> Option<String> {
    let dir_str = book_dir.to_string_lossy();
    let index = dir_str.find(author_name)?;
    let author_dir = Path::new(&dir_str[..index + author_name.len()]);
    find_cover(author_dir, base_path)
}

/// Get the most recent of created/modified timestamps as a NaiveDateTime.
fn get_file_date(path: &Path) -> Option<NaiveDateTime> {
    let metadata = fs::metadata(path).ok()?;
    let created = metadata.created().ok();
    let modified = metadata.modified().ok();

    let ts = match (created, modified) {
        (Some(c), Some(m)) => {
            if c > m {
                c
            } else {
                m
            }
        },
        (Some(c), None) => c,
        (None, Some(m)) => m,
        (None, None) => return None,
    };

    Some(DateTime::<Utc>::from(ts).naive_utc())
}

// ---------------------------------------------------------------------------
// Series Detection
// ---------------------------------------------------------------------------

/// Try to detect series from title patterns.
/// Supports patterns like:
/// - "Series Name - 01 - Book Title"
/// - "Series Name 01 - Book Title"
/// - "Series Name - Part 1 - Book Title"
/// - "Series Name Book 1 - Book Title"
fn detect_series_from_title(title: &str) -> Option<SeriesDetection> {
    // Pattern 1: "Series - 01 - Title" or "Series - Part 1 - Title"
    if let Some(caps) = SERIES_TITLE_PATTERN1.captures(title) {
        let series_name = caps.get(1)?.as_str().trim().to_string();
        let order: i32 = caps.get(2)?.as_str().parse().ok()?;
        let book_title = caps.get(3)?.as_str().trim().to_string();

        // Validate series name is reasonable (not just numbers or too short)
        if series_name.len() >= 2 && !series_name.chars().all(|c| c.is_numeric() || c.is_whitespace()) {
            return Some(SeriesDetection {
                series_name,
                series_order: Some(order),
                cleaned_title: book_title,
            });
        }
    }

    // Pattern 2: "Series 01 - Title" (number directly after series name)
    if let Some(caps) = SERIES_TITLE_PATTERN2.captures(title) {
        let series_name = caps.get(1)?.as_str().trim().to_string();
        let order: i32 = caps.get(2)?.as_str().parse().ok()?;
        let book_title = caps.get(3)?.as_str().trim().to_string();

        if series_name.len() >= 2 && !series_name.chars().all(|c| c.is_numeric() || c.is_whitespace()) {
            return Some(SeriesDetection {
                series_name,
                series_order: Some(order),
                cleaned_title: book_title,
            });
        }
    }

    None
}

/// Try to detect series from directory structure.
/// Looks for pattern: .../Author/Series/Book or .../Author/Series/NN - Book
fn detect_series_from_directory(book_dir: &Path, author_name: &str, base_path: &Path) -> Option<(String, Option<i32>)> {
    // Get the relative path from base
    let rel_path = book_dir.strip_prefix(base_path).ok()?;
    let components: Vec<&str> = rel_path.components().filter_map(|c| c.as_os_str().to_str()).collect();

    // We need at least 3 levels: Author/Series/Book
    if components.len() < 3 {
        return None;
    }

    // Find the author directory in the path
    let author_index = components
        .iter()
        .position(|&c| c.to_lowercase() == author_name.to_lowercase())?;

    // The next component after author should be the series
    // (if there's still a book directory after that)
    if author_index + 2 < components.len() {
        let series_name = components[author_index + 1].to_string();
        let book_dir_name = components[author_index + 2];

        // Validate series name: not just numbers, reasonable length
        if series_name.len() >= 2
            && !series_name
                .chars()
                .all(|c| c.is_numeric() || c.is_whitespace() || c == '-')
        {
            let series_order = SERIES_DIR_ORDER
                .captures(book_dir_name)
                .and_then(|caps| caps.get(1))
                .and_then(|m| m.as_str().parse::<i32>().ok());

            return Some((series_name, series_order));
        }
    }

    None
}

/// Combine series detection from title and directory structure.
/// Title detection takes priority as it often includes order information.
/// Returns `Some(SeriesDetection)` when a series is detected, with `cleaned_title`
/// holding the book title with any series prefix stripped. Returns `None` when
/// no series is detected and the original title should be used.
fn detect_series(title: &str, book_dir: &Path, author_name: &str, base_path: &Path) -> Option<SeriesDetection> {
    // First try title-based detection (includes order and cleaned title)
    if let Some(detection) = detect_series_from_title(title) {
        return Some(detection);
    }

    // Fall back to directory-based detection (no order info, title unchanged)
    if let Some((series_name, series_order)) = detect_series_from_directory(book_dir, author_name, base_path) {
        return Some(SeriesDetection {
            series_name,
            series_order,
            cleaned_title: title.to_string(),
        });
    }

    None
}

// ---------------------------------------------------------------------------
// Phase 3 – Persist
// ---------------------------------------------------------------------------

fn persist_batch_with_progress<F>(candidates: Vec<BookCandidate>, on_progress: &mut F) -> ScanResult
where
    F: FnMut(ScanProgress),
{
    let mut added: i64 = 0;
    let mut skipped: i64 = 0;
    let mut errors: Vec<String> = Vec::new();
    let total = candidates.len() as i64;

    // Cache for series IDs: (author_name, series_name_lowercase) -> series_id
    // A value of -1 means series creation previously failed (skip retry).
    let mut series_cache: HashMap<(String, String), i32> = HashMap::new();
    // Tracks authors whose full series list has been loaded into the cache.
    let mut authors_loaded: HashSet<String> = HashSet::new();

    for (idx, candidate) in candidates.into_iter().enumerate() {
        on_progress(ScanProgress {
            phase: "persist".to_string(),
            current: idx as i64 + 1,
            total,
            message: format!("Saving: {}", candidate.title),
        });

        // Skip books that are already in the database
        if is_book_exists(&candidate.title) {
            skipped += 1;
            continue;
        }

        // Create the author if not already present
        if !is_author_exists(&candidate.author_name) {
            let author = Author {
                name: candidate.author_name.clone(),
                relative_img_path: candidate.author_img_path,
            };
            println!("Adding author: {:?}", author);
            if let Err(e) = add_author(&author) {
                errors.push(format!("Failed to insert author '{}': {}", author.name, e));
                skipped += 1;
                continue;
            }
        }

        // Handle series detection
        let series_id = if let Some(ref series_name) = candidate.detected_series_name {
            let series_key = series_name.to_lowercase();
            let cache_key = (candidate.author_name.clone(), series_key.clone());

            if let Some(&cached_id) = series_cache.get(&cache_key) {
                // Positive: known series id; negative: known failure, skip
                if cached_id > 0 { Some(cached_id) } else { None }
            } else {
                // Determine if series already exists for this author.
                // Load all of the author's series on first encounter to populate
                // the cache and avoid per-book DB queries.
                let existing_series_id = if authors_loaded.contains(&candidate.author_name) {
                    // Already loaded this author's series — not in cache means doesn't exist yet
                    None
                } else {
                    authors_loaded.insert(candidate.author_name.clone());
                    get_series_by_author(&candidate.author_name)
                        .ok()
                        .and_then(|series_list| {
                            for s in &series_list {
                                let key = (candidate.author_name.clone(), s.name.to_lowercase());
                                series_cache.entry(key).or_insert(s.id);
                            }
                            series_list
                                .iter()
                                .find(|s| s.name.to_lowercase() == series_key)
                                .map(|s| s.id)
                        })
                };

                let id = if let Some(existing_id) = existing_series_id {
                    existing_id
                } else {
                    // Create new series
                    match create_series(NewSeries {
                        name: series_name.clone(),
                        author_name: candidate.author_name.clone(),
                        description: None,
                    }) {
                        Ok(new_series) => {
                            println!("Created series: {} (id: {})", series_name, new_series.id);
                            new_series.id
                        },
                        Err(e) => {
                            eprintln!("Failed to create series '{}': {}", series_name, e);
                            -1  // failure sentinel
                        },
                    }
                };

                // Always cache the result so we never retry the same DB operation
                series_cache.insert(cache_key, id);
                if id > 0 { Some(id) } else { None }
            }
        } else {
            None
        };

        let book = Book {
            title: candidate.title.clone(),
            author_name: candidate.author_name,
            relative_cover_path: candidate.relative_cover_path,
            genre: candidate.genre,
            lector: candidate.lector,
            create_date: candidate.create_date,
            read: Some(false),
            score: Some(0),
            relative_file_path: candidate.relative_file_path,
            duration_seconds: Some(candidate.duration_seconds),
            series_id,
            series_order: candidate.detected_series_order,
        };

        println!("Adding book: {:?}", book);
        match add_book(&book) {
            Ok(_) => added += 1,
            Err(e) => errors.push(format!("Failed to insert {}: {}", candidate.title, e)),
        }
    }

    ScanResult { added, skipped, errors }
}
