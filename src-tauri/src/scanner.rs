use std::{
    collections::HashSet,
    fs,
    path::Path,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use chrono::{Date, DateTime, NaiveDateTime, Utc};
use id3::{Tag, TagLike};
use rusqlite::Result;
use walkdir::WalkDir;

use crate::{
    models::{author::Author, book::Book},
    services::{
        absolute_paths_service::get_current_absolute_path,
        authors_service::{add_author, is_author_exists},
        // Added get_all_book_paths to the import
        books_service::{add_book, get_all_book_paths, is_book_exists},
    },
};

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

fn system_time_to_naive_date_time(
    create_time: Option<SystemTime>,
    edit_time: Option<SystemTime>,
) -> Option<NaiveDateTime> {
    let create_data_time = DateTime::<Utc>::from(create_time?);
    let edit_data_time = DateTime::<Utc>::from(edit_time?);

    if create_data_time > edit_data_time {
        Some(create_data_time.naive_utc())
    } else {
        Some(edit_data_time.naive_utc())
    }
}

pub fn quick_scan() -> Result<(), String> {
    let absolute_path_obj = get_current_absolute_path();
    let directory: String;
    if let Some(absolute_path) = absolute_path_obj {
        directory = absolute_path.absolute_path;
    } else {
        return Err("No path to scan".to_string());
    }

    println!("Quick scan in {}", directory);
    let start = Instant::now();
    let base_path = Path::new(&directory);

    // Build the processed_dirs HashSet based on the absolute_path and file paths from the database.
    let mut processed_dirs: HashSet<String> = match get_all_book_paths() {
        Ok(relative_paths) => relative_paths
            .into_iter()
            .filter_map(|relative_path| {
                // Get the parent directory of the relative path
                Path::new(&relative_path).parent().map(|p| {
                    // Construct the full absolute path and add it to the set
                    base_path.join(p).to_string_lossy().to_string()
                })
            })
            .collect(),
        Err(e) => {
            eprintln!(
                "Could not pre-populate processed directories from DB: {}. Starting fresh.",
                e
            );
            HashSet::new()
        },
    };

    println!("Pre-populated processed directories: {:?}", processed_dirs);

    for entry in WalkDir::new(&directory).min_depth(1).into_iter().filter_map(Result::ok) {
        if let Some(mp3_path) = get_mp3_path(&entry) {
            if let Some(parent_path) = mp3_path.parent().and_then(Path::to_str) {
                // Skip already processed directories.
                // The set is now pre-populated with directories from the database.
                if !processed_dirs.insert(parent_path.to_string()) {
                    continue;
                }

                // Process the metadata of the mp3 file
                if let Ok(tag) = Tag::read_from_path(mp3_path) {
                    let file_create_date = fs::metadata(mp3_path).ok().and_then(|metadata| {
                        system_time_to_naive_date_time(metadata.created().ok(), metadata.modified().ok())
                    });

                    process_metadata(&tag, parent_path, file_create_date, base_path, mp3_path);
                }
            }
        }
    }

    println!("Quick scan complete, elapsed time: {:?}", start.elapsed());
    Ok(())
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

fn process_metadata(
    tag: &Tag,
    parent_path: &str,
    file_create_date: Option<NaiveDateTime>,
    base_path: &Path,
    mp3_path: &Path,
) -> Option<Book> {
    let title = tag.album().unwrap_or(&format!("Unknown ({})", parent_path)).to_string();

    if is_book_exists(&title) {
        return None;
    }

    let genre = tag.genre().unwrap_or("Unknown").to_string();
    let lector = tag.artist().unwrap_or("Unknown").to_string();
    let author_name = tag.album_artist().unwrap_or("Unknown").to_string();
    let relative_cover_path = look_for_cover(parent_path, base_path);
    let file_path_str = mp3_path
        .strip_prefix(base_path)
        .unwrap_or(&mp3_path)
        .to_string_lossy()
        .to_string();

    if !is_author_exists(&author_name) {
        let author = Author {
            name: author_name.clone(),
            relative_img_path: Some(look_for_author_photo(parent_path, &author_name, base_path)),
        };
        println!("Adding author: {:?}", author);
        add_author(&author);
    }

    let book = Book {
        title,
        author_name,
        relative_cover_path: Some(relative_cover_path),
        genre: Some(genre),
        lector: Some(lector),
        create_date: file_create_date,
        read: Some(false),
        score: Some(0),
        relative_file_path: file_path_str,
    };
    println!("Adding book: {:?}", book);
    add_book(&book)
}
