use crate::{
    db::establish_connection,
    models::{
        book::Book,
        genre::{Genre, GenreWithBooks},
    },
    schema::books::dsl,
};
use diesel::dsl::count;
use diesel::prelude::*;

pub fn get_genres_list() -> Vec<Genre> {
    let conn = &mut establish_connection();

    // Construct the query
    let query = dsl::books
        .filter(dsl::genre.is_not_null())
        .group_by(dsl::genre)
        .select((dsl::genre, count(dsl::genre)))
        .order(count(dsl::genre).desc());

    // Execute the query and map the results to the Genre struct
    query
        .load::<(Option<String>, i64)>(conn)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(genre_name, books_count)| {
            genre_name.map(|name| Genre {
                name,
                books_amount: books_count,
            })
        })
        .collect()
}

pub fn get_genre(name: String) -> Option<GenreWithBooks> {
    let conn = &mut establish_connection();

    let books = dsl::books
        .filter(dsl::genre.eq(name.clone()))
        .load::<Book>(conn)
        .expect("Error loading books");

    if books.is_empty() {
        return None;
    }

    let books_amount = books.len() as i64;

    Some(GenreWithBooks {
        name,
        books,
        books_amount,
    })
}
