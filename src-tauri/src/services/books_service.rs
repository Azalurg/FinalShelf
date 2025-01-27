use crate::{
    db::establish_connection,
    models::{book::Book, query::QueryParams},
    schema::books,
    schema::books::dsl,
};
use diesel::prelude::*;

pub fn get_book(title: &str) -> Option<Book> {
    let conn = &mut establish_connection();

    let query = dsl::books.filter(dsl::title.eq(title));

    query.first::<Book>(conn).ok()
}

pub fn list_books(query_params: QueryParams) -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = dsl::books.order_by(dsl::author_name.desc());

    query.load::<Book>(conn).expect("Error loading books")
}

pub fn add_book(new_book: &Book) -> Option<Book> {
    let conn = &mut establish_connection();

    diesel::insert_into(books::table)
        .values(new_book)
        .execute(conn)
        .expect("Error saving new book");

    get_book(&new_book.title)
}

pub fn is_book_exists(title: &str) -> bool {
    let conn = &mut establish_connection();

    let query = dsl::books.filter(dsl::title.eq(title));

    query.first::<Book>(conn).is_ok()
}

pub fn get_books_by_author(author_name: &str) -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = dsl::books.filter(dsl::author_name.eq(author_name));

    query.load::<Book>(conn).expect("Error loading books")
}

pub fn get_books_by_genre(genre: &str) -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = dsl::books.filter(dsl::genre.eq(genre));

    query.load::<Book>(conn).expect("Error loading books")
}

pub fn get_books_by_lector(lector: &str) -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = dsl::books.filter(dsl::lector.eq(lector));

    query.load::<Book>(conn).expect("Error loading books")
}
