use crate::models::book::Book;
use crate::services::search_service::search;

#[tauri::command]
pub fn search_command(target: String, by: Vec<String>) -> Result<Vec<Book>, String> {
    Ok(search(target, by))
}
