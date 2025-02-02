use crate::{
    models::lector::{Lector, LectorWithBooks},
    services::lectors_service::{get_lector, get_lectors_list},
};

#[tauri::command]
pub fn get_lectors_list_command() -> Vec<Lector> {
    get_lectors_list()
}

#[tauri::command]
pub fn get_lector_command(lector_name: String) -> Option<LectorWithBooks> {
    get_lector(lector_name)
}
