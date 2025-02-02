use crate::{
    models::genre::{Genre, GenreWithBooks},
    services::genres_service::{get_genre, get_genres_list},
};

#[tauri::command]
pub fn get_genres_list_command() -> Vec<Genre> {
    get_genres_list()
}

#[tauri::command]
pub fn get_genre_command(genre_name: String) -> Option<GenreWithBooks> {
    get_genre(genre_name)
}
