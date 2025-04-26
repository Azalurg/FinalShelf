use std::collections::HashSet;

use diesel::{QueryDsl, RunQueryDsl, TextExpressionMethods};

use crate::db::establish_connection;
use crate::models::book::Book;
use crate::schema::books::dsl::books as books_schema;

pub fn search(target: String, by: Vec<String>) -> Vec<Book> {
    let by_set = by.into_iter().collect::<HashSet<String>>();
    let mut result = Vec::new();
    for by in by_set {
        result.append(&mut search_by(&target, by));
    }
    result
}

fn search_by(target: &String, by: String) -> Vec<Book> {
    match by.as_str() {
        "title" => search_by_title(target),
        "author_name" => search_by_author_name(target),
        "genre_name" => search_by_genre_name(target),
        "lector_name" => search_by_lector_name(target),
        _ => Vec::new(),
    }
}

fn search_by_title(target: &String) -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = books_schema.filter(crate::schema::books::dsl::title.like(format!("%{}%", target)));

    query.load::<Book>(conn).expect("Error loading books")
}

fn search_by_author_name(target: &String) -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = books_schema.filter(crate::schema::books::dsl::author_name.like(format!("%{}%", target)));

    query.load::<Book>(conn).expect("Error loading books")
}

fn search_by_genre_name(target: &String) -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = books_schema.filter(crate::schema::books::dsl::genre.like(format!("%{}%", target)));

    query.load::<Book>(conn).expect("Error loading books")
}

fn search_by_lector_name(target: &String) -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = books_schema.filter(crate::schema::books::dsl::lector.like(format!("%{}%", target)));

    query.load::<Book>(conn).expect("Error loading books")
}
