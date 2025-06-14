use crate::db::establish_connection;
use crate::models::book::Book;
use crate::schema::books;
use crate::schema::books::dsl::*;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection, TextExpressionMethods};
use std::collections::HashSet;

pub fn search(target: &str, by: &[String]) -> Vec<Book> {
    let conn = &mut establish_connection();
    let mut seen_ids = HashSet::new();
    let mut result = Vec::new();

    for by_field in by.iter().map(String::as_str).collect::<HashSet<_>>() {
        if let Ok(books_found) = search_by(conn, target, by_field) {
            for book in books_found {
                if seen_ids.insert(book.title.clone()) {
                    result.push(book);
                }
            }
        }
    }
    result
}

fn search_by(conn: &mut SqliteConnection, target: &str, by: &str) -> Result<Vec<Book>, diesel::result::Error> {
    let pattern = format!("%{}%", target);
    match by {
        "title" => books
            .filter(books::title.like(&pattern))
            .select(Book::as_select()) // <-- Add this
            .load::<Book>(conn),
        "author_name" => books
            .filter(books::author_name.like(&pattern))
            .select(Book::as_select()) // <-- Add this
            .load::<Book>(conn),
        "genre_name" => books
            .filter(books::genre.like(&pattern))
            .select(Book::as_select()) // <-- Add this
            .load::<Book>(conn),
        "lector_name" => books
            .filter(books::lector.like(&pattern))
            .select(Book::as_select()) // <-- Add this
            .load::<Book>(conn),
        _ => Ok(Vec::new()),
    }
}
