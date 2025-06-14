use crate::{
    db::establish_connection,
    models::{
        book::Book,
        lector::{Lector, LectorWithBooks},
    },
    schema::books::dsl,
};
use diesel::dsl::count;
use diesel::prelude::*;

pub fn get_lectors_list() -> Vec<Lector> {
    let conn = &mut establish_connection();

    // Construct the query
    let query = dsl::books
        .filter(dsl::lector.is_not_null())
        .group_by(dsl::lector)
        .select((dsl::lector, count(dsl::lector)))
        .order(count(dsl::lector).desc());

    // Execute the query and map the results to the Lector struct
    query
        .load::<(Option<String>, i64)>(conn)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(lector_name, books_count)| {
            lector_name.map(|name| Lector {
                name,
                books_amount: books_count,
            })
        })
        .collect()
}

pub fn get_lector(name: String) -> Option<LectorWithBooks> {
    let conn = &mut establish_connection();

    let books = dsl::books
        .filter(dsl::lector.eq(name.clone()))
        .select(Book::as_select())
        .load::<Book>(conn)
        .expect("Error loading books");

    if books.is_empty() {
        return None;
    }

    let books_amount = books.len() as i64;

    Some(LectorWithBooks {
        name,
        books,
        books_amount,
    })
}
