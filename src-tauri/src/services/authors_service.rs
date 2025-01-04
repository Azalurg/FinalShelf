// Add author service

use crate::{
    db::establish_connection,
    models::{models::Author, query::QueryParams},
    schema::authors,
    schema::authors::dsl,
};

use diesel::prelude::*;

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
