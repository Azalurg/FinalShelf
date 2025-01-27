// get lists, get by ID, search, get dashboard data

use crate::{
    models::{book::Book, query::QueryParams},
    services::books_service::{get_book, list_books},
};

#[tauri::command]
pub async fn get_books_list_command(
    filter_author: Option<String>,
    filter_genre: Option<String>,
    sort_by: Option<String>,
    sort_order: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Vec<Book> {
    let query_params = QueryParams::new(filter_author, filter_genre, sort_by, sort_order, page, page_size);

    list_books(query_params)
}

#[tauri::command]
pub async fn get_book_command(title: String) -> Option<Book> {
    get_book(&title)
}
