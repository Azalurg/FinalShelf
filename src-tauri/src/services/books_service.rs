use crate::{
    db::establish_connection,
    models::{
        book::{Book, BookListResponse},
        query::QueryParams,
    },
    schema::books::{self, dsl},
};
use diesel::{dsl::count_star, prelude::*};

pub fn get_book(title: &str) -> Option<Book> {
    let conn = &mut establish_connection();
    dsl::books
        .filter(dsl::title.eq(title))
        .select(Book::as_select())
        .first::<Book>(conn)
        .ok()
}

pub fn list_books(query_params: QueryParams) -> Result<BookListResponse, diesel::result::Error> {
    let conn = &mut establish_connection();

    let page = query_params.page.unwrap_or(1).max(1);
    let limit = query_params.limit.unwrap_or(21).clamp(1, 100);
    let offset = (page - 1) * limit;

    // Building the base query with filters
    let mut query = dsl::books.into_boxed();
    let mut count_query = dsl::books.into_boxed();

    if let Some(ref author) = query_params.author_name {
        query = query.filter(dsl::author_name.like(format!("%{}%", author)));
        count_query = count_query.filter(dsl::author_name.like(format!("%{}%", author)));
    }
    if let Some(ref genre_filter) = query_params.genre {
        query = query.filter(dsl::genre.like(format!("%{}%", genre_filter)));
        count_query = count_query.filter(dsl::genre.like(format!("%{}%", genre_filter)));
    }
    if let Some(ref title_filter) = query_params.title {
        query = query.filter(dsl::title.like(format!("%{}%", title_filter)));
        count_query = count_query.filter(dsl::title.like(format!("%{}%", title_filter)));
    }
    if let Some(ref lector_filter) = query_params.lector {
        query = query.filter(dsl::lector.like(format!("%{}%", lector_filter)));
        count_query = count_query.filter(dsl::lector.like(format!("%{}%", lector_filter)));
    }
    if let Some(read_filter) = query_params.read_status {
        query = query.filter(dsl::read.eq(read_filter));
        count_query = count_query.filter(dsl::read.eq(read_filter));
    }

    // Apply sorting
    let mut data_query = query; // Use the filtered query as the base for data
    if let Some(ref sort_field) = query_params.sort_by {
        let order = query_params.sort_order.as_deref().unwrap_or("asc");
        data_query = match (sort_field.as_str(), order) {
            ("title", "desc") => data_query.order(dsl::title.desc()),
            ("title", _) => data_query.order(dsl::title.asc()),
            ("author", "desc") => data_query.order((dsl::author_name.desc(), dsl::title.asc())),
            ("author", _) => data_query.order((dsl::author_name.asc(), dsl::title.asc())),
            ("create_date", "desc") => data_query.order(dsl::create_date.desc()),
            ("create_date", _) => data_query.order(dsl::create_date.asc()),
            ("score", "desc") => data_query.order(dsl::score.desc()),
            ("score", _) => data_query.order(dsl::score.asc()),
            _ => data_query.order(dsl::author_name.asc()),
        };
    } else {
        data_query = data_query.order(dsl::author_name.asc());
    }

    let books = data_query
        .limit(limit)
        .offset(offset)
        .select(Book::as_select())
        .load::<Book>(conn)?;

    let total_count = count_query.select(count_star()).first(conn)?;
    let total_pages = (total_count + limit - 1) / limit;

    Ok(BookListResponse {
        books,
        total_count,
        page,
        limit,
        total_pages,
    })
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

    dsl::books
        .filter(dsl::title.eq(title))
        .select(Book::as_select())
        .first::<Book>(conn)
        .is_ok()
}

pub fn get_books_by_author(author_name: &str) -> Vec<Book> {
    let conn = &mut establish_connection();

    dsl::books
        .filter(dsl::author_name.eq(author_name))
        .select(Book::as_select())
        .load::<Book>(conn)
        .expect("Error loading books")
}

pub fn get_books_by_genre(genre: &str) -> Vec<Book> {
    let conn = &mut establish_connection();

    dsl::books
        .filter(dsl::genre.eq(genre))
        .select(Book::as_select())
        .load::<Book>(conn)
        .expect("Error loading books")
}

pub fn get_books_by_lector(lector: &str) -> Vec<Book> {
    let conn = &mut establish_connection();

    dsl::books
        .filter(dsl::lector.eq(lector))
        .select(Book::as_select())
        .load::<Book>(conn)
        .expect("Error loading books")
}

pub fn get_read_books() -> Vec<Book> {
    let conn = &mut establish_connection();

    dsl::books
        .filter(dsl::read.eq(true))
        .select(Book::as_select())
        .load::<Book>(conn)
        .expect("Error loading books")
}

pub fn update_book(book: &Book) -> Option<Book> {
    let conn = &mut establish_connection();

    diesel::update(books::table.find(&book.title))
        .set(book)
        .execute(conn)
        .expect("Error updating book");

    get_book(&book.title)
}

pub fn get_books_count() -> i64 {
    let conn = &mut establish_connection();

    dsl::books.count().get_result(conn).expect("Error counting books")
}

pub fn get_read_books_count() -> i64 {
    let conn = &mut establish_connection();

    dsl::books
        .filter(dsl::read.eq(true))
        .count()
        .get_result(conn)
        .expect("Error counting read books")
}

pub fn get_books_by_date(limit: i64) -> Vec<Book> {
    let conn = &mut establish_connection();

    dsl::books
        .order((dsl::create_date.desc(), dsl::author_name.asc(), dsl::title.asc()))
        .limit(limit)
        .select(Book::as_select())
        .load::<Book>(conn)
        .expect("Error getting books by date")
}

pub fn get_books_by_score(limit: i64) -> Vec<Book> {
    let conn = &mut establish_connection();

    dsl::books
        .order((dsl::score.asc(), dsl::author_name.asc(), dsl::title.asc()))
        .limit(limit)
        .select(Book::as_select())
        .load::<Book>(conn)
        .expect("Error getting books by score")
}

pub fn get_all_book_paths() -> Result<Vec<String>, diesel::result::Error> {
    let conn = &mut establish_connection();

    dsl::books
        .select(dsl::relative_file_path)
        .load::<String>(conn)
        .map_err(|e| e.into())
}
