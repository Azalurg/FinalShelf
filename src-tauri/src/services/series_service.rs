use crate::{
    db::establish_connection,
    models::{
        book::Book,
        series::{NewSeries, Series, SeriesListItem, SeriesWithBooks},
        query::{ListParams, ListResponse},
    },
    schema::{books, series},
};
use diesel::{dsl::count, prelude::*};

/// List all series with pagination, sorting, filtering, and search.
///
/// Filters (search, author) are applied at the SQL level to avoid loading
/// the full table. Sort and pagination are applied in Rust because `books_count`
/// is a computed aggregate column.
///
/// ## Sort fields
/// `"name"`, `"author_name"`, `"books_count"` (default: `"name"`)
pub fn list_series(params: &ListParams) -> Result<ListResponse<SeriesListItem>, String> {
    let conn = &mut establish_connection();

    let search_pattern = params.search_pattern();

    // Build the joined query with optional SQL-level filters.
    let mut query = series::table
        .left_join(books::table.on(series::id.nullable().eq(books::series_id)))
        .group_by(series::id)
        .select((
            series::id,
            series::name,
            series::author_name,
            series::description,
            count(books::title.nullable()),
        ))
        .into_boxed();

    if let Some(ref pattern) = search_pattern {
        query = query.filter(
            series::name
                .like(pattern)
                .or(series::author_name.like(pattern)),
        );
    }

    if let Some(ref author) = params.author_name {
        query = query.filter(series::author_name.eq(author.clone()));
    }

    let all_items: Vec<(i32, String, String, Option<String>, i64)> = query
        .load::<(i32, String, String, Option<String>, i64)>(conn)
        .map_err(|e| format!("Failed to load series: {}", e))?;

    let total_count = all_items.len() as i64;

    let mut items: Vec<SeriesListItem> = all_items
        .into_iter()
        .map(|(id, name, author_name, description, cnt)| SeriesListItem {
            id,
            name,
            author_name,
            description,
            books_count: cnt,
        })
        .collect();

    // Sort in Rust (needed for case-insensitive string sort and books_count)
    let sort_by = params.sort_field(&["name", "author_name", "books_count"], "name");
    let desc = params.is_desc();
    match sort_by.as_str() {
        "books_count" => items.sort_by(|a, b| {
            let cmp = a.books_count.cmp(&b.books_count);
            if desc { cmp.reverse() } else { cmp }
        }),
        "author_name" => items.sort_by(|a, b| {
            let cmp = a.author_name.to_lowercase().cmp(&b.author_name.to_lowercase());
            if desc { cmp.reverse() } else { cmp }
        }),
        _ => items.sort_by(|a, b| {
            let cmp = a.name.to_lowercase().cmp(&b.name.to_lowercase());
            if desc { cmp.reverse() } else { cmp }
        }),
    }

    // Paginate in Rust
    let limit = params.limit();
    let offset = params.offset() as usize;
    let page_items: Vec<SeriesListItem> = items
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

/// Get a single series by ID with its associated books (ordered by series_order).
pub fn get_series(id: i32) -> Result<SeriesWithBooks, String> {
    let conn = &mut establish_connection();

    let series_record = series::table
        .find(id)
        .first::<Series>(conn)
        .map_err(|e| format!("Series not found: {}", e))?;

    let series_books = books::table
        .filter(books::series_id.eq(id))
        .order((
            books::series_order.is_null().asc(),
            books::series_order.asc(),
            books::title.asc(),
        ))
        .load::<Book>(conn)
        .map_err(|e| format!("Failed to load series books: {}", e))?;

    Ok(SeriesWithBooks {
        series: series_record,
        books: series_books,
    })
}

/// Create a new series.
pub fn create_series(new_series: NewSeries) -> Result<Series, String> {
    let conn = &mut establish_connection();

    diesel::insert_into(series::table)
        .values(&new_series)
        .execute(conn)
        .map_err(|e| format!("Failed to create series: {}", e))?;

    series::table
        .filter(series::name.eq(&new_series.name))
        .filter(series::author_name.eq(&new_series.author_name))
        .first::<Series>(conn)
        .map_err(|e| format!("Failed to retrieve created series: {}", e))
}

/// Update series details.
pub fn update_series(id: i32, name: Option<String>, description: Option<String>) -> Result<Series, String> {
    let conn = &mut establish_connection();

    if let Some(ref new_name) = name {
        diesel::update(series::table.find(id))
            .set(series::name.eq(new_name))
            .execute(conn)
            .map_err(|e| format!("Failed to update series name: {}", e))?;
    }

    if let Some(ref new_desc) = description {
        diesel::update(series::table.find(id))
            .set(series::description.eq(new_desc))
            .execute(conn)
            .map_err(|e| format!("Failed to update series description: {}", e))?;
    }

    series::table
        .find(id)
        .first::<Series>(conn)
        .map_err(|e| format!("Series not found: {}", e))
}

/// Delete a series (books will have their series_id set to NULL due to ON DELETE SET NULL).
pub fn delete_series(id: i32) -> Result<(), String> {
    let conn = &mut establish_connection();

    diesel::delete(series::table.find(id))
        .execute(conn)
        .map_err(|e| format!("Failed to delete series: {}", e))?;

    Ok(())
}

/// Assign a book to a series with a specific order.
/// Note: If series_id is None, series_order will be set to None to maintain consistency.
pub fn assign_book_to_series(book_title: &str, series_id: Option<i32>, series_order: Option<i32>) -> Result<(), String> {
    let conn = &mut establish_connection();

    // Ensure series_order is None when series_id is None to prevent inconsistent state
    let effective_series_order = if series_id.is_some() { series_order } else { None };

    let affected = diesel::update(books::table.find(book_title))
        .set((
            books::series_id.eq(series_id),
            books::series_order.eq(effective_series_order),
        ))
        .execute(conn)
        .map_err(|e| format!("Failed to assign book to series: {}", e))?;

    if affected == 0 {
        return Err(format!("Book '{}' not found or no changes applied", book_title));
    }

    Ok(())
}

/// Get series count for dashboard.
pub fn get_series_count() -> Result<i64, String> {
    let conn = &mut establish_connection();

    series::table
        .count()
        .get_result::<i64>(conn)
        .map_err(|e| format!("Failed to count series: {}", e))
}

/// Get series by author.
pub fn get_series_by_author(author_name: &str) -> Result<Vec<Series>, String> {
    let conn = &mut establish_connection();

    series::table
        .filter(series::author_name.eq(author_name))
        .order(series::name.asc())
        .load::<Series>(conn)
        .map_err(|e| format!("Failed to load series for author: {}", e))
}
