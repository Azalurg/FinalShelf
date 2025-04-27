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
    use crate::schema::books::dsl;

    let conn = &mut establish_connection();

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let mut query = dsl::books.into_boxed();

    // Apply sorting dynamically
    if let Some(sort_field) = query_params.sort_by {
        let order = query_params.sort_order.unwrap_or("asc".to_string());

        query = match sort_field.as_str() {
            "title" => {
                if order == "desc" {
                    query.order(dsl::title.desc())
                } else {
                    query.order(dsl::title.asc())
                }
            },
            "author_name" => {
                if order == "desc" {
                    query.order(dsl::author_name.desc())
                } else {
                    query.order(dsl::author_name.asc())
                }
            },
            "create_date" => {
                if order == "desc" {
                    query.order(dsl::create_date.desc())
                } else {
                    query.order(dsl::create_date.asc())
                }
            },
            _ => query.order(dsl::author_name.asc()), // Default sorting
        };
    }

    let books = query
        .limit(limit)
        .offset(offset)
        .load::<Book>(conn)
        .expect("Error loading books");

    books
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

pub fn get_read_books() -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = dsl::books.filter(dsl::read.eq(true));

    query.load::<Book>(conn).expect("Error loading books")
}

pub fn update_book(book: &Book) -> Option<Book> {
    let conn = &mut establish_connection();

    diesel::update(books::table.find(&book.title)) // More efficient than filter
        .set(book)
        .execute(conn)
        .expect("Error updating book");

    get_book(&book.title)
}
