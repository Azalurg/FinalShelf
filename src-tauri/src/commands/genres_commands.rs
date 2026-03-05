use crate::{
    models::{
        genre::{Genre, GenreWithBooks},
        query::{ListParams, ListResponse},
    },
    services::genres_service,
};

#[tauri::command]
pub async fn get_genres_list_command(params: ListParams) -> Result<ListResponse<Genre>, String> {
    params.validate()?;
    genres_service::list_genres(&params)
}

#[tauri::command]
pub async fn get_genre_command(genre_name: String) -> Result<GenreWithBooks, String> {
    genres_service::get_genre(&genre_name)
}
