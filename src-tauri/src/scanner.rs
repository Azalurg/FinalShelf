use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
    time::Instant,
};

use id3::{Tag, TagLike};
use rusqlite::Result;
use walkdir::WalkDir;

use crate::{
    models::models::{Author, Book},
    services::books_service::{add_book, is_book_exists},
    services::authors_service::{add_author, is_author_exists},
};

fn look_for_cover(directory: &str) -> String {
    let exts = [".jpg", ".jpeg", ".png", ".gif", ".webp", ".nfo"];
    let names = ["cover", "folder", "album", "poster", "default", "art"];
    for name in names.iter() {
        for ext in exts.iter() {
            let image_name = format!("{}{}", name, ext);
            let image_path = format!("{}/{}", directory, image_name);
            if fs::metadata(&image_path).is_ok() {
                return image_path;
            }
        }
    }
    String::new()
}

fn look_for_author_photo(path: &str, name: &str) -> String {
    let directory = match path.find(name) {
        Some(index) => &path[..index + name.len()],
        None => return String::new(),
    };

    return look_for_cover(directory);
}

pub fn quick_scan(directory: &str) -> Result<(), String> {
    println!("Quick scan in {}", directory);
    let mut processed_dirs = HashSet::new();
    let start = Instant::now();

    for entry in WalkDir::new(directory).min_depth(1).into_iter().filter_map(|e| e.ok()) {
        if let Some(mp3_path) = get_mp3_path(&entry) {
            let parent_path = mp3_path.parent().and_then(Path::to_str).unwrap();

            if processed_dirs.insert(parent_path.to_string()) {
                if let Ok(tag) = Tag::read_from_path(mp3_path) {
                    process_metadata(&tag, parent_path);
                }
            }
        }
    }

    println!("Quick scan complete, elapsed time: {:?}", start.elapsed());

    Ok(())
}

// pub fn full_scan(directory: &str) -> Result<()> {
//     println!("Full scan in {}", directory);
//     let mut books_hashmap = HashMap::new();
//     let start = Instant::now();

//     for entry in WalkDir::new(directory).min_depth(1).into_iter().filter_map(|e| e.ok()) {
//         if let Some(mp3_path) = get_mp3_path(&entry) {
//             if let Ok(tag) = Tag::read_from_path(mp3_path) {
//                 let title = tag.album().unwrap_or("Unknown").to_string();
//                 let duration = tag.duration().unwrap_or(0) as u64;

//                 if let Some(&book_id) = books_hashmap.get(&title) {
//                     db::increment_book_duration(&conn, book_id, duration)?;
//                 } else {
//                     let book_id = process_metadata(&conn, &tag, mp3_path.parent().unwrap().to_str().unwrap())?;
//                     books_hashmap.insert(title, book_id);
//                 }
//             }
//         }
//     }

//     println!("Full scan complete, elapsed time: {:?}", start.elapsed());
//     Ok(())
// }

// Helper function to extract mp3 path from a directory entry
fn get_mp3_path(entry: &walkdir::DirEntry) -> Option<&Path> {
    let path = entry.path();
    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("mp3") {
        Some(path)
    } else {
        None
    }
}

fn process_metadata(tag: &Tag, parent_path: &str) -> Option<Book> {
    let title = tag.album().unwrap_or("Unknown").to_string();
    let genre = tag.genre().unwrap_or("Unknown").to_string();
    let lector = tag.artist().unwrap_or("Unknown").to_string();
    let year = tag.year().unwrap_or(0);
    let duration = tag.duration().unwrap_or(0) as u64;
    let author_name = tag.album_artist().unwrap_or("Unknown").to_string();
    let relative_cover_path = look_for_cover(parent_path);

    if is_book_exists(&title) {
        return None;
    }

    if !is_author_exists(&author_name) {
        let author = Author {
            name: author_name.clone(),
            relative_img_path: Some(look_for_author_photo(parent_path, &author_name)),
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
        create_date: None

    }; 
    println!("Adding book: {:?}", book);
    add_book(&book)
}
