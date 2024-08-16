use std::collections::HashMap;

use walkdir::WalkDir;
use id3::{Tag, TagLike};
use rusqlite::Result;

use crate::db::{add_book, clear_db, get_db_connection, get_or_create_author, init_db};



pub fn scan_for_metadata(directory: String) -> Result<()> {
    

    println!("Scanning for metadata in {}", directory);
    clear_db()?;
    println!("Database cleared");
    let conn = get_db_connection()?;
    init_db(&conn)?;
    println!("Database initialized");

    let mut books = HashMap::new();

    // TODO: Check the for loop below
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("mp3") {
            if let Ok(tag) = Tag::read_from_path(entry.path()) {
                let title = tag.title().unwrap_or("Unknown").to_string();
                let genre = tag.genre().unwrap_or("Unknown").to_string();
                let author = tag.artist().unwrap_or("Unknown").to_string();
                books.insert(title, (genre, author));
                
            }
        }
    }

    for (title, (genre, author)) in books {
        let author_id = get_or_create_author(&conn, &author)?;
        add_book(&conn, &title, &genre, author_id)?;
    }
    Ok(())
}
    
    