use rusqlite::{params, Connection, Result};

pub fn get_db_connection() -> Result<Connection> {
    match std::env::var("DATABASE_URL") {
        Ok(url) => Connection::open(url),
        Err(_) => Connection::open("finalshelf.db")
    }
}

// -------------------------
// Common database functions
// -------------------------

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS authors (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            picture_path TEXT
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
        "CREATE TABLE IF NOT EXISTS genres (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL UNIQUE,
            duration INTEGER NOT NULL,
            year INTEGER NOT NULL,
            cover_path TEXT,
            author_id INTEGER NOT NULL,
            lector_id INTEGER NOT NULL,
            genre_id INTEGER NOT NULL,
            FOREIGN KEY (author_id) REFERENCES authors (id),
            FOREIGN KEY (lector_id) REFERENCES lectors (id),
            FOREIGN KEY (genre_id) REFERENCES genres (id)
        )",
        [],
    )?;

    Ok(())
}

// -------------------------
// Book database functions
// -------------------------
pub fn add_book(conn: &Connection, book: &DBBook) -> Result<i64> {
    conn.execute(
        "INSERT INTO books (title, duration, year, cover_path, author_id, lector_id, genre_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![book.title, book.duration, book.year, book.cover_path, book.author_id, book.lector_id, book.genre_id],
    )?;
    let book_id = conn.last_insert_rowid();
    Ok(book_id)
}

