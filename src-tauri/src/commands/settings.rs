// Kill, Change background, Add path, change path, remove path

use crate::{
    models::path::AbsolutePath,
    scanner::quick_scan,
    services::absolute_paths_service::{add_absolute_path, get_all_absolute_path, set_current_absolute_path_by_id},
};

#[tauri::command]
pub fn ping_command() -> String {
    "ping".to_string()
}

#[tauri::command]
pub fn quick_scan_command() -> Result<(), String> {
    quick_scan()
}

#[tauri::command]
pub fn kill_command() -> Result<(), String> {
    std::process::exit(0);
}

#[tauri::command]
pub fn add_absolute_path_command(absolute_path: String) -> Result<(), String> {
    add_absolute_path(absolute_path)
}

#[tauri::command]
pub fn get_all_absolute_path_command() -> Vec<AbsolutePath> {
    get_all_absolute_path()
}

#[tauri::command]
pub fn set_current_absolute_path_by_id_command(absolute_path_id: i32) -> Result<(), String> {
    print!("Setting current path by id: {}", absolute_path_id);
    let _ = set_current_absolute_path_by_id(absolute_path_id);
    Ok(())
}
