use crate::{
    models::{
        dashboard::Dashboard,
        query::ListParams,
    },
    services::{authors_service, books_service, genres_service, lectors_service},
};

#[tauri::command]
pub async fn get_dashboard_data_command() -> Result<Dashboard, String> {
    let all_books = books_service::list_books(&ListParams {
        limit: Some(1),
        ..Default::default()
    })?;

    let read_books = books_service::list_books(&ListParams {
        read_status: Some(true),
        limit: Some(1),
        ..Default::default()
    })?;

    let new_books = books_service::list_books(&ListParams {
        sort_by: Some("create_date".to_string()),
        sort_order: Some("desc".to_string()),
        limit: Some(8),
        ..Default::default()
    })?;

    let top_books = books_service::list_books(&ListParams {
        sort_by: Some("score".to_string()),
        sort_order: Some("desc".to_string()),
        limit: Some(5),
        ..Default::default()
    })?;

    let top_authors = authors_service::list_authors(&ListParams {
        sort_by: Some("books_count".to_string()),
        sort_order: Some("desc".to_string()),
        limit: Some(5),
        ..Default::default()
    })?;

    let genres = genres_service::list_genres(&ListParams {
        limit: Some(1),
        ..Default::default()
    })?;

    let lectors = lectors_service::list_lectors(&ListParams {
        limit: Some(1),
        ..Default::default()
    })?;

    Ok(Dashboard {
        books_count: all_books.total_count,
        read_books_count: read_books.total_count,
        authors_count: top_authors.total_count,
        genres_count: genres.total_count,
        lectors_count: lectors.total_count,
        new_books: new_books.items,
        top_authors: top_authors.items,
        top_books: top_books.items,
    })
}
