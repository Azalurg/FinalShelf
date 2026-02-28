use crate::{
    db::establish_connection,
    models::{
        book::Book,
        genre::{Genre, GenreWithBooks},
        query::{ListParams, ListResponse},
    },
    schema::books::dsl,
};
use diesel::{dsl::count, prelude::*};

/// Unified genre listing with pagination, sorting, and search.
///
/// Genres are virtual entities derived from `GROUP BY books.genre`.
///
/// ## Sort fields
/// `"name"`, `"books_count"` (default: `"name"`)
pub fn list_genres(params: &ListParams) -> Result<ListResponse<Genre>, String> {
    let conn = &mut establish_connection();

    let mut items: Vec<Genre> = dsl::books
        .filter(dsl::genre.is_not_null())
        .group_by(dsl::genre)
        .select((dsl::genre, count(dsl::genre)))
        .load::<(Option<String>, i64)>(conn)
        .map_err(|e| format!("Failed to load genres: {}", e))?
        .into_iter()
        .filter_map(|(name, cnt)| {
            name.map(|n| Genre {
                name: n,
                books_amount: cnt,
            })
        })
        .collect();

    // Search filter
    if let Some(ref search) = params.search {
        let lower = search.to_lowercase();
        items.retain(|g| g.name.to_lowercase().contains(&lower));
    }

    // Sort
    let sort_by = params.sort_field(&["name", "books_count"], "name");
    let desc = params.is_desc();
    match sort_by.as_str() {
        "books_count" => items.sort_by(|a, b| {
            let cmp = a.books_amount.cmp(&b.books_amount);
            if desc {
                cmp.reverse()
            } else {
                cmp
            }
        }),
        _ => items.sort_by(|a, b| {
            let cmp = a.name.to_lowercase().cmp(&b.name.to_lowercase());
            if desc {
                cmp.reverse()
            } else {
                cmp
            }
        }),
    }

    // Paginate
    let total_count = items.len() as i64;
    let limit = params.limit();
    let offset = params.offset() as usize;
    let page_items: Vec<Genre> = items
        .into_iter()
        .skip(offset)
        .take(limit as usize)
        .collect();

    Ok(ListResponse::new(
        page_items,
        total_count,
        params.page(),
        limit,
    ))
}

pub fn get_genre(name: &str) -> Result<GenreWithBooks, String> {
    let conn = &mut establish_connection();

    let genre_books = dsl::books
        .filter(dsl::genre.eq(name))
        .select(Book::as_select())
        .load::<Book>(conn)
        .map_err(|e| format!("Failed to load books for genre: {}", e))?;

    if genre_books.is_empty() {
        return Err(format!("Genre '{}' not found", name));
    }

    let books_amount = genre_books.len() as i64;

    Ok(GenreWithBooks {
        name: name.to_string(),
        books: genre_books,
        books_amount,
    })
}
