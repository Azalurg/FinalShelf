// get single author by name, get all authors

use crate::{
    models::{
        author::{Author, AuthorWithBooks},
        query::QueryParams,
    },
    services::authors_service::{get_author, list_authors},
};

#[tauri::command]
pub async fn get_authors_list_command(
    sort_by: Option<String>,
    sort_order: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Vec<Author> {
    let query_params = QueryParams {
        page,
        limit: page_size,
        author_name: None,
        genre: None,
        title: None,
        lector: None,
        sort_by,
        sort_order,
        read_status: None,
    };
    list_authors(query_params)
}

#[tauri::command]
pub async fn get_author_command(name: String) -> Option<AuthorWithBooks> {
    get_author(&name)
}
