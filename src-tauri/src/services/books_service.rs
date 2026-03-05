use crate::{
    db::establish_connection,
    models::{
        book::Book,
        query::{ListParams, ListResponse},
    },
    schema::books::{self, dsl},
};
use diesel::{dsl::count_star, prelude::*};

/// Unified book listing with pagination, sorting, filtering, and free-text search.
///
/// ## Filters (exact match)
/// - `author_name` — match `books.author_name`
/// - `genre` — match `books.genre`
/// - `lector` — match `books.lector`
/// - `read_status` — match `books.read`
/// - `min_score` — `books.score >= value`
///
/// ## Search (fuzzy)
/// - `search` — LIKE across title, author_name, genre, lector
///
/// ## Sort fields
/// `"title"`, `"author"`, `"create_date"`, `"score"`, `"duration"` (default: `"title"`)
pub fn list_books(params: &ListParams) -> Result<ListResponse<Book>, String> {
    let conn = &mut establish_connection();
    let limit = params.limit();
    let offset = params.offset();
    let search_pattern = params.search_pattern();

    let mut query = dsl::books.into_boxed();
    let mut count_query = dsl::books.into_boxed();

    // --- Exact-match filters -------------------------------------------------

    if let Some(ref author) = params.author_name {
        query = query.filter(dsl::author_name.eq(author));
        count_query = count_query.filter(dsl::author_name.eq(author));
    }
    if let Some(ref genre_filter) = params.genre {
        query = query.filter(dsl::genre.eq(genre_filter));
        count_query = count_query.filter(dsl::genre.eq(genre_filter));
    }
    if let Some(ref lector_filter) = params.lector {
        query = query.filter(dsl::lector.eq(lector_filter));
        count_query = count_query.filter(dsl::lector.eq(lector_filter));
    }
    if let Some(read_filter) = params.read_status {
        query = query.filter(dsl::read.eq(read_filter));
        count_query = count_query.filter(dsl::read.eq(read_filter));
    }
    if let Some(min_score) = params.min_score {
        query = query.filter(dsl::score.ge(min_score));
        count_query = count_query.filter(dsl::score.ge(min_score));
    }

    // --- Free-text search ----------------------------------------------------

    if let Some(ref pattern) = search_pattern {
        let search_title = params.should_search_field("title");
        let search_author = params.should_search_field("author");
        let search_genre = params.should_search_field("genre");
        let search_lector = params.should_search_field("lector");

        // Build OR chain only for requested fields.
        // At least one must be true (guaranteed: if search_fields is empty, all are true).
        let condition = dsl::title
            .like(pattern)
            .and(diesel::dsl::sql::<diesel::sql_types::Bool>(if search_title { "1" } else { "0" }))
            .or(dsl::author_name
                .like(pattern)
                .and(diesel::dsl::sql::<diesel::sql_types::Bool>(if search_author { "1" } else { "0" })))
            .or(dsl::genre
                .like(pattern)
                .and(diesel::dsl::sql::<diesel::sql_types::Bool>(if search_genre { "1" } else { "0" })))
            .or(dsl::lector
                .like(pattern)
                .and(diesel::dsl::sql::<diesel::sql_types::Bool>(if search_lector { "1" } else { "0" })));

        query = query.filter(condition);

        let condition2 = dsl::title
            .like(pattern)
            .and(diesel::dsl::sql::<diesel::sql_types::Bool>(if search_title { "1" } else { "0" }))
            .or(dsl::author_name
                .like(pattern)
                .and(diesel::dsl::sql::<diesel::sql_types::Bool>(if search_author { "1" } else { "0" })))
            .or(dsl::genre
                .like(pattern)
                .and(diesel::dsl::sql::<diesel::sql_types::Bool>(if search_genre { "1" } else { "0" })))
            .or(dsl::lector
                .like(pattern)
                .and(diesel::dsl::sql::<diesel::sql_types::Bool>(if search_lector { "1" } else { "0" })));

        count_query = count_query.filter(condition2);
    }

    // --- Sorting -------------------------------------------------------------

    let sort_field = params.sort_field(
        &["title", "author", "create_date", "score", "duration"],
        "title",
    );
    let desc = params.is_desc();

    query = match (sort_field.as_str(), desc) {
        ("title", true) => query.order(dsl::title.desc()),
        ("title", false) => query.order(dsl::title.asc()),
        ("author", true) => query.order((dsl::author_name.desc(), dsl::title.asc())),
        ("author", false) => query.order((dsl::author_name.asc(), dsl::title.asc())),
        ("create_date", true) => query.order(dsl::create_date.desc()),
        ("create_date", false) => query.order(dsl::create_date.asc()),
        ("score", true) => query.order((dsl::score.desc(), dsl::title.asc())),
        ("score", false) => query.order((dsl::score.asc(), dsl::title.asc())),
        ("duration", true) => query.order((dsl::duration_seconds.desc(), dsl::title.asc())),
        ("duration", false) => query.order((dsl::duration_seconds.asc(), dsl::title.asc())),
        _ => query.order(dsl::title.asc()),
    };

    // --- Execute -------------------------------------------------------------

    let items = query
        .limit(limit)
        .offset(offset)
        .select(Book::as_select())
        .load::<Book>(conn)
        .map_err(|e| format!("Failed to load books: {}", e))?;

    let total_count: i64 = count_query
        .select(count_star())
        .first(conn)
        .map_err(|e| format!("Failed to count books: {}", e))?;

    Ok(ListResponse::new(items, total_count, params.page(), limit))
}

pub fn get_book(title: &str) -> Result<Book, String> {
    let conn = &mut establish_connection();
    dsl::books
        .filter(dsl::title.eq(title))
        .select(Book::as_select())
        .first::<Book>(conn)
        .map_err(|e| format!("Book '{}' not found: {}", title, e))
}

pub fn add_book(new_book: &Book) -> Result<Book, String> {
    let conn = &mut establish_connection();
    diesel::insert_into(books::table)
        .values(new_book)
        .execute(conn)
        .map_err(|e| format!("Failed to save book: {}", e))?;
    get_book(&new_book.title)
}

pub fn is_book_exists(title: &str) -> bool {
    let conn = &mut establish_connection();
    dsl::books
        .filter(dsl::title.eq(title))
        .select(Book::as_select())
        .first::<Book>(conn)
        .is_ok()
}

pub fn update_book(book: &Book) -> Result<Book, String> {
    let conn = &mut establish_connection();
    diesel::update(books::table.find(&book.title))
        .set(book)
        .execute(conn)
        .map_err(|e| format!("Failed to update book: {}", e))?;
    get_book(&book.title)
}

/// Returns all relative file paths in the database (used by the scanner to
/// skip already-imported directories during quick scans).
pub fn get_all_book_paths() -> Result<Vec<String>, diesel::result::Error> {
    let conn = &mut establish_connection();
    dsl::books
        .select(dsl::relative_file_path)
        .load::<String>(conn)
}
