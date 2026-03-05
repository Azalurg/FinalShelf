use crate::{
    models::{
        book::Book,
        query::{ListParams, ListResponse},
    },
    services::books_service,
};

/// Search for books by free-text query across title, author, genre, and lector.
///
/// This is a convenience wrapper around the books list endpoint that sets
/// the `search` field and optionally filters to specific fields via `by`.
///
/// Accepted values for `by`: `"title"`, `"author"`, `"genre"`, `"lector"`.
/// If `by` is empty, searches all fields.
#[tauri::command]
pub fn search_command(
    target: String,
    by: Vec<String>,
    page: Option<i64>,
    limit: Option<i64>,
    sort_by: Option<String>,
    sort_order: Option<String>,
) -> Result<ListResponse<Book>, String> {
    let params = ListParams {
        search: Some(target),
        search_fields: if by.is_empty() { None } else { Some(by) },
        page,
        limit,
        sort_by,
        sort_order,
        ..Default::default()
    };
    params.validate()?;
    books_service::list_books(&params)
}