pub fn get_book_id_by_title(conn: &Connection, title: &str) -> Result<i64> {
    let mut stmt = conn.prepare("SELECT id FROM books WHERE title = ?1")?;
    let mut rows = stmt.query(params![title])?;

    if let Some(row) = rows.next()? {
        let book_id: i64 = row.get(0)?;
        return Ok(book_id);
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn get_or_create_book(conn: &Connection, book: &DBBook) -> Result<i64> {
    match get_book_id_by_title(conn, &book.title) {
        Ok(book_id) => Ok(book_id),
        Err(_) => add_book(conn, book),
    }
}

pub fn increment_book_duration(conn: &Connection, book_id: i64, duration: u64) -> Result<()> {
    conn.execute(
        "UPDATE books SET duration = duration + ?1 WHERE id = ?2",
        params![duration, book_id],
    )?;
    Ok(())
}

pub fn get_all_books_list_frontend(conn: &Connection) -> Result<Vec<FrontendBook>> {
    let mut stmt = conn.prepare("SELECT books.id, books.title, books.cover_path, authors.id, authors.name FROM books JOIN authors ON books.author_id = authors.id")?;
    let book_iter = stmt.query_map([], |row| {
        Ok(FrontendBook {
            id: row.get(0)?,
            title: row.get(1)?,
            cover_path: row.get(2)?,
            author_id: row.get(3)?,
            author_name: row.get(4)?,
        })
    })?;

    let mut books = Vec::new();
    for book in book_iter {
        books.push(book?);
    }
    Ok(books)
}

pub fn get_book_by_id(conn: &Connection, book_id: i64) -> Result<FrontendBookDetails> {
    let mut stmt = conn.prepare("SELECT books.id, books.title, books.cover_path, books.duration, books.year, genres.id, genres.name, authors.id, authors.name, authors.picture_path, lectors.id, lectors.name FROM books JOIN genres ON books.genre_id = genres.id JOIN authors ON books.author_id = authors.id JOIN lectors ON books.lector_id = lectors.id WHERE books.id = ?1")?;
    let mut rows = stmt.query(params![book_id])?;

    if let Some(row) = rows.next()? {
        Ok(FrontendBookDetails {
            id: row.get(0)?,
            title: row.get(1)?,
            cover_path: row.get(2)?,
            duration: row.get(3)?,
            year: row.get(4)?,
            genre_id: row.get(5)?,
            genre_name: row.get(6)?,
            author_id: row.get(7)?,
            author_name: row.get(8)?,
            author_picture_path: row.get(9)?,
            lector_id: row.get(10)?,
            lector_name: row.get(11)?,
        })
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

// -------------------------
// Author database functions
// -------------------------

pub fn add_author(conn: &Connection, autor: &Author) -> Result<i64> {
    conn.execute(
        "INSERT INTO authors (name, picture_path) VALUES (?1, ?2)",
        params![autor.name, autor.picture_path],
    )?;
    let author_id = conn.last_insert_rowid();
    Ok(author_id)
}

pub fn get_author_id_by_name(conn: &Connection, name: &str) -> Result<i64> {
    let mut stmt = conn.prepare("SELECT id FROM authors WHERE name = ?1")?;
    let mut rows = stmt.query(params![name])?;

    if let Some(row) = rows.next()? {
        let author_id: i64 = row.get(0)?;
        return Ok(author_id);
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn get_or_create_author(conn: &Connection, autor: &Author) -> Result<i64> {
    match get_author_id_by_name(conn, &autor.name) {
        Ok(author_id) => Ok(author_id),
        Err(_) => add_author(conn, autor),
    }
}

pub fn get_all_authors(conn: &Connection) -> Result<Vec<Author>> {
    let mut stmt = conn.prepare("SELECT id, name, picture_path FROM authors")?;
    let author_iter = stmt.query_map([], |row| {
        Ok(Author {
            id: row.get(0)?,
            name: row.get(1)?,
            picture_path: row.get(2)?,
        })
    })?;

    let mut authors = Vec::new();
    for author in author_iter {
        authors.push(author?);
    }
    Ok(authors)
}

pub fn get_author_by_id(conn: &Connection, author_id: i64) -> Result<AuthorDetails> {
    let mut stmt = conn.prepare("SELECT authors.id, authors.name, authors.picture_path, books.id, books.title, books.cover_path FROM authors JOIN books ON authors.id = books.author_id WHERE authors.id = ?1")?;
    let mut rows = stmt.query(params![author_id])?;

    if let Some(row) = rows.next()? {
        let mut author = AuthorDetails {
            id: row.get(0)?,
            name: row.get(1)?,
            picture_path: row.get(2)?,
            books: Vec::new(),
        };

        let mut books = Vec::new();
        books.push(FrontendBook {
            id: row.get(3)?,
            title: row.get(4)?,
            cover_path: row.get(5)?,
            author_id: author.id,
            author_name: author.name.clone(),
        });

        author.books = books;
        return Ok(author);
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}


// pub fn get_authors(conn: &Connection) -> Result<Vec<Author>> {
//     let mut stmt = conn.prepare("SELECT id, name FROM authors")?;
//     let author_iter = stmt.query_map([], |row| {
//         Ok(Author {
//             id: row.get(0)?,
//             name: row.get(1)?,
//         })
//     })?;

//     let mut authors = Vec::new();
//     for author in author_iter {
//         authors.push(author?);
//     }
//     Ok(authors)
// }

// -------------------------
// Lector database functions
// -------------------------

pub fn get_or_create_lector(conn: &Connection, lector_name: &str) -> Result<i64> {
    let mut stmt = conn.prepare("SELECT id FROM lectors WHERE name = ?1")?;
    let mut rows = stmt.query(params![lector_name])?;

    if let Some(row) = rows.next()? {
        let lector_id: i64 = row.get(0)?;
        return Ok(lector_id);
    }

    conn.execute(
        "INSERT INTO lectors (name) VALUES (?1)",
        params![lector_name],
    )?;

    let lector_id = conn.last_insert_rowid();
    Ok(lector_id)
}

// -------------------------
// Genre database functions
// -------------------------

pub fn get_or_create_genre(conn: &Connection, genre_name: &str) -> Result<i64> {
    let mut stmt = conn.prepare("SELECT id FROM genres WHERE name = ?1")?;
    let mut rows = stmt.query(params![genre_name])?;

    if let Some(row) = rows.next()? {
        let genre_id: i64 = row.get(0)?;
        return Ok(genre_id);
    }

    conn.execute("INSERT INTO genres (name) VALUES (?1)", params![genre_name])?;

    let genre_id = conn.last_insert_rowid();
    Ok(genre_id)
}

// -------------------------
// Dashboard
// -------------------------

pub fn get_dashboard_data(conn: &Connection) -> Result<(DashboardData)> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM authors")?;
    let authors_amount: i64 = stmt.query_row([], |row| row.get(0))?;

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM genres")?;
    let genres_amount: i64 = stmt.query_row([], |row| row.get(0))?;

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM lectors")?;
    let lectors_amount: i64 = stmt.query_row([], |row| row.get(0))?;

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM books")?;
    let books_amount: i64 = stmt.query_row([], |row| row.get(0))?;

    Ok(DashboardData {
        authors_amount,
        genres_amount,
        lectors_amount,
        books_amount,
    })
}


// -------------------------
// TODO
// -------------------------

use std::fs;

use crate::structs::{Author, AuthorDetails, DBBook, DashboardData, FrontendBook, FrontendBookDetails};
// TODO: Change this function in the future
pub fn clear_db() -> Result<()> {
    match std::env::var("DATABASE_URL") {
        Ok(url) => fs::remove_file(url).ok(),
        Err(_) => fs::remove_file("finalshelf.db").ok()
    };
    Ok(())
}
