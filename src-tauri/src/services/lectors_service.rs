use crate::{
    db::establish_connection,
    models::{
        book::Book,
        lector::{Lector, LectorWithBooks},
        query::{ListParams, ListResponse},
    },
    schema::books::dsl,
};
use diesel::{dsl::count, prelude::*};

/// Unified lector listing with pagination, sorting, and search.
///
/// Lectors are virtual entities derived from `GROUP BY books.lector`.
///
/// ## Sort fields
/// `"name"`, `"books_count"` (default: `"name"`)
pub fn list_lectors(params: &ListParams) -> Result<ListResponse<Lector>, String> {
    let conn = &mut establish_connection();

    let mut items: Vec<Lector> = dsl::books
        .filter(dsl::lector.is_not_null())
        .group_by(dsl::lector)
        .select((dsl::lector, count(dsl::lector)))
        .load::<(Option<String>, i64)>(conn)
        .map_err(|e| format!("Failed to load lectors: {}", e))?
        .into_iter()
        .filter_map(|(name, cnt)| {
            name.map(|n| Lector {
                name: n,
                books_amount: cnt,
            })
        })
        .collect();

    // Search filter
    if let Some(ref search) = params.search {
        let lower = search.to_lowercase();
        items.retain(|l| l.name.to_lowercase().contains(&lower));
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
    let page_items: Vec<Lector> = items
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

pub fn get_lector(name: &str) -> Result<LectorWithBooks, String> {
    let conn = &mut establish_connection();

    let lector_books = dsl::books
        .filter(dsl::lector.eq(name))
        .select(Book::as_select())
        .load::<Book>(conn)
        .map_err(|e| format!("Failed to load books for lector: {}", e))?;

    if lector_books.is_empty() {
        return Err(format!("Lector '{}' not found", name));
    }

    let books_amount = lector_books.len() as i64;

    Ok(LectorWithBooks {
        name: name.to_string(),
        books: lector_books,
        books_amount,
    })
}
