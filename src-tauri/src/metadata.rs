use std::collections::HashMap;

use walkdir::WalkDir;
use id3::{Tag, TagLike};
use rusqlite::Result;

use crate::db::{self};

pub fn scan_for_metadata(directory: String) -> Result<()> {
    println!("Scanning for metadata in {}", directory);
    let conn = db::get_db_connection()?;
    let mut books_hashmap = HashMap::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("mp3") {
            if let Ok(tag) = Tag::read_from_path(entry.path()) {
                let title = tag.album().unwrap_or("Unknown").to_string();
                let genre = tag.genre().unwrap_or("Unknown").to_string();
                let lector = tag.artist().unwrap_or("Unknown").to_string();
                let year = tag.year().unwrap_or(0);
                let duration = tag.duration().unwrap_or(0) as u64;
                let author= tag.album_artist().unwrap_or("Unknown").to_string();

                if !books_hashmap.contains_key(&title) {
                    let author_id = db::get_or_create_author(&conn, &author)?;
                    let lector_id = db::get_or_create_lector(&conn, &lector)?;
                    let book = db::Book::new(title.clone(), genre, duration, year, author_id, lector_id);
                    let book_id = db::get_or_create_book(&conn, &book).unwrap();
                    books_hashmap.insert(title, book_id);
                } else {
                    let book_id = books_hashmap.get(&title).unwrap();
                    let _ = db::increment_book_duration(&conn, *book_id, duration);
                }
            }
        }
    }
    println!("Metadata scan complete");
    Ok(())
}
    
    