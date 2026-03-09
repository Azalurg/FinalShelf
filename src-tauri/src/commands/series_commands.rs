use crate::{
    models::{
        series::{NewSeries, Series, SeriesListItem, SeriesWithBooks},
        query::{ListParams, ListResponse},
    },
    services::series_service,
};

#[tauri::command]
pub async fn get_series_list_command(params: ListParams) -> Result<ListResponse<SeriesListItem>, String> {
    params.validate()?;
    series_service::list_series(&params)
}

#[tauri::command]
pub async fn get_series_command(id: i32) -> Result<SeriesWithBooks, String> {
    series_service::get_series(id)
}

#[tauri::command]
pub async fn create_series_command(name: String, author_name: String, description: Option<String>) -> Result<Series, String> {
    let new_series = NewSeries {
        name,
        author_name,
        description,
    };
    series_service::create_series(new_series)
}

#[tauri::command]
pub async fn update_series_command(id: i32, name: Option<String>, description: Option<String>) -> Result<Series, String> {
    series_service::update_series(id, name, description)
}

#[tauri::command]
pub async fn delete_series_command(id: i32) -> Result<(), String> {
    series_service::delete_series(id)
}

#[tauri::command]
pub async fn assign_book_to_series_command(book_title: String, series_id: Option<i32>, series_order: Option<i32>) -> Result<(), String> {
    series_service::assign_book_to_series(&book_title, series_id, series_order)
}

#[tauri::command]
pub async fn get_series_by_author_command(author_name: String) -> Result<Vec<Series>, String> {
    series_service::get_series_by_author(&author_name)
}
