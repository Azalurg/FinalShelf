use crate::{
    db::establish_connection,
    models::{
        author::{Author, AuthorWithBooks},
        query::QueryParams,
    },
    schema::{
        authors::{self, dsl},
        books::{self},
    },
};
use diesel::{dsl::count, prelude::*};

pub fn list_authors(query_params: QueryParams) -> Vec<Author> {
    let conn = &mut establish_connection();

    let query = dsl::authors.order_by(dsl::name.desc());

    query.load::<Author>(conn).expect("Error loading authors")
}

pub fn add_author(new_author: &Author) {
    let conn = &mut establish_connection();

    diesel::insert_into(authors::table)
        .values(new_author)
        .execute(conn)
        .expect("Error saving new author");
}

pub fn is_author_exists(name: &str) -> bool {
    let conn = &mut establish_connection();

    let query = dsl::authors.filter(dsl::name.eq(name));

    query.first::<Author>(conn).is_ok()
}

pub fn get_author(name: &str) -> Option<AuthorWithBooks> {
    let conn = &mut establish_connection();

    let author = dsl::authors
        .filter(dsl::name.eq(name))
        .first::<Author>(conn)
        .expect("Error loading author");

    let books = crate::services::books_service::get_books_by_author(&author.name);

    Some(AuthorWithBooks { author, books })
}

pub fn get_authors_count() -> i64 {
    let conn = &mut establish_connection();

    dsl::authors.count().get_result(conn).expect("Error counting authors")
}

pub fn get_top_authors(limit: i64) -> Vec<(Author, i64)> {
    let conn = &mut establish_connection();

    authors::table
        .inner_join(books::table.on(authors::name.eq(books::author_name)))
        .group_by(authors::name)
        .select((authors::all_columns, count(books::title))) // Changed here
        .order_by(count(books::title).desc()) // And here
        .limit(limit)
        .load::<(Author, i64)>(conn)
        .expect("Error loading top authors")
}
