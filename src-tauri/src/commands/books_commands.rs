// get lists, get by ID, search, get dashboard data

use crate::{
    models::{
        book::{Book, BookListResponse},
        query::QueryParams,
    },
    services::books_service::{get_book, get_read_books, list_books, update_book},
};

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub async fn get_books_list_command(
    page: Option<i64>,
    limit: Option<i64>,
    sort_by: Option<String>,
    sort_order: Option<String>,
    author_name: Option<String>,
    genre: Option<String>,
    title: Option<String>,
    lector: Option<String>,
    read_status: Option<bool>,
) -> Result<BookListResponse, String> {
    let query_params = QueryParams {
        page,
        limit,
        author_name,
        genre,
        title,
        lector,
        sort_by,
        sort_order,
        read_status,
    };

    match list_books(query_params) {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[tauri::command]
pub async fn get_book_command(title: String) -> Option<Book> {
    get_book(&title)
}

#[tauri::command]
pub async fn get_all_read_books_command() -> Vec<Book> {
    get_read_books()
}

#[tauri::command]
pub async fn update_book_command(book: Book) -> Option<Book> {
    update_book(&book)
}
