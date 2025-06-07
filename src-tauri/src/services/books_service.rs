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

    let query = dsl::books.filter(dsl::title.eq(title));

    query.first::<Book>(conn).ok()
}

pub fn list_books(query_params: QueryParams) -> Result<BookListResponse, diesel::result::Error> {
    let conn = &mut establish_connection();

    let page = query_params.page.unwrap_or(1).max(1);
    let limit = query_params.limit.unwrap_or(21).clamp(1, 100);
    let offset = (page - 1) * limit;

    let mut query = dsl::books.into_boxed();
    let mut count_query = dsl::books.into_boxed();

    // Declare pattern variables outside the closures to extend their lifetime
    if let Some(ref author) = query_params.author_name {
        let author_pattern = format!("%{}%", author);
        query = query.filter(dsl::author_name.like(author_pattern.clone()));
        count_query = count_query.filter(dsl::author_name.like(author_pattern));
    }

    if let Some(ref genre_filter) = query_params.genre {
        let genre_pattern = format!("%{}%", genre_filter);
        query = query.filter(dsl::genre.like(genre_pattern.clone()));
        count_query = count_query.filter(dsl::genre.like(genre_pattern));
    }

    if let Some(ref title_filter) = query_params.title {
        let title_pattern = format!("%{}%", title_filter);
        query = query.filter(dsl::title.like(title_pattern.clone()));
        count_query = count_query.filter(dsl::title.like(title_pattern));
    }

    if let Some(ref lector_filter) = query_params.lector {
        let lector_pattern = format!("%{}%", lector_filter);
        query = query.filter(dsl::lector.like(lector_pattern.clone()));
        count_query = count_query.filter(dsl::lector.like(lector_pattern));
    }

    if let Some(read_filter) = query_params.read_status {
        query = query.filter(dsl::read.eq(read_filter));
        count_query = count_query.filter(dsl::read.eq(read_filter));
    }

    // Apply sorting
    if let Some(ref sort_field) = query_params.sort_by {
        let order = query_params.sort_order.as_deref().unwrap_or("asc");

        query = match sort_field.as_str() {
            "title" => {
                if order == "desc" {
                    query.order(dsl::title.desc())
                } else {
                    query.order(dsl::title.asc())
                }
            },
            "author_name" => {
                if order == "desc" {
                    query.order(dsl::author_name.desc())
                } else {
                    query.order(dsl::author_name.asc())
                }
            },
            "create_date" => {
                if order == "desc" {
                    query.order(dsl::create_date.desc())
                } else {
                    query.order(dsl::create_date.asc())
                }
            },
            "score" => {
                if order == "desc" {
                    query.order(dsl::score.desc())
                } else {
                    query.order(dsl::score.asc())
                }
            },
            _ => query.order(dsl::author_name.asc()),
        };
    } else {
        query = query.order(dsl::author_name.asc());
    }

    // Get total count first
    let total_count: i64 = count_query.select(count_star()).first(conn)?;

    // Calculate total pages
    let total_pages = (total_count + limit - 1) / limit;

    // Get books with pagination
    let books = query.limit(limit).offset(offset).load::<Book>(conn)?;

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

    let query = dsl::books.filter(dsl::title.eq(title));

    query.first::<Book>(conn).is_ok()
}

pub fn get_books_by_author(author_name: &str) -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = dsl::books.filter(dsl::author_name.eq(author_name));

    query.load::<Book>(conn).expect("Error loading books")
}

pub fn get_books_by_genre(genre: &str) -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = dsl::books.filter(dsl::genre.eq(genre));

    query.load::<Book>(conn).expect("Error loading books")
}

pub fn get_books_by_lector(lector: &str) -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = dsl::books.filter(dsl::lector.eq(lector));

    query.load::<Book>(conn).expect("Error loading books")
}

pub fn get_read_books() -> Vec<Book> {
    let conn = &mut establish_connection();

    let query = dsl::books.filter(dsl::read.eq(true));

    query.load::<Book>(conn).expect("Error loading books")
}

pub fn update_book(book: &Book) -> Option<Book> {
    let conn = &mut establish_connection();

    diesel::update(books::table.find(&book.title)) // More efficient than filter
        .set(book)
        .execute(conn)
        .expect("Error updating book");

    get_book(&book.title)
}
