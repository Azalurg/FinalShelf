use crate::{
    db::establish_connection,
    models::{
        author::{Author, AuthorListItem, AuthorWithBooks},
        book::Book,
        query::{ListParams, ListResponse},
    },
    schema::{authors, books},
};
use diesel::{dsl::count, prelude::*};

/// Unified author listing with pagination, sorting, filtering, and search.
///
/// Every author is returned with its `books_count` (via INNER JOIN).
/// Authors with zero books are excluded.
///
/// ## Sort fields
/// `"name"`, `"books_count"` (default: `"name"`)
pub fn list_authors(params: &ListParams) -> Result<ListResponse<AuthorListItem>, String> {
    let conn = &mut establish_connection();

    // Load all authors with book counts via INNER JOIN + GROUP BY.
    // Sorting and pagination are done in Rust because Diesel's boxed queries
    // don't compose well with JOIN + GROUP BY + dynamic ORDER BY.
    let mut items: Vec<AuthorListItem> = authors::table
        .inner_join(books::table.on(authors::name.eq(books::author_name)))
        .group_by(authors::name)
        .select((authors::all_columns, count(books::title)))
        .load::<(Author, i64)>(conn)
        .map_err(|e| format!("Failed to load authors: {}", e))?
        .into_iter()
        .map(|(author, cnt)| AuthorListItem {
            name: author.name,
            relative_img_path: author.relative_img_path,
            books_count: cnt,
        })
        .collect();

    // Search filter (case-insensitive substring match on name)
    if let Some(ref search) = params.search {
        let lower = search.to_lowercase();
        items.retain(|a| a.name.to_lowercase().contains(&lower));
    }

    // Sort
    let sort_by = params.sort_field(&["name", "books_count"], "name");
    let desc = params.is_desc();
    match sort_by.as_str() {
        "books_count" => items.sort_by(|a, b| {
            let cmp = a.books_count.cmp(&b.books_count);
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
    let page_items: Vec<AuthorListItem> = items
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

pub fn get_author(name: &str) -> Result<AuthorWithBooks, String> {
    let conn = &mut establish_connection();

    let author = authors::dsl::authors
        .filter(authors::dsl::name.eq(name))
        .first::<Author>(conn)
        .map_err(|_| format!("Author '{}' not found", name))?;

    let author_books = books::dsl::books
        .filter(books::dsl::author_name.eq(name))
        .select(Book::as_select())
        .load::<Book>(conn)
        .map_err(|e| format!("Failed to load books for author: {}", e))?;

    Ok(AuthorWithBooks {
        author,
        books: author_books,
    })
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
    authors::dsl::authors
        .filter(authors::dsl::name.eq(name))
        .first::<Author>(conn)
        .is_ok()
}
