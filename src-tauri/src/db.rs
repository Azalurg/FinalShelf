use serde::Serialize;
use rusqlite::{params, Connection, Result};

const DB_FILE: &str = "/tmp/libraalchemy2.db"; // Development database

#[derive(Serialize)]
pub struct Author {
    id: i64,
    name: String,
}

#[derive(Serialize)]
pub struct Lector {
    id: i64,
    name: String,
}

#[derive(Serialize)]
pub struct Book {
    id: i64,
    title: String,
    genre: String,
    duration: u64,
    year: i32,
    author_id: i64,
    lector_id: i64,
}

impl Book {
    pub fn new(title: String, genre: String, duration: u64, year: i32, author_id: i64, lector_id: i64) -> Self {
        Book {
            id: 0,
            title,
            genre,
            duration,
            year,
            author_id,
            lector_id,
        }
    }
    
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
        "CREATE TABLE IF NOT EXISTS lectors (
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
            duration INTEGER NOT NULL,
            year INTEGER NOT NULL,
            author_id INTEGER NOT NULL,
            lector_id INTEGER NOT NULL,
            FOREIGN KEY (author_id) REFERENCES authors (id),
            FOREIGN KEY (lector_id) REFERENCES lectors (id)
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

pub fn get_or_create_lector(conn: &Connection, lector_name: &str) -> Result<i64> {
    let mut stmt = conn.prepare("SELECT id FROM lectors WHERE name = ?1")?;
    let mut rows = stmt.query(params![lector_name])?;

    if let Some(row) = rows.next()? {
        let lector_id: i64 = row.get(0)?;
        return Ok(lector_id);
    }

    conn.execute("INSERT INTO lectors (name) VALUES (?1)", params![lector_name])?;

    let lector_id = conn.last_insert_rowid();
    Ok(lector_id)
}

pub fn add_book(conn: &Connection, book: &Book) -> Result<(i64)> {
    conn.execute(
        "INSERT INTO books (title, genre, duration, year, author_id, lector_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![book.title, book.genre, book.duration, book.year, book.author_id, book.lector_id],
    )?;
    let book_id = conn.last_insert_rowid();
    Ok(book_id)
}

pub fn increment_book_duration(conn: &Connection, book_id: i64, duration: u64) -> Result<()> {
    conn.execute(
        "UPDATE books SET duration = duration + ?1 WHERE id = ?2",
        params![duration, book_id],
    )?;
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
    let mut stmt = conn.prepare("SELECT id, title, genre, duration, year, author_id, lector_id FROM books")?;
    let book_iter = stmt.query_map([], |row| {
        Ok(Book {
            id: row.get(0)?,
            title: row.get(1)?,
            genre: row.get(2)?,
            duration: row.get(3)?,
            year: row.get(4)?,
            author_id: row.get(5)?,
            lector_id: row.get(6)?,
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
