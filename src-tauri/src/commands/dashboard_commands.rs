use crate::{
    commands::books_commands,
    models::dashboard::{self, Dashboard},
    services::{authors_service, books_service, genres_service, lectors_service},
};

#[tauri::command]
pub async fn get_dashboard_data_command() -> Result<Dashboard, String> {
    let dashboard_data = Dashboard {
        books_count: books_service::get_books_count(),
        authors_count: authors_service::get_authors_count(),
        genres_count: genres_service::get_genres_count(),
        lectors_count: lectors_service::get_lectors_count(),
        read_books_count: books_service::get_read_books_count(),
        new_books: books_service::get_books_by_date(8),
        top_authors: authors_service::get_top_authors(5),
        top_books: books_service::get_books_by_score(5),
    };
    Ok(dashboard_data)
}
