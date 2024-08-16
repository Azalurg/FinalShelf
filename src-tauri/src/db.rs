use serde::Serialize;
use rusqlite::{params, Connection, Result};

const DB_FILE: &str = "/tmp/libraalchemy2.db"; // Development database

#[derive(Serialize)]
pub struct Author {
    id: i64,
    name: String,
}

#[derive(Serialize)]
pub struct Book {
    id: i64,
    title: String,
    genre: String,
    author_id: i64,
}

pub fn get_db_connection() -> Result<Connection> {
    Connection::open(DB_FILE)
}

pub fn init_db(conn: &Connection) -> Result<()> {
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS authors (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL UNIQUE,
            genre TEXT NOT NULL,
            author_id INTEGER NOT NULL,
            FOREIGN KEY (author_id) REFERENCES authors (id)
        )",
        [],
    )?;

    Ok(())
}



pub fn add_author(conn: &Connection, author: &str) -> Result<()> {
    conn.execute("INSERT INTO authors (name) VALUES (?)", params![author])?;
    Ok(())
}

pub fn get_or_create_author(conn: &Connection, author_name: &str) -> Result<i64> {
    let mut stmt = conn.prepare("SELECT id FROM authors WHERE name = ?1")?;
    let mut rows = stmt.query(params![author_name])?;

    if let Some(row) = rows.next()? {
        let author_id: i64 = row.get(0)?;
        return Ok(author_id);
    }

    conn.execute("INSERT INTO authors (name) VALUES (?1)", params![author_name])?;

    let author_id = conn.last_insert_rowid();
    Ok(author_id)
}

pub fn add_book(conn: &Connection, title: &str, genre: &str, author_id: i64) -> Result<()> {
    conn.execute("INSERT INTO books (title, genre, author_id) VALUES (?, ?, ?)", params![title, genre, author_id])?;
    Ok(())
}

pub fn get_authors(conn: &Connection) -> Result<Vec<Author>> {
    let mut stmt = conn.prepare("SELECT id, name FROM authors")?;
    let author_iter = stmt.query_map([], |row| {
        Ok(Author {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let mut authors = Vec::new();
    for author in author_iter {
        authors.push(author?);
    }
    Ok(authors)
}

pub fn get_books(conn: &Connection) -> Result<Vec<Book>> {
    let mut stmt = conn.prepare("SELECT id, title, genre, author_id FROM books")?;
    let book_iter = stmt.query_map([], |row| {
        Ok(Book {
            id: row.get(0)?,
            title: row.get(1)?,
            genre: row.get(2)?,
            author_id: row.get(3)?,
        })
    })?;

    let mut books = Vec::new();
    for book in book_iter {
        books.push(book?);
    }
    Ok(books)
}

use std::fs;
// TODO: Change this function in the future
pub fn clear_db() -> Result<()> {
    fs::remove_file(DB_FILE).ok();
    Ok(())
}
